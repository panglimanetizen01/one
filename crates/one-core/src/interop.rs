//! Bounded subprocess execution.
//!
//! Defaults: 10 seconds wall-clock time, 1 MiB stdout, 1 MiB stderr, and
//! 2 MiB combined captured output. Capture is read concurrently to avoid
//! stdout/stderr pipe deadlocks. A timeout or output-limit breach kills and
//! reaps the direct child. On Unix, the child is placed in a new process group
//! and the whole group is killed on timeout/limit (and cleaned up after the
//! leader exits, so descendants cannot keep inherited pipes open). On Windows,
//! this implementation guarantees termination/reaping of the direct child;
//! remaining descendants are terminated by a kill-on-close Windows Job Object.
use std::io::Read;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(windows)]
use std::mem::{size_of, zeroed};
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
#[cfg(windows)]
use std::ptr::null;
#[cfg(windows)]
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
    SetInformationJobObject,
};

use crate::capability::{Capability, CapabilitySet};

#[cfg(windows)]
struct WindowsJob {
    handle: Option<HANDLE>,
}

#[cfg(windows)]
impl WindowsJob {
    fn assign(child: &Child) -> Result<Self, String> {
        // SAFETY: null attributes/name request a private unnamed job object.
        let handle = unsafe { CreateJobObjectW(null(), null()) };
        if handle.is_null() {
            return Err("Gagal membuat Windows Job Object untuk proses.".into());
        }
        // SAFETY: zero initialization followed by setting the documented limit flag.
        let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { zeroed() };
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        // SAFETY: handle and structure are valid for this API call.
        let configured = unsafe {
            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &limits as *const _ as *const core::ffi::c_void,
                size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        };
        if configured == 0 {
            // SAFETY: handle was created above and has not been closed.
            unsafe { CloseHandle(handle) };
            return Err("Gagal mengonfigurasi Windows Job Object.".into());
        }
        // SAFETY: Child owns this valid process handle; assignment does not transfer ownership.
        let assigned = unsafe { AssignProcessToJobObject(handle, child.as_raw_handle() as HANDLE) };
        if assigned == 0 {
            // SAFETY: handle was created above and has not been closed.
            unsafe { CloseHandle(handle) };
            return Err("Gagal memasukkan proses ke Windows Job Object.".into());
        }
        Ok(Self {
            handle: Some(handle),
        })
    }

    fn close_now(&mut self) {
        if let Some(handle) = self.handle.take() {
            // Closing a kill-on-close job terminates any remaining assigned processes.
            // SAFETY: this object uniquely owns the handle and closes it at most once.
            unsafe { CloseHandle(handle) };
        }
    }
}

#[cfg(windows)]
impl Drop for WindowsJob {
    fn drop(&mut self) {
        self.close_now();
    }
}

