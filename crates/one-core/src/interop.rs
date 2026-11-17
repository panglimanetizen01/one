use std::io::{self, Read};
#[cfg(unix)]
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::capability::{Capability, CapabilitySet};

const PROCESS_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_OUTPUT_BYTES_PER_STREAM: usize = 1024 * 1024;
#[cfg(unix)]
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(10);

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

fn spawn_bounded_reader<R: Read + Send + 'static>(
    mut reader: R,
    output_limit_exceeded: Arc<AtomicBool>,
) -> JoinHandle<io::Result<Vec<u8>>> {
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

fn join_reader(reader: JoinHandle<io::Result<Vec<u8>>>) -> Result<Vec<u8>, String> {
    reader
        .join()
        .map_err(|_| "Subprocess output reader panicked.".to_owned())?
        .map_err(|error| format!("Failed to read subprocess output: {error}"))
}

pub fn run_process(
    spec: &ProcessSpec,
    capabilities: &CapabilitySet,
) -> Result<ProcessResult, String> {
    if !capabilities.permits(Capability::Process) {
        return Err("Kemampuan proses belum diberikan kepada program.".into());
    }

    #[cfg(unix)]
    {
        run_process_unix(spec)
    }
    #[cfg(windows)]
    {
        run_process_windows(spec)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = spec;
        Err("Bounded process execution is unsupported on this platform.".into())
    }
}

#[cfg(unix)]
fn run_process_unix(spec: &ProcessSpec) -> Result<ProcessResult, String> {
    use std::os::unix::process::CommandExt;

    let mut command = Command::new(&spec.program);
    command
        .args(&spec.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    if let Some(directory) = &spec.working_directory {
        command.current_dir(directory);
    }

    let mut child = command
        .spawn()
        .map_err(|error| format!("Gagal menjalankan proses '{}': {error}", spec.program))?;
    let process_group = i32::try_from(child.id())
        .map_err(|_| "Subprocess identifier exceeds the supported process-group range.".to_owned())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Subprocess stdout pipe was not created.".to_owned())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Subprocess stderr pipe was not created.".to_owned())?;

    let output_limit_exceeded = Arc::new(AtomicBool::new(false));
    let stdout_reader = spawn_bounded_reader(stdout, Arc::clone(&output_limit_exceeded));
    let stderr_reader = spawn_bounded_reader(stderr, Arc::clone(&output_limit_exceeded));
    let deadline = Instant::now() + PROCESS_TIMEOUT;
    let mut timed_out = false;

    let status = loop {
        if output_limit_exceeded.load(Ordering::Acquire) {
            kill_process_group(process_group);
            let _ = child.kill();
            break child.wait();
        }

        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => {}
            Err(error) => {
                kill_process_group(process_group);
                let _ = child.kill();
                let _ = child.wait();
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

    // A child may exit while leaving descendants holding the inherited pipes.
    // They share this process group and must not outlive the bounded invocation.
    kill_process_group(process_group);
    let stdout = join_reader(stdout_reader)?;
    let stderr = join_reader(stderr_reader)?;
    let status = status.map_err(|error| format!("Failed to reap subprocess: {error}"))?;

    if timed_out {
        return Err(format!(
            "Subprocess exceeded the {} second execution deadline.",
            PROCESS_TIMEOUT.as_secs()
        ));
    }
    if output_limit_exceeded.load(Ordering::Acquire) {
        return Err(format!(
            "Subprocess exceeded the {} byte per-stream output limit.",
            MAX_OUTPUT_BYTES_PER_STREAM
        ));
    }

    Ok(ProcessResult {
        status_code: status.code(),
        success: status.success(),
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
    })
}

#[cfg(unix)]
fn kill_process_group(process_group: i32) {
    // SAFETY: a negative pid addresses only the process group created for this
    // invocation via CommandExt::process_group(0).
    unsafe {
        libc::kill(-process_group, libc::SIGKILL);
    }
}

#[cfg(windows)]
fn run_process_windows(spec: &ProcessSpec) -> Result<ProcessResult, String> {
    use std::ffi::{OsStr, OsString};
    use std::fs::File;
    use std::mem::{size_of, size_of_val};
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::io::FromRawHandle;
    use std::path::Path;
    use std::ptr::{null, null_mut};

    use windows_sys::Win32::Foundation::{
        CloseHandle, GetLastError, SetHandleInformation, HANDLE, HANDLE_FLAG_INHERIT,
        WAIT_OBJECT_0, WAIT_TIMEOUT,
    };
    use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ,
        OPEN_EXISTING,
    };
    use windows_sys::Win32::System::JobObjects::{
        CreateJobObjectW, SetInformationJobObject, TerminateJobObject,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        JobObjectExtendedLimitInformation,
    };
    use windows_sys::Win32::System::Pipes::CreatePipe;
    use windows_sys::Win32::System::Threading::{
        CreateProcessW, DeleteProcThreadAttributeList, GetExitCodeProcess,
        InitializeProcThreadAttributeList, UpdateProcThreadAttribute, WaitForSingleObject,
        PROCESS_INFORMATION, STARTUPINFOEXW, STARTUPINFOW, CREATE_NO_WINDOW,
        EXTENDED_STARTUPINFO_PRESENT, INFINITE, PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
        PROC_THREAD_ATTRIBUTE_JOB_LIST, STARTF_USESTDHANDLES,
    };

    struct Handle(HANDLE);
    impl Handle {
        fn new(handle: HANDLE) -> Result<Self, String> {
            if handle.is_null() || handle as isize == -1 {
                Err(format!("Windows handle creation failed (error {}).", unsafe {
                    GetLastError()
                }))
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
        let extensions: &[&str] = if has_extension { &[""] } else { &[".exe", ".com"] };
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
        Err(format!("Executable '{}' was not found in PATH.", path.display()))
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

    fn create_pipe_pair(
        security: &SECURITY_ATTRIBUTES,
    ) -> Result<(Handle, Handle), String> {
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
    let mut command_line = Vec::new();
    append_windows_argument(&mut command_line, &executable);
    for argument in &spec.args {
        command_line.push(0x20);
        append_windows_argument(&mut command_line, OsStr::new(argument));
    }
    command_line.push(0);

    let mut security = SECURITY_ATTRIBUTES {
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
        return Err(format!("SetInformationJobObject failed (error {}).", unsafe {
            GetLastError()
        }));
    }

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
    if unsafe {
        InitializeProcThreadAttributeList(
            attribute_pointer,
            2,
            0,
            &mut attribute_size,
        )
    } == 0
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

    let inherited_handles = [stdin.raw(), stdout_write.raw(), stderr_write.raw()];
    // SAFETY: every handle in this list is inheritable and remains alive until
    // CreateProcessW returns; parent-side read handles are explicitly non-inheritable.
    if unsafe {
        UpdateProcThreadAttribute(
            attributes.raw,
            0,
            PROC_THREAD_ATTRIBUTE_HANDLE_LIST as usize,
            inherited_handles.as_ptr().cast(),
            size_of_val(&inherited_handles),
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

    let process = Handle::new(process_information.hProcess)?;
    let thread_handle = Handle::new(process_information.hThread)?;
    drop(thread_handle);
    drop(attributes);
    drop(stdin);
    drop(stdout_write);
    drop(stderr_write);

    // SAFETY: ownership of the read handles is transferred to File, which closes them.
    let stdout_file = unsafe { File::from_raw_handle(stdout_read.into_raw() as _) };
    let stderr_file = unsafe { File::from_raw_handle(stderr_read.into_raw() as _) };
    let output_limit_exceeded = Arc::new(AtomicBool::new(false));
    let stdout_reader = spawn_bounded_reader(stdout_file, Arc::clone(&output_limit_exceeded));
    let stderr_reader = spawn_bounded_reader(stderr_file, Arc::clone(&output_limit_exceeded));

    let deadline = Instant::now() + PROCESS_TIMEOUT;
    let mut timed_out = false;
    let mut wait_failed = false;
    loop {
        if output_limit_exceeded.load(Ordering::Acquire) {
            // SAFETY: the job contains this process and every descendant.
            unsafe {
                TerminateJobObject(job.raw(), 1);
                WaitForSingleObject(process.raw(), INFINITE);
            }
            break;
        }
        if Instant::now() >= deadline {
            timed_out = true;
            // SAFETY: terminating the job also terminates all contained descendants.
            unsafe {
                TerminateJobObject(job.raw(), 1);
                WaitForSingleObject(process.raw(), INFINITE);
            }
            break;
        }
        // SAFETY: process is a valid process handle and the timeout is bounded.
        match unsafe { WaitForSingleObject(process.raw(), 25) } {
            WAIT_OBJECT_0 => break,
            WAIT_TIMEOUT => {}
            _ => {
                wait_failed = true;
                // SAFETY: fail closed and reap the process tree.
                unsafe {
                    TerminateJobObject(job.raw(), 1);
                    WaitForSingleObject(process.raw(), INFINITE);
                }
                break;
            }
        }
    }

    // KILL_ON_JOB_CLOSE prevents descendants from surviving even if the direct
    // child exits before they do.
    drop(job);
    let mut exit_code = 1u32;
    // SAFETY: the process has signaled or was terminated and remains open.
    if unsafe { GetExitCodeProcess(process.raw(), &mut exit_code) } == 0 {
        wait_failed = true;
    }
    drop(process);

    let stdout = join_reader(stdout_reader)?;
    let stderr = join_reader(stderr_reader)?;
    if wait_failed {
        return Err("Waiting for the Windows subprocess failed.".into());
    }
    if timed_out {
        return Err(format!(
            "Subprocess exceeded the {} second execution deadline.",
            PROCESS_TIMEOUT.as_secs()
        ));
    }
    if output_limit_exceeded.load(Ordering::Acquire) {
        return Err(format!(
            "Subprocess exceeded the {} byte per-stream output limit.",
            MAX_OUTPUT_BYTES_PER_STREAM
        ));
    }

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

    #[test]
    fn denied_process_capability_never_spawns() {
        let spec = ProcessSpec {
            program: "one-test-process-that-must-not-run".into(),
            args: vec![],
            working_directory: None,
        };
        let error = run_process(&spec, &CapabilitySet::none())
            .expect_err("process must be denied");
        assert_eq!(error, "Kemampuan proses belum diberikan kepada program.");
    }

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

    #[test]
    fn subprocess_stdout_capture_has_a_hard_limit() {
        let (program, args) = if cfg!(windows) {
            (
                "powershell.exe".to_owned(),
                vec![
                    "-NoProfile".into(),
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
        let error = run_process(
            &ProcessSpec {
                program,
                args,
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("stdout beyond the configured cap must be rejected");
        assert!(error.contains("output limit"));
    }

    #[test]
    fn subprocess_stderr_capture_has_a_hard_limit() {
        let (program, args) = if cfg!(windows) {
            (
                "powershell.exe".to_owned(),
                vec![
                    "-NoProfile".into(),
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
        let error = run_process(
            &ProcessSpec {
                program,
                args,
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect_err("stderr beyond the configured cap must be rejected");
        assert!(error.contains("output limit"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_job_object_terminates_descendants_when_parent_exits() {
        use windows_sys::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0};
        use windows_sys::Win32::System::Threading::{
            OpenProcess, WaitForSingleObject, PROCESS_QUERY_LIMITED_INFORMATION, SYNCHRONIZE,
        };

        let script = "$p = Start-Process -FilePath ping.exe -ArgumentList @('-n','30','127.0.0.1') -PassThru; [Console]::Out.WriteLine($p.Id)";
        let result = run_process(
            &ProcessSpec {
                program: "powershell.exe".into(),
                args: vec!["-NoProfile".into(), "-Command".into(), script.into()],
                working_directory: None,
            },
            &CapabilitySet::none().allow(Capability::Process),
        )
        .expect("PowerShell parent process should complete");
        let pid = result
            .stdout
            .trim()
            .parse::<u32>()
            .expect("PowerShell should report the descendant PID");

        // The child was created after the parent was assigned to the Job Object.
        // Closing the kill-on-close job must terminate it before run_process returns.
        let process = unsafe {
            OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION | SYNCHRONIZE,
                0,
                pid,
            )
        };
        if !process.is_null() {
            let wait = unsafe { WaitForSingleObject(process, 0) };
            unsafe { CloseHandle(process) };
            assert_eq!(
                wait, WAIT_OBJECT_0,
                "descendant process {pid} survived the bounded process invocation"
            );
        }
    }

}
