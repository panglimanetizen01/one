use std::io::{self, Read};
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::fd::AsRawFd;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::capability::{Capability, CapabilitySet};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_OUTPUT_BYTES_PER_STREAM: usize = 1024 * 1024;
#[cfg(any(target_os = "linux", target_os = "android", windows))]
const MAX_OUTPUT_BYTES_TOTAL: usize = 2 * MAX_OUTPUT_BYTES_PER_STREAM;

#[cfg(any(target_os = "linux", target_os = "android", windows))]
fn validate_total_output_limit(stdout_bytes: usize, stderr_bytes: usize) -> Result<(), String> {
    let total = stdout_bytes.saturating_add(stderr_bytes);
    if total > MAX_OUTPUT_BYTES_TOTAL {
        return Err(format!(
            "Subprocess exceeded the {} byte aggregate output limit.",
            MAX_OUTPUT_BYTES_TOTAL
        ));
    }
    Ok(())
}
#[cfg(any(target_os = "linux", target_os = "android"))]
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(10);
#[cfg(any(target_os = "linux", target_os = "android"))]
const READER_SHUTDOWN_GRACE: Duration = Duration::from_millis(100);
#[cfg(any(target_os = "linux", target_os = "android"))]
const READER_IDLE_GRACE: Duration = Duration::from_millis(20);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSpec {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub status_code: Option<i32>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn spawn_bounded_reader<R: Read + Send + 'static>(
    mut reader: R,
    output_limit_exceeded: Arc<AtomicBool>,
    shutdown_requested: Arc<AtomicBool>,
    shutdown_incomplete: Arc<AtomicBool>,
) -> JoinHandle<io::Result<Vec<u8>>> {
    thread::spawn(move || {
        let mut captured = Vec::with_capacity(MAX_OUTPUT_BYTES_PER_STREAM.min(16 * 1024));
        let mut buffer = [0_u8; 8192];
        let mut shutdown_deadline = None;
        let mut shutdown_idle_since = None;
        loop {
            let shutdown = shutdown_requested.load(Ordering::Acquire);
            if shutdown && shutdown_deadline.is_none() {
                shutdown_deadline = Some(Instant::now() + READER_SHUTDOWN_GRACE);
            }
            if shutdown_deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                // A writer that never lets the nonblocking pipe become idle must
                // not keep this thread alive forever or be silently accepted.
                shutdown_incomplete.store(true, Ordering::Release);
                break;
            }

            let read = match reader.read(&mut buffer) {
                Ok(read) => read,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    if shutdown {
                        let idle_since = shutdown_idle_since.get_or_insert_with(Instant::now);
                        if idle_since.elapsed() >= READER_IDLE_GRACE {
                            break;
                        }
                    }
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Err(error) => return Err(error),
            };
            if read == 0 {
                break;
            }
            shutdown_idle_since = None;
            let remaining = MAX_OUTPUT_BYTES_PER_STREAM.saturating_sub(captured.len());
            let retained = read.min(remaining);
            captured.extend_from_slice(&buffer[..retained]);
            if retained < read {
                output_limit_exceeded.store(true, Ordering::Release);
                break;
            }
            // After shutdown, keep draining only until the pipe becomes idle,
            // the output cap is exceeded, or the bounded grace period expires.
            // This preserves overflow detection without waiting indefinitely
            // for an escaped descendant to close its inherited writer.
        }
        Ok(captured)
    })
}

#[cfg(windows)]
fn spawn_bounded_reader<R: Read + Send + 'static>(
    mut reader: R,
    output_limit_exceeded: Arc<AtomicBool>,
    _shutdown_requested: Arc<AtomicBool>,
) -> JoinHandle<io::Result<Vec<u8>>> {
    // Windows reader threads remain blocking: closing the invocation's Job
    // Object terminates its descendants and closes inherited pipe writers.
    // The shutdown flag is intentionally not used to interrupt a blocking
    // Read; Job Object teardown remains the Windows reader termination path.
    thread::spawn(move || {
        let mut captured = Vec::with_capacity(MAX_OUTPUT_BYTES_PER_STREAM.min(16 * 1024));
        let mut buffer = [0_u8; 8192];
        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            let remaining = MAX_OUTPUT_BYTES_PER_STREAM.saturating_sub(captured.len());
            let retained = read.min(remaining);
            captured.extend_from_slice(&buffer[..retained]);
            if retained < read {
                output_limit_exceeded.store(true, Ordering::Release);
            }
        }
        Ok(captured)
    })
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn set_reader_nonblocking<R: AsRawFd>(reader: &R) -> io::Result<()> {
    let descriptor = reader.as_raw_fd();
    // SAFETY: fcntl only reads and updates flags for this live pipe descriptor.
    let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFL) };
    if flags < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: the descriptor remains owned by the reader and the flags are valid.
    if unsafe { libc::fcntl(descriptor, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn install_linux_process_group_filter() -> io::Result<()> {
    // Keep every descendant in the invocation's original process group.
    // The filter is inherited across fork/exec and cannot be removed after
    // PR_SET_NO_NEW_PRIVS. Namespace creation is denied as another escape path.
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct SockFilter { code: u16, jt: u8, jf: u8, k: u32 }
    #[repr(C)]
    struct SockFprog { len: u16, filter: *mut SockFilter }

    const BPF_LD_W_ABS: u16 = 0x20;
    const BPF_JMP_JEQ_K: u16 = 0x15;
    const BPF_ALU_AND_K: u16 = 0x54;
    const BPF_RET_K: u16 = 0x06;
    const SECCOMP_RET_KILL_PROCESS: u32 = 0x8000_0000;
    const SECCOMP_RET_ERRNO: u32 = 0x0005_0000;
    const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
    const SECCOMP_MODE_FILTER: libc::c_int = 2;
    const PR_SET_SECCOMP: libc::c_int = 22;
    const PR_SET_NO_NEW_PRIVS: libc::c_int = 38;
    const EPERM_ACTION: u32 = SECCOMP_RET_ERRNO | libc::EPERM as u32;
    const CLONE_NAMESPACE_FLAGS: u32 = 0x0000_0080 | 0x0002_0000 | 0x0200_0000 |
        0x0400_0000 | 0x0800_0000 | 0x1000_0000 | 0x2000_0000 | 0x4000_0000;

    let audit_arch = if cfg!(target_arch = "x86_64") { 0xc000_003e }
        else if cfg!(target_arch = "aarch64") { 0xc000_00b7 }
        else if cfg!(target_arch = "x86") { 0x4000_0003 }
        else if cfg!(target_arch = "arm") { 0x4000_0028 }
        else { return Err(io::Error::new(io::ErrorKind::Unsupported, "Linux process containment is unavailable for this CPU architecture")); };

    // seccomp_data: arch at byte 4, syscall number at byte 0, args[0] at byte 16.
    let mut filter = [
        SockFilter { code: BPF_LD_W_ABS, jt: 0, jf: 0, k: 4 },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 1, jf: 0, k: audit_arch },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: SECCOMP_RET_KILL_PROCESS },
        SockFilter { code: BPF_LD_W_ABS, jt: 0, jf: 0, k: 0 },
        SockFilter { code: BPF_ALU_AND_K, jt: 0, jf: 0, k: if cfg!(target_arch = "x86_64") { 0x4000_0000 } else { 0 } },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 1, jf: 0, k: 0 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: SECCOMP_RET_KILL_PROCESS },
        SockFilter { code: BPF_LD_W_ABS, jt: 0, jf: 0, k: 0 },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 0, jf: 1, k: libc::SYS_setpgid as u32 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: EPERM_ACTION },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 0, jf: 1, k: libc::SYS_setsid as u32 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: EPERM_ACTION },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 0, jf: 1, k: libc::SYS_unshare as u32 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: EPERM_ACTION },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 0, jf: 1, k: libc::SYS_setns as u32 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: EPERM_ACTION },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 0, jf: 4, k: libc::SYS_clone as u32 },
        SockFilter { code: BPF_LD_W_ABS, jt: 0, jf: 0, k: 16 },
        SockFilter { code: BPF_ALU_AND_K, jt: 0, jf: 0, k: CLONE_NAMESPACE_FLAGS },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 1, jf: 0, k: 0 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: EPERM_ACTION },
        SockFilter { code: BPF_LD_W_ABS, jt: 0, jf: 0, k: 0 },
        SockFilter { code: BPF_JMP_JEQ_K, jt: 0, jf: 1, k: libc::SYS_clone3 as u32 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: SECCOMP_RET_ERRNO | libc::ENOSYS as u32 },
        SockFilter { code: BPF_RET_K, jt: 0, jf: 0, k: SECCOMP_RET_ALLOW },
    ];
    let program = SockFprog { len: filter.len() as u16, filter: filter.as_mut_ptr() };
    // SAFETY: these prctl calls affect only the just-forked child before exec.
    if unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } != 0 {
        return Err(io::Error::last_os_error());
    }
    if unsafe { libc::prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &program as *const SockFprog as libc::c_ulong, 0, 0) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