pub const DEFAULT_PROCESS_TIMEOUT: Duration = Duration::from_secs(10);
pub const DEFAULT_STDOUT_LIMIT: usize = 1024 * 1024;
pub const DEFAULT_STDERR_LIMIT: usize = 1024 * 1024;
pub const DEFAULT_TOTAL_OUTPUT_LIMIT: usize = 2 * 1024 * 1024;
const POLL_INTERVAL: Duration = Duration::from_millis(10);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSpec {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessLimits {
    pub timeout: Duration,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub total_output_bytes: usize,
}

impl Default for ProcessLimits {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_PROCESS_TIMEOUT,
            stdout_bytes: DEFAULT_STDOUT_LIMIT,
            stderr_bytes: DEFAULT_STDERR_LIMIT,
            total_output_bytes: DEFAULT_TOTAL_OUTPUT_LIMIT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub status_code: Option<i32>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_process(
    spec: &ProcessSpec,
    capabilities: &CapabilitySet,
) -> Result<ProcessResult, String> {
    run_process_with_limits(spec, capabilities, ProcessLimits::default())
}

pub fn run_process_with_limits(
    spec: &ProcessSpec,
    capabilities: &CapabilitySet,
    limits: ProcessLimits,
) -> Result<ProcessResult, String> {
    if !capabilities.permits(Capability::Process) {
        return Err("Kemampuan proses belum diberikan kepada program.".into());
    }
    if limits.timeout.is_zero() {
        return Err("Batas waktu proses harus lebih besar dari nol.".into());
    }

    let mut command = Command::new(&spec.program);
    command
        .args(&spec.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(dir) = &spec.working_directory {
        command.current_dir(dir);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }

    let mut child = command
        .spawn()
        .map_err(|error| format!("Gagal menjalankan proses '{}': {}", spec.program, error))?;
    #[cfg(windows)]
    let mut windows_job = match WindowsJob::assign(&child) {
        Ok(job) => Some(job),
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }
    };
    let stdout = child.stdout.take().expect("piped stdout must be available");
    let stderr = child.stderr.take().expect("piped stderr must be available");

    let total = Arc::new(AtomicUsize::new(0));
    let exceeded = Arc::new(AtomicBool::new(false));
    let stdout_reader = spawn_bounded_reader(
        stdout,
        limits.stdout_bytes,
        limits.total_output_bytes,
        Arc::clone(&total),
        Arc::clone(&exceeded),
    );
    let stderr_reader = spawn_bounded_reader(
        stderr,
        limits.stderr_bytes,
        limits.total_output_bytes,
        Arc::clone(&total),
        Arc::clone(&exceeded),
    );

    let deadline = Instant::now() + limits.timeout;
    let mut status: Option<ExitStatus> = None;
    let mut timed_out = false;
    loop {
        if exceeded.load(Ordering::Acquire) {
            terminate_child(&mut child);
            break;
        }
        match child.try_wait() {
            Ok(Some(exit_status)) => {
                status = Some(exit_status);
                break;
            }
            Ok(None) => {}
            Err(error) => {
                terminate_child(&mut child);
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err(format!(
                    "Gagal memeriksa proses '{}': {}",
                    spec.program, error
                ));
            }
        }
        if Instant::now() >= deadline {
            timed_out = true;
            terminate_child(&mut child);
            break;
        }
        thread::sleep(POLL_INTERVAL);
    }

    // Close process groups/jobs before joining readers so descendants cannot
    // keep inherited pipe handles open after the direct child exits.
    #[cfg(unix)]
    if status.is_some() {
        kill_process_group(child.id());
    }
    #[cfg(windows)]
    if let Some(job) = windows_job.as_mut() {
        job.close_now();
    }

    let stdout = stdout_reader
        .join()
        .map_err(|_| "Pembaca stdout proses gagal.".to_owned())?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| "Pembaca stderr proses gagal.".to_owned())?;

    if timed_out {
        return Err(format!(
            "Proses '{}' melewati batas waktu {:?} dan dihentikan.",
            spec.program, limits.timeout
        ));
    }
    if exceeded.load(Ordering::Acquire) {
        return Err(format!(
            "Proses '{}' melewati batas output (stdout={} byte, stderr={} byte, total={} byte) dan dihentikan.",
            spec.program, limits.stdout_bytes, limits.stderr_bytes, limits.total_output_bytes
        ));
    }
    let status = match status {
        Some(status) => status,
        None => child
            .try_wait()
            .map_err(|error| {
                format!(
                    "Gagal mengambil status proses '{}': {}",
                    spec.program, error
                )
            })?
            .ok_or_else(|| {
                format!(
                    "Status proses '{}' tidak tersedia setelah dihentikan.",
                    spec.program
                )
            })?,
    };

    Ok(ProcessResult {
        status_code: status.code(),
        success: status.success(),
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
    })
}

fn spawn_bounded_reader<R: Read + Send + 'static>(
    mut reader: R,
    stream_limit: usize,
    total_limit: usize,
    total: Arc<AtomicUsize>,
    exceeded: Arc<AtomicBool>,
) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut captured = Vec::with_capacity(stream_limit.min(8192));
        let mut buffer = [0u8; 8192];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(count) => {
                    if captured.len().saturating_add(count) > stream_limit {
                        exceeded.store(true, Ordering::Release);
                        break;
                    }
                    let reserved =
                        total.fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                            current
                                .checked_add(count)
                                .filter(|next| *next <= total_limit)
                        });
                    if reserved.is_err() {
                        exceeded.store(true, Ordering::Release);
                        break;
                    }
                    captured.extend_from_slice(&buffer[..count]);
                }
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }
        captured
    })
}

fn terminate_child(child: &mut Child) {
    #[cfg(unix)]
    kill_process_group(child.id());
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(unix)]
fn kill_process_group(child_id: u32) {
    // The child was started in a new process group whose ID is its PID.
    // SAFETY: kill is called with a negative process-group ID and no pointers.
    unsafe {
        libc::kill(-(child_id as libc::pid_t), libc::SIGKILL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn allowed() -> CapabilitySet {
        CapabilitySet::none().allow(Capability::Process)
    }

    fn spec(script: &str) -> ProcessSpec {
        #[cfg(windows)]
        let (program, args) = (
            "powershell".to_owned(),
            vec![
                "-NoProfile".into(),
                "-NonInteractive".into(),
                "-Command".into(),
                script.into(),
            ],
        );
        #[cfg(not(windows))]
        let (program, args) = ("sh".to_owned(), vec!["-c".into(), script.into()]);
        ProcessSpec {
            program,
            args,
            working_directory: None,
        }
    }

    fn tight_limits() -> ProcessLimits {
        ProcessLimits {
            timeout: Duration::from_secs(3),
            stdout_bytes: 4096,
            stderr_bytes: 4096,
            total_output_bytes: 6144,
        }
    }

    #[test]
    fn denied_process_capability_never_spawns() {
        let process = ProcessSpec {
            program: "one-test-process-that-must-not-run".into(),
            args: vec![],
            working_directory: None,
        };
        let error =
            run_process(&process, &CapabilitySet::none()).expect_err("process must be denied");
        assert_eq!(error, "Kemampuan proses belum diberikan kepada program.");
    }

    #[test]
    fn allowed_process_preserves_exit_status_and_streams() {
        let process = if cfg!(windows) {
            spec("Write-Output stdout; [Console]::Error.Write('stderr'); exit 7")
        } else {
            spec("printf stdout; printf stderr >&2; exit 7")
        };
        let result =
            run_process(&process, &allowed()).expect("process capability permits execution");
        assert!(!result.success);
        assert_eq!(result.status_code, Some(7));
        assert!(result.stdout.contains("stdout"));
        assert!(result.stderr.contains("stderr"));
    }

    #[test]
    fn process_timeout_is_enforced_and_child_is_reaped() {
        let process = if cfg!(windows) {
            spec("Start-Sleep -Seconds 5")
        } else {
            spec("sleep 5")
        };
        let limits = ProcessLimits {
            timeout: Duration::from_millis(100),
            ..ProcessLimits::default()
        };
        let error = run_process_with_limits(&process, &allowed(), limits)
            .expect_err("long process must time out");
        assert!(error.contains("melewati batas waktu"), "{error}");
    }

    #[cfg(windows)]
    #[test]
    fn windows_job_object_terminates_descendant_after_parent_exits() {
        let process = spec(
            "$child = Start-Process -FilePath powershell.exe -ArgumentList @('-NoProfile','-NonInteractive','-Command','Start-Sleep -Seconds 30') -PassThru; Write-Output $child.Id; exit 0",
        );
        let result = run_process(&process, &allowed())
            .expect("parent should exit successfully while descendant is contained");
        let child_pid: u32 = result
            .stdout
            .trim()
            .parse()
            .expect("child PID should be printed");
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            let child_exists = std::process::Command::new("powershell.exe")
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    &format!("if (Get-Process -Id {child_pid} -ErrorAction SilentlyContinue) {{ exit 0 }} else {{ exit 1 }}"),
                ])
                .status()
                .expect("PowerShell process query should run")
                .success();
            if !child_exists {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "descendant process {child_pid} survived Job Object closure"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    #[test]
    fn stdout_flood_is_bounded() {
        let process = if cfg!(windows) {
            spec(
                "while ($true) { [Console]::Out.Write('xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx') }",
            )
        } else {
            spec("yes x")
        };
        let error = run_process_with_limits(&process, &allowed(), tight_limits())
            .expect_err("stdout flood must be stopped");
        assert!(error.contains("batas output"), "{error}");
    }

    #[test]
    fn stderr_flood_is_bounded() {
        let process = if cfg!(windows) {
            spec(
                "while ($true) { [Console]::Error.Write('xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx') }",
            )
        } else {
            spec("yes x >&2")
        };
        let error = run_process_with_limits(&process, &allowed(), tight_limits())
            .expect_err("stderr flood must be stopped");
        assert!(error.contains("batas output"), "{error}");
    }

    #[test]
    fn aggregate_output_limit_is_enforced() {
        let process = if cfg!(windows) {
            spec(
                "1..1000 | ForEach-Object { [Console]::Out.Write('xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'); [Console]::Error.Write('yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy') }",
            )
        } else {
            spec(
                "i=0; while [ $i -lt 1000 ]; do printf 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'; printf 'yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy' >&2; i=$((i+1)); done",
            )
        };
        let limits = ProcessLimits {
            total_output_bytes: 1024,
            stdout_bytes: 4096,
            stderr_bytes: 4096,
            ..tight_limits()
        };
        let error = run_process_with_limits(&process, &allowed(), limits)
            .expect_err("aggregate flood must be stopped");
        assert!(error.contains("batas output"), "{error}");
    }
}