fn join_reader(reader: JoinHandle<io::Result<Vec<u8>>>) -> Result<Vec<u8>, String> {
    reader
        .join()
        .map_err(|_| "Subprocess output reader panicked.".to_owned())?
        .map_err(|error| format!("Failed to read subprocess output: {error}"))
}

fn validate_process_spec(spec: &ProcessSpec) -> Result<(), String> {
    if spec.program.is_empty() {
        return Err("Process executable path must not be empty.".into());
    }
    if spec.program.contains('\0') {
        return Err("Process executable path must not contain an embedded NUL byte.".into());
    }
    if let Some((index, _)) = spec
        .args
        .iter()
        .enumerate()
        .find(|(_, arg)| arg.contains('\0'))
    {
        return Err(format!(
            "Process argument {index} must not contain an embedded NUL byte."
        ));
    }
    if spec
        .working_directory
        .as_deref()
        .is_some_and(|directory| directory.contains('\0'))
    {
        return Err("Process working directory must not contain an embedded NUL byte.".into());
    }
    Ok(())
}

pub fn run_process(
    spec: &ProcessSpec,
    capabilities: &CapabilitySet,
) -> Result<ProcessResult, String> {
    run_process_with_timeout(spec, capabilities, PROCESS_TIMEOUT)
}

// The timeout override is private and exists for integration probes whose
// host-side setup (for example, Windows PowerShell Add-Type) is slower than
// the production process budget. Public callers always use PROCESS_TIMEOUT.
fn run_process_with_timeout(
    spec: &ProcessSpec,
    capabilities: &CapabilitySet,
    timeout: Duration,
) -> Result<ProcessResult, String> {
    if !capabilities.permits(Capability::Process) {
        return Err("Kemampuan proses belum diberikan kepada program.".into());
    }
    validate_process_spec(spec)?;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        run_process_unix(spec, timeout)
    }
    #[cfg(windows)]
    {
        run_process_windows(spec, timeout)
    }
    #[cfg(target_os = "macos")]
    {
        let _ = (spec, timeout);
        Err("Process execution is unavailable on macOS until non-escapable descendant containment is implemented.".into())
    }
    #[cfg(not(any(target_os = "linux", windows, target_os = "android", target_os = "macos")))]
    {
        let _ = (spec, timeout);
        Err("Bounded process execution is unsupported on this platform.".into())
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn run_process_unix(spec: &ProcessSpec, timeout: Duration) -> Result<ProcessResult, String> {
    use std::os::unix::process::CommandExt;

    let mut command = Command::new(&spec.program);
    command
        .args(&spec.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);

    #[cfg(any(target_os = "linux", target_os = "android"))]
    // SAFETY: the pre-exec hook performs only child-local prctl syscalls and
    // installs the inherited process-group containment filter before exec.
    unsafe {
        command.pre_exec(install_linux_process_group_filter);
    }
    if let Some(directory) = &spec.working_directory {
        command.current_dir(directory);
    }

    let mut child = command
        .spawn()
        .map_err(|error| format!("Gagal menjalankan proses '{}': {error}", spec.program))?;
    let process_group = i32::try_from(child.id()).map_err(|_| {
        "Subprocess identifier exceeds the supported process-group range.".to_owned()
    })?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Subprocess stdout pipe was not created.".to_owned())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Subprocess stderr pipe was not created.".to_owned())?;

    if let Err(error) =
        set_reader_nonblocking(&stdout).and_then(|()| set_reader_nonblocking(&stderr))
    {
        kill_process_group(process_group);
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!(
            "Failed to configure bounded subprocess pipe readers: {error}"
        ));
    }

    let output_limit_exceeded = Arc::new(AtomicBool::new(false));
    let reader_shutdown = Arc::new(AtomicBool::new(false));
    let reader_shutdown_incomplete = Arc::new(AtomicBool::new(false));
    let stdout_reader = spawn_bounded_reader(
        stdout,
        Arc::clone(&output_limit_exceeded),
        Arc::clone(&reader_shutdown),
        Arc::clone(&reader_shutdown_incomplete),
    );
    let stderr_reader = spawn_bounded_reader(
        stderr,
        Arc::clone(&output_limit_exceeded),
        Arc::clone(&reader_shutdown),
        Arc::clone(&reader_shutdown_incomplete),
    );
    let deadline = Instant::now() + timeout;
    let mut timed_out = false;

    let status = loop {
        if output_limit_exceeded.load(Ordering::Acquire) {
            kill_process_group(process_group);
            let _ = child.kill();
            break child.wait();
        }

        match child_exited_without_reaping(child.id()) {
            Ok(true) => {
                // Keep the group leader unreaped until after all descendants
                // have received SIGKILL; this prevents process-group ID reuse.
                kill_process_group(process_group);
                break child.wait();
            }
            Ok(false) => {}
            Err(error) => {
                kill_process_group(process_group);
                let _ = child.kill();
                let _ = child.wait();
                reader_shutdown.store(true, Ordering::Release);
                let _ = join_reader(stdout_reader);
                let _ = join_reader(stderr_reader);
                return Err(format!("Failed while waiting for subprocess: {error}"));
            }
        }

        if Instant::now() >= deadline {
            timed_out = true;
            kill_process_group(process_group);
            let _ = child.kill();
            break child.wait();
        }
        thread::sleep(PROCESS_POLL_INTERVAL);
    };

    // The process group was terminated before reaping the direct child on
    // every exit path, so its numeric group ID could not be recycled.
    // Nonblocking readers retain a bounded shutdown path for unusual pipe
    // behavior even though Linux descendants cannot leave the invocation group.
    reader_shutdown.store(true, Ordering::Release);
    let stdout = join_reader(stdout_reader)?;
    let stderr = join_reader(stderr_reader)?;
    let status = status.map_err(|error| format!("Failed to reap subprocess: {error}"))?;

    if timed_out {
        return Err(format!(
            "Subprocess exceeded the {} second execution deadline.",
            timeout.as_secs()
        ));
    }
    if output_limit_exceeded.load(Ordering::Acquire) {
        return Err(format!(
            "Subprocess exceeded the {} byte per-stream output limit.",
            MAX_OUTPUT_BYTES_PER_STREAM
        ));
    }
    if reader_shutdown_incomplete.load(Ordering::Acquire) {
        return Err(format!(
            "Subprocess output reader did not become idle within the {:?} shutdown grace period.",
            READER_SHUTDOWN_GRACE
        ));
    }
    validate_total_output_limit(stdout.len(), stderr.len())?;

    Ok(ProcessResult {
        status_code: status.code(),
        success: status.success(),
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
    })
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn child_exited_without_reaping(pid: u32) -> io::Result<bool> {
    loop {
        // WNOWAIT observes the direct child as a zombie without releasing its
        // PID. The process-group ID therefore cannot be recycled before group
        // cleanup has run.
        let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
        let result = unsafe {
            libc::waitid(
                libc::P_PID,
                pid as libc::id_t,
                &mut info,
                libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
            )
        };
        if result == 0 {
            return Ok(unsafe { info.si_pid() } != 0);
        }
        let error = io::Error::last_os_error();
        if error.kind() == io::ErrorKind::Interrupted {
            continue;
        }
        return Err(error);
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn kill_process_group(process_group: i32) {
    // SAFETY: a negative pid addresses only the process group created for this
    // invocation via CommandExt::process_group(0).
    unsafe {
        libc::kill(-process_group, libc::SIGKILL);
    }
}

#[cfg(windows)]
fn run_process_windows(spec: &ProcessSpec, timeout: Duration) -> Result<ProcessResult, String> {
    use std::ffi::{OsStr, OsString};
    use std::fs::File;
    use std::mem::{size_of, size_of_val};
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::io::FromRawHandle;
    use std::path::Path;
    use std::ptr::{null, null_mut};

    #[cfg(test)]
    use windows_sys::Win32::Foundation::DuplicateHandle;
    use windows_sys::Win32::Foundation::{
        CloseHandle, GENERIC_READ, GetLastError, HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation,
        WAIT_OBJECT_0, WAIT_TIMEOUT,
    };
    use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::JobObjects::{
        CreateJobObjectW, IsProcessInJob, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JobObjectBasicAccountingInformation, JobObjectExtendedLimitInformation,
        QueryInformationJobObject, SetInformationJobObject, TerminateJobObject,
    };
    use windows_sys::Win32::System::Pipes::CreatePipe;
    #[cfg(test)]
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    use windows_sys::Win32::System::Threading::{
        CREATE_NO_WINDOW, CreateProcessW, DeleteProcThreadAttributeList,
        EXTENDED_STARTUPINFO_PRESENT, GetExitCodeProcess, INFINITE,
        InitializeProcThreadAttributeList, PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
        PROC_THREAD_ATTRIBUTE_JOB_LIST, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOEXW,
        STARTUPINFOW, TerminateProcess, UpdateProcThreadAttribute, WaitForSingleObject,
    };

    struct Handle(HANDLE);
    impl Handle {
        fn new(handle: HANDLE) -> Result<Self, String> {
            if handle.is_null() || handle as isize == -1 {
                Err(format!(
                    "Windows handle creation failed (error {}).",
                    unsafe { GetLastError() }
                ))
            } else {
                Ok(Self(handle))
            }
        }

        fn raw(&self) -> HANDLE {
            self.0
        }

        fn into_raw(mut self) -> HANDLE {
            let handle = self.0;
            self.0 = null_mut();
            handle
        }
    }
    impl Drop for Handle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                // SAFETY: this wrapper uniquely owns the handle.
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }

    struct AttributeList {
        raw: *mut core::ffi::c_void,
        _storage: Vec<usize>,
    }
    impl Drop for AttributeList {
        fn drop(&mut self) {
            // SAFETY: the list was initialized successfully and remains backed
            // by _storage for its full lifetime.
            unsafe {
                DeleteProcThreadAttributeList(self.raw);
            }
        }
    }

    fn wide_null(value: &OsStr) -> Vec<u16> {
        value.encode_wide().chain(std::iter::once(0)).collect()
    }

    fn resolve_executable(program: &OsStr) -> Result<OsString, String> {
        let path = Path::new(program);
        if path.is_absolute() || path.components().count() > 1 {
            return if path.is_file() {
                Ok(path.as_os_str().to_os_string())
            } else {
                Err(format!("Executable '{}' was not found.", path.display()))
            };
        }

        let has_extension = path.extension().is_some();
        let extensions: &[&str] = if has_extension {
            &[""]
        } else {
            &[".exe", ".com"]
        };
        let search_path = std::env::var_os("PATH").unwrap_or_default();
        for directory in std::env::split_paths(&search_path) {
            for extension in extensions {
                let mut candidate = directory.join(program);
                if !extension.is_empty() {
                    let mut name = candidate.into_os_string();
                    name.push(extension);
                    candidate = Path::new(&name).to_path_buf();
                }
                if candidate.is_file() {
                    return Ok(candidate.into_os_string());
                }
            }
        }
        Err(format!(
            "Executable '{}' was not found in PATH.",
            path.display()
        ))
    }

    fn append_windows_argument(output: &mut Vec<u16>, argument: &OsStr) {
        let wide: Vec<u16> = argument.encode_wide().collect();
        let needs_quotes = wide.is_empty()
            || wide
                .iter()
                .any(|character| *character == 0x20 || *character == 0x09 || *character == 0x22);
        if !needs_quotes {
            output.extend(wide);
            return;
        }

        output.push(0x22);
        let mut backslashes = 0usize;
        for character in wide {
            if character == 0x5c {
                backslashes += 1;
            } else if character == 0x22 {
                for _ in 0..(backslashes * 2 + 1) {
                    output.push(0x5c);
                }
                output.push(character);
                backslashes = 0;
            } else {
                for _ in 0..backslashes {
                    output.push(0x5c);
                }
                output.push(character);
                backslashes = 0;
            }
        }
        for _ in 0..(backslashes * 2) {
            output.push(0x5c);
        }
        output.push(0x22);
    }

    fn create_pipe_pair(security: &SECURITY_ATTRIBUTES) -> Result<(Handle, Handle), String> {
        let mut read = null_mut();
        let mut write = null_mut();
        // SAFETY: output pointers are valid and security describes inheritable
        // pipe handles for the child process.
        if unsafe { CreatePipe(&mut read, &mut write, security, 0) } == 0 {
            return Err(format!("CreatePipe failed (error {}).", unsafe {
                GetLastError()
            }));
        }
        let read = Handle::new(read)?;
        let write = Handle::new(write)?;
        // SAFETY: only the parent-side read handle must not be inherited.
        if unsafe { SetHandleInformation(read.raw(), HANDLE_FLAG_INHERIT, 0) } == 0 {
            return Err(format!("SetHandleInformation failed (error {}).", unsafe {
                GetLastError()
            }));
        }
        Ok((read, write))
    }

    let executable = resolve_executable(OsStr::new(&spec.program))?;
    let application_name = wide_null(&executable);

    let security = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: null_mut(),
        bInheritHandle: 1,
    };
    let (stdout_read, stdout_write) = create_pipe_pair(&security)?;
    let (stderr_read, stderr_write) = create_pipe_pair(&security)?;

    let nul_name: Vec<u16> = "NUL".encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: NUL is a built-in Windows device; the inheritable handle is
    // explicitly listed in PROC_THREAD_ATTRIBUTE_HANDLE_LIST.
    let stdin = Handle::new(unsafe {
        CreateFileW(
            nul_name.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            &security,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    })?;

    // The job is assigned as part of CreateProcessW via PROC_THREAD_ATTRIBUTE_JOB_LIST.
    // This avoids the spawn-then-AssignProcessToJobObject race in which a child
    // could create an uncontained descendant before assignment.
    let job = Handle::new(unsafe { CreateJobObjectW(null(), null()) })?;
    let mut job_limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    job_limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    // SAFETY: job_limits has the exact structure and size required by this class.
    if unsafe {
        SetInformationJobObject(
            job.raw(),
            JobObjectExtendedLimitInformation,
            &job_limits as *const _ as *const core::ffi::c_void,
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )
    } == 0
    {
        return Err(format!(
            "SetInformationJobObject failed (error {}).",
            unsafe { GetLastError() }
        ));
    }

    // The test-only query handle lets the contained parent ask whether its
    // descendant belongs to this exact Job Object, not merely any outer job.
    // It is narrowed to JOB_OBJECT_QUERY and its inherit flag is cleared by
    // the PowerShell parent before that parent creates the ping descendant.
    #[cfg(test)]
    let job_query_handle = if spec
        .args
        .iter()
        .any(|arg| arg.contains("__ONE_JOB_QUERY_HANDLE__"))
    {
        let mut duplicated = null_mut();
        // SAFETY: source and target are this process; the new handle is query-only
        // and inheritable so it can be passed through the explicit handle list.
        if unsafe {
            DuplicateHandle(
                GetCurrentProcess(),
                job.raw(),
                GetCurrentProcess(),
                &mut duplicated,
                0x0004, // JOB_OBJECT_QUERY: query-only access to a Job Object.
                1,
                0,
            )
        } == 0
        {
            return Err(format!(
                "DuplicateHandle(JOB_OBJECT_QUERY) failed (error {}).",
                unsafe { GetLastError() }
            ));
        }
        Some(Handle::new(duplicated)?)
    } else {
        None
    };
    #[cfg(not(test))]
    let job_query_handle: Option<Handle> = None;

    let mut command_line = Vec::new();
    append_windows_argument(&mut command_line, &executable);
    for argument in &spec.args {
        command_line.push(0x20);
        let argument = if let Some(query_handle) = job_query_handle.as_ref() {
            OsString::from(argument.replace(
                "__ONE_JOB_QUERY_HANDLE__",
                &(query_handle.raw() as usize).to_string(),
            ))
        } else {
            OsString::from(argument)
        };
        append_windows_argument(&mut command_line, argument.as_os_str());
    }
    command_line.push(0);

    let mut attribute_size = 0usize;
    // SAFETY: the documented sizing call uses a null list and returns required bytes.
    unsafe {
        InitializeProcThreadAttributeList(null_mut(), 2, 0, &mut attribute_size);
    }
    if attribute_size == 0 {
        return Err(format!(
            "InitializeProcThreadAttributeList sizing failed (error {}).",
            unsafe { GetLastError() }
        ));
    }
    let mut attribute_storage =
        vec![0usize; (attribute_size + size_of::<usize>() - 1) / size_of::<usize>()];
    let attribute_pointer = attribute_storage.as_mut_ptr() as *mut core::ffi::c_void;
    // SAFETY: the buffer is sufficiently sized and pointer-aligned.
    if unsafe { InitializeProcThreadAttributeList(attribute_pointer, 2, 0, &mut attribute_size) }
        == 0
    {
        return Err(format!(
            "InitializeProcThreadAttributeList failed (error {}).",
            unsafe { GetLastError() }
        ));
    }
    let attributes = AttributeList {
        raw: attribute_pointer,
        _storage: attribute_storage,
    };

    let job_list = [job.raw()];
    // SAFETY: job_list remains alive through CreateProcessW; the OS copies the
    // handle list during process creation and assigns the job before execution.
    if unsafe {
        UpdateProcThreadAttribute(
            attributes.raw,
            0,
            PROC_THREAD_ATTRIBUTE_JOB_LIST as usize,
            job_list.as_ptr().cast(),
            size_of_val(&job_list),
            null_mut(),
            null(),
        )
    } == 0
    {
        return Err(format!(
            "UpdateProcThreadAttribute(JOB_LIST) failed (error {}).",
            unsafe { GetLastError() }
        ));
    }

    let mut inherited_handles = vec![stdin.raw(), stdout_write.raw(), stderr_write.raw()];
    if let Some(query_handle) = job_query_handle.as_ref() {
        inherited_handles.push(query_handle.raw());
    }
    // SAFETY: every handle in this list is inheritable and remains alive until
    // CreateProcessW returns; parent-side read handles are explicitly non-inheritable.
    if unsafe {
        UpdateProcThreadAttribute(
            attributes.raw,
            0,
            PROC_THREAD_ATTRIBUTE_HANDLE_LIST as usize,
            inherited_handles.as_ptr().cast(),
            (inherited_handles.len() * size_of::<HANDLE>()) as usize,
            null_mut(),
            null(),
        )
    } == 0
    {
        return Err(format!(
            "UpdateProcThreadAttribute(HANDLE_LIST) failed (error {}).",
            unsafe { GetLastError() }
        ));
    }

    let mut startup = STARTUPINFOEXW::default();
    startup.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
    startup.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
    startup.StartupInfo.hStdInput = stdin.raw();
    startup.StartupInfo.hStdOutput = stdout_write.raw();
    startup.StartupInfo.hStdError = stderr_write.raw();
    startup.lpAttributeList = attributes.raw;

    let working_directory = spec
        .working_directory
        .as_deref()
        .map(|directory| wide_null(OsStr::new(directory)));
    let working_directory_pointer = working_directory
        .as_ref()
        .map_or(null(), |directory| directory.as_ptr());

    let mut process_information = PROCESS_INFORMATION::default();
    // SAFETY: all pointers reference valid, NUL-terminated buffers and initialized
    // structures; STARTUPINFOEXW is passed through its STARTUPINFOW prefix as required.
    if unsafe {
        CreateProcessW(
            application_name.as_ptr(),
            command_line.as_mut_ptr(),
            null(),
            null(),
            1,
            EXTENDED_STARTUPINFO_PRESENT | CREATE_NO_WINDOW,
            null(),
            working_directory_pointer,
            &startup.StartupInfo as *const STARTUPINFOW,
            &mut process_information,
        )
    } == 0
    {
        return Err(format!("CreateProcessW failed (error {}).", unsafe {
            GetLastError()
        }));
    }

    // The child now owns its inherited query-only handle; close our duplicate
    // so only the PowerShell parent can hold it while running the probe.
    drop(job_query_handle);

    let process = Handle::new(process_information.hProcess)?;
    let thread_handle = Handle::new(process_information.hThread)?;

    // Verify that PROC_THREAD_ATTRIBUTE_JOB_LIST actually placed the direct
    // child in this invocation's job. A successful CreateProcessW call alone
    // is not sufficient evidence of the intended containment relationship.
    let mut process_in_job = 0i32;
    // SAFETY: both handles are live; process has query rights and this job was
    // created by this function with query access.
    if unsafe { IsProcessInJob(process.raw(), job.raw(), &mut process_in_job) } == 0 {
        let error = unsafe { GetLastError() };
        // Fail closed even if CreateProcessW returned a process that could not
        // be proven to belong to this invocation's Job Object.
        unsafe {
            TerminateProcess(process.raw(), 1);
            WaitForSingleObject(process.raw(), INFINITE);
        }
        return Err(format!("IsProcessInJob failed (error {error})."));
    }
    if process_in_job == 0 {
        // A process outside the job would not be terminated by closing the job.
        // Explicitly terminate and reap it before returning the containment error.
        unsafe {
            TerminateProcess(process.raw(), 1);
            WaitForSingleObject(process.raw(), INFINITE);
        }
        return Err("Windows subprocess was created outside its assigned Job Object.".into());
    }
    drop(thread_handle);
    drop(attributes);
    drop(stdin);
    drop(stdout_write);
    drop(stderr_write);

    // SAFETY: ownership of the read handles is transferred to File, which closes them.
    let stdout_file = unsafe { File::from_raw_handle(stdout_read.into_raw() as _) };
    let stderr_file = unsafe { File::from_raw_handle(stderr_read.into_raw() as _) };
    let output_limit_exceeded = Arc::new(AtomicBool::new(false));
    let reader_shutdown = Arc::new(AtomicBool::new(false));
    let stdout_reader = spawn_bounded_reader(
        stdout_file,
        Arc::clone(&output_limit_exceeded),
        Arc::clone(&reader_shutdown),
    );
    let stderr_reader = spawn_bounded_reader(
        stderr_file,
        Arc::clone(&output_limit_exceeded),
        Arc::clone(&reader_shutdown),
    );

    let deadline = Instant::now() + timeout;
    let mut timed_out = false;
    let mut wait_failed = false;
    loop {
        if output_limit_exceeded.load(Ordering::Acquire) {
            break;
        }
        if Instant::now() >= deadline {
            timed_out = true;
            break;
        }
        // SAFETY: process is a valid process handle and the timeout is bounded.
        match unsafe { WaitForSingleObject(process.raw(), 25) } {
            WAIT_OBJECT_0 => break,
            WAIT_TIMEOUT => {}
            _ => {
                wait_failed = true;
                break;
            }
        }
    }

    // A successful direct-child exit does not imply that its descendants have
    // exited. Explicitly terminate any remaining job members, then wait for the
    // job accounting to reach zero before returning. KILL_ON_JOB_CLOSE remains
    // a fail-safe for every early-return path.
    let mut accounting: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION = unsafe { core::mem::zeroed() };
    let query_ok = unsafe {
        QueryInformationJobObject(
            job.raw(),
            JobObjectBasicAccountingInformation,
            &mut accounting as *mut _ as *mut core::ffi::c_void,
            size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>() as u32,
            null_mut(),
        )
    };
    if query_ok == 0 {
        wait_failed = true;
        // SAFETY: the job handle has termination rights and owns this invocation.
        if unsafe { TerminateJobObject(job.raw(), 1) } == 0 {
            wait_failed = true;
        }
    }

    // Do not make termination conditional on the accounting snapshot. The
    // snapshot is diagnostic, while TerminateJobObject is the authoritative
    // operation for every process still associated with this invocation job.
    // Calling it unconditionally also closes the race where a descendant is
    // still alive but the accounting query reports no active members.
    let terminate_error = if unsafe { TerminateJobObject(job.raw(), 1) } == 0 {
        wait_failed = true;
        Some(unsafe { GetLastError() })
    } else {
        None
    };

    // Ensure the direct process is reaped even after timeout/output-limit/error,
    // but keep cleanup bounded if the termination request itself fails.
    if unsafe { WaitForSingleObject(process.raw(), 5_000) } != WAIT_OBJECT_0 {
        wait_failed = true;
    }

    let cleanup_deadline = Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let mut accounting: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION = unsafe { core::mem::zeroed() };
        if unsafe {
            QueryInformationJobObject(
                job.raw(),
                JobObjectBasicAccountingInformation,
                &mut accounting as *mut _ as *mut core::ffi::c_void,
                size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>() as u32,
                null_mut(),
            )
        } == 0
        {
            wait_failed = true;
            break;
        }
        if accounting.ActiveProcesses == 0 {
            break;
        }
        if Instant::now() >= cleanup_deadline {
            wait_failed = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // KILL_ON_JOB_CLOSE remains the final safety net after explicit cleanup.
    drop(job);
    reader_shutdown.store(true, Ordering::Release);
    let mut exit_code = 1u32;
    // SAFETY: the process has signaled or was terminated and remains open.
    if unsafe { GetExitCodeProcess(process.raw(), &mut exit_code) } == 0 {
        wait_failed = true;
    }
    drop(process);

    let stdout = join_reader(stdout_reader)?;
    let stderr = join_reader(stderr_reader)?;
    if wait_failed {
        return Err(match terminate_error {
            Some(error) => {
                format!("Windows subprocess cleanup failed (TerminateJobObject error {error}).")
            }
            None => "Waiting for the Windows subprocess failed.".into(),
        });
    }
    if timed_out {
        return Err(format!(
            "Subprocess exceeded the {} second execution deadline.",
            timeout.as_secs()
        ));
    }
    if output_limit_exceeded.load(Ordering::Acquire) {
        return Err(format!(
            "Subprocess exceeded the {} byte per-stream output limit.",
            MAX_OUTPUT_BYTES_PER_STREAM
        ));
    }
    validate_total_output_limit(stdout.len(), stderr.len())?;

    Ok(ProcessResult {
        status_code: Some(exit_code as i32),
        success: exit_code == 0,
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
#[cfg(any(target_os = "linux", target_os = "android"))]
    fn test_escape_fixture(label: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        static NEXT_FIXTURE: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);
        let nonce = NEXT_FIXTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "one-process-escape-{}-{label}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("unique process fixture directory");
        let pid_file = directory.join("escaped.pid");
        (directory, pid_file)
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn cleanup_test_escaped_group(pid_file: &std::path::Path) -> Option<i32> {
        let pid = std::fs::read_to_string(pid_file).ok()?.trim().parse::<i32>().ok()?;
        // SAFETY: this helper only targets the process group created by the
        // adversarial test's setsid child, never a caller-provided PID.
        unsafe {
            libc::kill(-pid, libc::SIGKILL);
            libc::kill(pid, libc::SIGKILL);
        }
        Some(pid)
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn spawn_escape_probe(mode: u8, use_setpgid: bool) {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let path = std::env::current_dir()
            .expect("process fixture working directory")
            .join("escaped.pid");
        let path = CString::new(path.as_os_str().as_bytes()).expect("temporary path has no NUL");
        // SAFETY: the child uses only async-signal-safe libc calls after fork.
        let child = unsafe { libc::fork() };
        assert!(child >= 0, "fork failed: {}", std::io::Error::last_os_error());
        if child == 0 {
            let escaped = if use_setpgid {
                unsafe { libc::setpgid(0, 0) } == 0
            } else {
                unsafe { libc::setsid() } >= 0
            };
            if escaped {
                let mut pid = unsafe { libc::getpid() } as u32;
                let mut digits = [0u8; 16];
                let mut cursor = digits.len();
                loop {
                    cursor -= 1;
                    digits[cursor] = b'0' + (pid % 10) as u8;
                    pid /= 10;
                    if pid == 0 { break; }
                }
                let fd = unsafe { libc::open(path.as_ptr(), libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC, 0o600) };
                if fd >= 0 {
                    unsafe {
                        libc::write(fd, digits[cursor..].as_ptr().cast(), digits.len() - cursor);
                        libc::close(fd);
                    }
                }
            }
            if mode == 2 {
                let buffer = [b'x'; 8192];
                for _ in 0..129 {
                    let written = unsafe { libc::write(libc::STDOUT_FILENO, buffer.as_ptr().cast(), buffer.len()) };
                    if written <= 0 { break; }
                }
            }
            unsafe {
                libc::sleep(30);
                libc::_exit(0);
            }
        }
        if mode != 0 {
            unsafe { libc::sleep(30); }
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    #[ignore = "invoked only as a child helper by process containment regressions"]
    fn process_group_escape_helper_exit() {
        spawn_escape_probe(0, false);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    #[ignore = "invoked only as a child helper by process containment regressions"]
    fn process_group_escape_helper_timeout() {
        spawn_escape_probe(1, false);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    #[ignore = "invoked only as a child helper by process containment regressions"]
    fn process_group_escape_helper_output() {
        spawn_escape_probe(2, false);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    #[ignore = "invoked only as a child helper by process containment regressions"]
    fn process_group_setpgid_helper_exit() {
        spawn_escape_probe(0, true);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    #[ignore = "invoked only as a child helper by process containment regressions"]
    fn process_group_setpgid_helper_timeout() {
        spawn_escape_probe(1, true);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    #[ignore = "invoked only as a child helper by process containment regressions"]
    fn process_group_setpgid_helper_output() {
        spawn_escape_probe(2, true);
    }




    #[test]
    fn denied_process_capability_never_spawns() {
        let spec = ProcessSpec {
            program: "one-test-process-that-must-not-run".into(),
            args: vec![],
            working_directory: None,
        };
        let error = run_process(&spec, &CapabilitySet::none()).expect_err("process must be denied");
        assert_eq!(error, "Kemampuan proses belum diberikan kepada program.");
    }

    #[test]
    fn process_spec_rejects_empty_executable_path() {
        let error = run_process(
            &ProcessSpec {
                program: String::new(),
                args: vec![],
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("an empty executable path must be rejected before spawn");
        assert!(error.contains("must not be empty"));
    }

    #[test]
    fn process_spec_rejects_embedded_nul_in_executable_path() {
        let error = run_process(
            &ProcessSpec {
                program: "tool\0.exe".into(),
                args: vec![],
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("an executable path with NUL must be rejected before spawn");
        assert!(error.contains("embedded NUL"));
    }

    #[test]
    fn process_spec_rejects_embedded_nul_in_arguments() {
        let error = run_process(
            &ProcessSpec {
                program: "tool".into(),
                args: vec!["safe\0truncated".into()],
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("an argument with NUL must be rejected before spawn");
        assert!(error.contains("argument 0"));
        assert!(error.contains("embedded NUL"));
    }

    #[test]
    fn process_spec_rejects_embedded_nul_in_working_directory() {
        let error = run_process(
            &ProcessSpec {
                program: "tool".into(),
                args: vec![],
                working_directory: Some("safe\0truncated".into()),
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("a working directory with NUL must be rejected before spawn");
        assert!(error.contains("working directory"));
        assert!(error.contains("embedded NUL"));
    }

    #[cfg(any(target_os = "linux", target_os = "android", windows))]
    #[test]
    fn allowed_process_preserves_exit_status_and_streams() {
        let (program, args) = if cfg!(windows) {
            (
                "cmd".to_owned(),
                vec![
                    "/C".into(),
                    "echo stdout & echo stderr 1>&2 & exit /B 7".into(),
                ],
            )
        } else {
            (
                "sh".to_owned(),
                vec![
                    "-c".into(),
                    "printf stdout; printf stderr >&2; exit 7".into(),
                ],
            )
        };
        let spec = ProcessSpec {
            program,
            args,
            working_directory: None,
        };
        let result = run_process(&spec, &CapabilitySet::none().allow(Capability::Process))
            .expect("process capability permits execution");
        assert!(!result.success);
        assert_eq!(result.status_code, Some(7));
        assert!(result.stdout.contains("stdout"));
        assert!(result.stderr.contains("stderr"));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn non_idle_reader_stops_after_shutdown_grace() {
        struct SlowAlwaysReadyReader;

        impl Read for SlowAlwaysReadyReader {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                thread::sleep(Duration::from_millis(1));
                buffer[0] = b'x';
                Ok(1)
            }
        }

        let output_limit_exceeded = Arc::new(AtomicBool::new(false));
        let shutdown_requested = Arc::new(AtomicBool::new(true));
        let shutdown_incomplete = Arc::new(AtomicBool::new(false));
        let started = Instant::now();
        let reader = spawn_bounded_reader(
            SlowAlwaysReadyReader,
            Arc::clone(&output_limit_exceeded),
            shutdown_requested,
            Arc::clone(&shutdown_incomplete),
        );
        let _captured = join_reader(reader).expect("bounded reader should return captured bytes");

        assert!(
            started.elapsed() < Duration::from_secs(1),
            "reader exceeded the bounded shutdown grace"
        );
        assert!(
            shutdown_incomplete.load(Ordering::Acquire),
            "a reader that never becomes idle must report incomplete shutdown"
        );
        assert!(
            !output_limit_exceeded.load(Ordering::Acquire),
            "the slow fixture must exercise the non-idle deadline, not output overflow"
        );
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn linux_process_cannot_escape_its_invocation_group_with_setsid() {
        let (working_directory, pid_file) = test_escape_fixture("setsid-normal");
        let executable = std::env::current_exe().expect("current test executable");
        let result = run_process(&ProcessSpec {
            program: executable.to_string_lossy().into_owned(),
            args: vec!["--ignored".into(), "--exact".into(), "interop::tests::process_group_escape_helper_exit".into(), "--nocapture".into()],
            working_directory: Some(working_directory.to_string_lossy().into_owned()),
        }, &CapabilitySet::none().allow(Capability::Process));
        let escaped_pid = cleanup_test_escaped_group(&pid_file);
        let _ = std::fs::remove_dir_all(&working_directory);
        assert!(escaped_pid.is_none(), "a descendant entered a new session; escaped PID: {escaped_pid:?}");
        let result = result.expect("the helper should exit after its descendant is contained");
        assert!(result.success, "helper failed: stdout={:?}, stderr={:?}", result.stdout, result.stderr);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn linux_process_cannot_escape_its_invocation_group_with_setpgid() {
        let (working_directory, pid_file) = test_escape_fixture("setpgid-normal");
        let executable = std::env::current_exe().expect("current test executable");
        let result = run_process(&ProcessSpec {
            program: executable.to_string_lossy().into_owned(),
            args: vec!["--ignored".into(), "--exact".into(), "interop::tests::process_group_setpgid_helper_exit".into(), "--nocapture".into()],
            working_directory: Some(working_directory.to_string_lossy().into_owned()),
        }, &CapabilitySet::none().allow(Capability::Process));
        let escaped_pid = cleanup_test_escaped_group(&pid_file);
        let _ = std::fs::remove_dir_all(&working_directory);
        assert!(escaped_pid.is_none(), "a descendant entered a new process group; escaped PID: {escaped_pid:?}");
        let result = result.expect("the helper should exit after its descendant is contained");
        assert!(result.success, "helper failed: stdout={:?}, stderr={:?}", result.stdout, result.stderr);
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn linux_process_timeout_cannot_leave_a_setpgid_descendant() {
        let (working_directory, pid_file) = test_escape_fixture("setpgid-timeout");
        let executable = std::env::current_exe().expect("current test executable");
        let result = run_process_with_timeout(&ProcessSpec {
            program: executable.to_string_lossy().into_owned(),
            args: vec!["--ignored".into(), "--exact".into(), "interop::tests::process_group_setpgid_helper_timeout".into(), "--nocapture".into()],
            working_directory: Some(working_directory.to_string_lossy().into_owned()),
        }, &CapabilitySet::none().allow(Capability::Process), Duration::from_secs(1));
        let escaped_pid = cleanup_test_escaped_group(&pid_file);
        let _ = std::fs::remove_dir_all(&working_directory);
        assert!(escaped_pid.is_none(), "a descendant survived timeout in another process group; PID: {escaped_pid:?}");
        let error = result.expect_err("the helper parent must hit the configured deadline");
        assert!(error.contains("deadline"), "unexpected timeout result: {error}");
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn linux_output_overflow_cannot_leave_a_setpgid_writer() {
        let (working_directory, pid_file) = test_escape_fixture("setpgid-output");
        let executable = std::env::current_exe().expect("current test executable");
        let result = run_process_with_timeout(&ProcessSpec {
            program: executable.to_string_lossy().into_owned(),
            args: vec!["--ignored".into(), "--exact".into(), "interop::tests::process_group_setpgid_helper_output".into(), "--nocapture".into()],
            working_directory: Some(working_directory.to_string_lossy().into_owned()),
        }, &CapabilitySet::none().allow(Capability::Process), Duration::from_secs(5));
        let escaped_pid = cleanup_test_escaped_group(&pid_file);
        let _ = std::fs::remove_dir_all(&working_directory);
        assert!(escaped_pid.is_none(), "a descendant survived output overflow in another process group; PID: {escaped_pid:?}");
        let error = result.expect_err("stdout overflow must terminate the invocation group");
        assert!(error.contains("output limit"), "unexpected overflow result: {error}");
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn linux_process_timeout_cannot_leave_an_escaped_session_descendant() {
        let (working_directory, pid_file) = test_escape_fixture("setsid-timeout");
        let executable = std::env::current_exe().expect("current test executable");
        let result = run_process_with_timeout(&ProcessSpec {
            program: executable.to_string_lossy().into_owned(),
            args: vec!["--ignored".into(), "--exact".into(), "interop::tests::process_group_escape_helper_timeout".into(), "--nocapture".into()],
            working_directory: Some(working_directory.to_string_lossy().into_owned()),
        }, &CapabilitySet::none().allow(Capability::Process), Duration::from_secs(1));
        let escaped_pid = cleanup_test_escaped_group(&pid_file);
        let _ = std::fs::remove_dir_all(&working_directory);
        assert!(escaped_pid.is_none(), "a descendant survived timeout outside the invocation group; PID: {escaped_pid:?}");
        let error = result.expect_err("the helper parent must hit the configured deadline");
        assert!(error.contains("deadline"), "unexpected timeout result: {error}");
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn linux_output_overflow_cannot_leave_an_escaped_session_writer() {
        let (working_directory, pid_file) = test_escape_fixture("setsid-output");
        let executable = std::env::current_exe().expect("current test executable");
        let result = run_process_with_timeout(&ProcessSpec {
            program: executable.to_string_lossy().into_owned(),
            args: vec!["--ignored".into(), "--exact".into(), "interop::tests::process_group_escape_helper_output".into(), "--nocapture".into()],
            working_directory: Some(working_directory.to_string_lossy().into_owned()),
        }, &CapabilitySet::none().allow(Capability::Process), Duration::from_secs(5));
        let escaped_pid = cleanup_test_escaped_group(&pid_file);
        let _ = std::fs::remove_dir_all(&working_directory);
        assert!(escaped_pid.is_none(), "a descendant survived output overflow outside the invocation group; PID: {escaped_pid:?}");
        let error = result.expect_err("stdout overflow must terminate the invocation group");
        assert!(error.contains("output limit"), "unexpected overflow result: {error}");
    }

    #[cfg(any(target_os = "linux", target_os = "android", windows))]
    #[test]
    fn subprocess_execution_is_bounded_by_a_deadline() {
        let (program, args) = if cfg!(windows) {
            (
                "cmd".to_owned(),
                vec!["/C".into(), "ping -n 20 127.0.0.1 >NUL".into()],
            )
        } else {
            ("sh".to_owned(), vec!["-c".into(), "sleep 20".into()])
        };
        let started = Instant::now();
        let error = run_process(
            &ProcessSpec {
                program,
                args,
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("long-running process must be terminated");
        assert!(error.contains("deadline"));
        assert!(started.elapsed() < Duration::from_secs(15));
    }

    #[cfg(any(target_os = "linux", target_os = "android", windows))]
    #[test]
    fn aggregate_output_limit_is_explicit_and_checked() {
        assert_eq!(MAX_OUTPUT_BYTES_TOTAL, 2 * MAX_OUTPUT_BYTES_PER_STREAM);
        assert!(validate_total_output_limit(
            MAX_OUTPUT_BYTES_PER_STREAM,
            MAX_OUTPUT_BYTES_PER_STREAM
        )
        .is_ok());
        let error = validate_total_output_limit(
            MAX_OUTPUT_BYTES_PER_STREAM,
            MAX_OUTPUT_BYTES_PER_STREAM.saturating_add(1),
        )
        .expect_err("aggregate output above two stream caps must be rejected");
        assert!(error.contains("aggregate output limit"));
    }

    #[cfg(any(target_os = "linux", target_os = "android", windows))]
    #[test]
    fn subprocess_stdout_capture_has_a_hard_limit() {
        let (program, args) = if cfg!(windows) {
            (
                "powershell.exe".to_owned(),
                vec![
                    "-NoProfile".into(),
                    "-NonInteractive".into(),
                    "-Command".into(),
                    "$b = New-Object byte[] 1048577; [Console]::OpenStandardOutput().Write($b,0,$b.Length)".into(),
                ],
            )
        } else {
            (
                "sh".to_owned(),
                vec!["-c".into(), "head -c 1048577 /dev/zero".into()],
            )
        };
        let error = run_process_with_timeout(
            &ProcessSpec {
                program,
                args,
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
            if cfg!(windows) {
                Duration::from_secs(30)
            } else {
                PROCESS_TIMEOUT
            },
        )
        .expect_err("stdout beyond the configured cap must be rejected");
        assert!(
            error.contains("output limit"),
            "unexpected bounded stdout error: {error}"
        );
    }

    #[cfg(any(target_os = "linux", target_os = "android", windows))]
    #[test]
    fn subprocess_stderr_capture_has_a_hard_limit() {
        let (program, args) = if cfg!(windows) {
            (
                "powershell.exe".to_owned(),
                vec![
                    "-NoProfile".into(),
                    "-NonInteractive".into(),
                    "-Command".into(),
                    "$b = New-Object byte[] 1048577; [Console]::OpenStandardError().Write($b,0,$b.Length)".into(),
                ],
            )
        } else {
            (
                "sh".to_owned(),
                vec!["-c".into(), "head -c 1048577 /dev/zero >&2".into()],
            )
        };
        let error = run_process_with_timeout(
            &ProcessSpec {
                program,
                args,
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
            if cfg!(windows) {
                Duration::from_secs(30)
            } else {
                PROCESS_TIMEOUT
            },
        )
        .expect_err("stderr beyond the configured cap must be rejected");
        assert!(
            error.contains("output limit"),
            "unexpected bounded stderr error: {error}"
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_job_object_terminates_descendants_when_parent_exits() {
        use windows_sys::Win32::Foundation::{HANDLE, WAIT_OBJECT_0};
        use windows_sys::Win32::System::Threading::{
            GetExitCodeProcess, GetProcessId, INFINITE, TerminateProcess, WaitForSingleObject,
        };

        let parent_pid = std::process::id();
        let script = "$ErrorActionPreference = 'Stop'; Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public static class JobProbe { [DllImport(\"kernel32.dll\", SetLastError=true)] public static extern bool IsProcessInJob(IntPtr process, IntPtr job, out bool result); [DllImport(\"kernel32.dll\", SetLastError=true)] public static extern bool SetHandleInformation(IntPtr handle, uint mask, uint flags); [DllImport(\"kernel32.dll\", SetLastError=true)] public static extern IntPtr OpenProcess(uint access, bool inherit, uint pid); [DllImport(\"kernel32.dll\", SetLastError=true)] public static extern bool DuplicateHandle(IntPtr sourceProcess, IntPtr sourceHandle, IntPtr targetProcess, out IntPtr targetHandle, uint access, bool inherit, uint options); [DllImport(\"kernel32.dll\")] public static extern IntPtr GetCurrentProcess(); [DllImport(\"kernel32.dll\", SetLastError=true)] public static extern bool CloseHandle(IntPtr handle); }'; $jobHandle = [IntPtr]::new([Int64]::Parse('__ONE_JOB_QUERY_HANDLE__')); if (-not [JobProbe]::SetHandleInformation($jobHandle, 1, 0)) { throw ('SetHandleInformation failed: ' + [Runtime.InteropServices.Marshal]::GetLastWin32Error()) }; $psi = [System.Diagnostics.ProcessStartInfo]::new(); $psi.FileName = 'ping.exe'; $psi.Arguments = '-n 30 127.0.0.1'; $psi.UseShellExecute = $false; $child = [System.Diagnostics.Process]::Start($psi); if ($null -eq $child) { throw 'Process.Start returned null' }; [Console]::Out.WriteLine(('CHILD_PID=' + $child.Id)); try { $inJob = $false; $queryOk = [JobProbe]::IsProcessInJob($child.Handle, $jobHandle, [ref]$inJob); if (-not $queryOk) { throw ('IsProcessInJob(target job) failed: ' + [Runtime.InteropServices.Marshal]::GetLastWin32Error()) }; [Console]::Out.WriteLine(('CHILD_IN_INVOCATION_JOB=' + $inJob.ToString().ToLowerInvariant())); $parent = [JobProbe]::OpenProcess(0x0040, $false, [uint32]__ONE_PARENT_PID__); if ($parent -eq [IntPtr]::Zero) { throw ('OpenProcess(parent PROCESS_DUP_HANDLE) failed: ' + [Runtime.InteropServices.Marshal]::GetLastWin32Error()) }; $childWaitHandle = [IntPtr]::Zero; try { if (-not [JobProbe]::DuplicateHandle([JobProbe]::GetCurrentProcess(), $child.Handle, $parent, [ref]$childWaitHandle, 0x00101001, $false, 0)) { throw ('DuplicateHandle(child TERMINATE|QUERY_LIMITED_INFORMATION|SYNCHRONIZE into parent) failed: ' + [Runtime.InteropServices.Marshal]::GetLastWin32Error()) } } finally { [void][JobProbe]::CloseHandle($parent) }; [Console]::Out.WriteLine(('CHILD_WAIT_HANDLE=' + $childWaitHandle.ToInt64())) } catch { try { if (-not $child.HasExited) { $child.Kill(); $child.WaitForExit() } } catch {}; throw }";
        let result = run_process_with_timeout(
            &ProcessSpec {
                program: "powershell.exe".into(),
                args: vec![
                    "-NoProfile".into(),
                    "-Command".into(),
                    script.replace("__ONE_PARENT_PID__", &parent_pid.to_string()),
                ],
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
            Duration::from_secs(30),
        )
        .expect("PowerShell parent process should complete within its test-only setup budget");
        let child_in_invocation_job = result
            .stdout
            .lines()
            .find_map(|line| line.strip_prefix("CHILD_IN_INVOCATION_JOB="))
            .unwrap_or_else(|| panic!(
                "PowerShell did not report descendant membership in the invocation Job Object; stdout={:?}, stderr={:?}",
                result.stdout, result.stderr
            ))
            .trim()
            .parse::<bool>()
            .expect("PowerShell should report a lowercase boolean Job membership result");

        let pid = result
            .stdout
            .lines()
            .find_map(|line| line.strip_prefix("CHILD_PID="))
            .unwrap_or_else(|| {
                panic!(
                    "PowerShell did not report a child PID; stdout={:?}, stderr={:?}",
                    result.stdout, result.stderr
                )
            })
            .trim()
            .parse::<u32>()
            .expect("PowerShell should report a numeric descendant PID");

        // The child was created after the parent was assigned to the Job Object.
        // Closing the kill-on-close job must terminate it before run_process returns.
        let wait_handle = result
            .stdout
            .lines()
            .find_map(|line| line.strip_prefix("CHILD_WAIT_HANDLE="))
            .unwrap_or_else(|| panic!(
                "PowerShell did not duplicate the descendant wait handle into the test process; stdout={:?}, stderr={:?}",
                result.stdout, result.stderr
            ))
            .trim()
            .parse::<i64>()
            .expect("PowerShell should report a numeric duplicated process handle") as usize as HANDLE;
        assert!(
            !wait_handle.is_null(),
            "duplicated descendant process handle must be non-null"
        );
        // Confirm that the retained handle still identifies the exact process
        // whose PID the wrapper reported; this validates the cross-process
        // DuplicateHandle direction and rules out a malformed or stale handle.
        let retained_pid = unsafe { GetProcessId(wait_handle) };
        let mut retained_exit_code = 0u32;
        let exit_code_ok = unsafe { GetExitCodeProcess(wait_handle, &mut retained_exit_code) } != 0;
        // A process that is still running must report STILL_ACTIVE (259).
        // GetExitCodeProcess returning any other code is the authoritative
        // process-object state; retain the zero-time wait result as a second
        // diagnostic because this runner has reported WAIT_TIMEOUT even when
        // the same handle already exposes a final exit code.
        let wait = unsafe { WaitForSingleObject(wait_handle, 0) };
        let process_still_active = exit_code_ok && retained_exit_code == 259;
        if !exit_code_ok || process_still_active {
            // Reap only the test-owned ping process through the same retained
            // handle (never reopen by PID).
            unsafe {
                TerminateProcess(wait_handle, 1);
                WaitForSingleObject(wait_handle, INFINITE);
            }
        }
        unsafe { windows_sys::Win32::Foundation::CloseHandle(wait_handle) };

        assert!(
            child_in_invocation_job,
            "descendant was not in the invocation's assigned Job Object; stdout={:?}, stderr={:?}",
            result.stdout, result.stderr
        );
        assert_eq!(
            retained_pid, pid,
            "retained process handle identity mismatch: reported PID {pid}, handle PID {retained_pid}, exit-code query ok={exit_code_ok}, exit code={retained_exit_code}"
        );
        assert!(
            exit_code_ok,
            "GetExitCodeProcess failed for retained descendant handle; PID={pid}, wait result={wait}"
        );
        assert!(
            !process_still_active,
            "descendant process {pid} remained active after the bounded process invocation; retained handle PID={retained_pid}, exit code={retained_exit_code}, wait result={wait}"
        );
        assert!(
            wait == WAIT_OBJECT_0
                || (wait == windows_sys::Win32::Foundation::WAIT_TIMEOUT && !process_still_active),
            "unexpected wait result {wait} for terminated descendant {pid} (exit code {retained_exit_code})"
        );
    }
}
