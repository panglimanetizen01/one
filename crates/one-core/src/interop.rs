use std::io::{self, Read};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::capability::{Capability, CapabilitySet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSpec { pub program: String, pub args: Vec<String>, pub working_directory: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessLimits {
    pub timeout: Duration,
    pub max_stdout_bytes: usize,
    pub max_stderr_bytes: usize,
}

impl Default for ProcessLimits {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_stdout_bytes: 1024 * 1024,
            max_stderr_bytes: 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub status_code: Option<i32>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

fn read_bounded<R: Read>(mut reader: R, limit: usize) -> io::Result<(Vec<u8>, bool)> {
    let mut captured = Vec::with_capacity(limit.min(8192));
    let mut buffer = [0_u8; 8192];
    let mut truncated = false;

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        let remaining = limit.saturating_sub(captured.len());
        let retained = read.min(remaining);
        captured.extend_from_slice(&buffer[..retained]);
        truncated |= retained < read;
        // Continue draining after the retention limit so the child cannot block
        // forever on a full pipe while the parent waits for process termination.
    }

    Ok((captured, truncated))
}

fn terminate_and_wait(child: &mut Child) -> io::Result<ExitStatus> {
    let _ = child.kill();
    child.wait()
}

pub fn run_process(spec: &ProcessSpec, capabilities: &CapabilitySet) -> Result<ProcessResult, String> {
    run_process_with_limits(spec, capabilities, &ProcessLimits::default())
}

pub fn run_process_with_limits(
    spec: &ProcessSpec,
    capabilities: &CapabilitySet,
    limits: &ProcessLimits,
) -> Result<ProcessResult, String> {
    if !capabilities.permits(Capability::Process) {
        return Err("Kemampuan proses belum diberikan kepada program.".into());
    }

    let mut command = Command::new(&spec.program);
    command.args(&spec.args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(dir) = &spec.working_directory {
        command.current_dir(dir);
    }

    let mut child = command
        .spawn()
        .map_err(|error| format!("Gagal menjalankan proses '{}': {}", spec.program, error))?;
    let stdout = child.stdout.take().ok_or_else(|| "Gagal mengambil stdout proses.".to_owned())?;
    let stderr = child.stderr.take().ok_or_else(|| "Gagal mengambil stderr proses.".to_owned())?;

    let stdout_limit = limits.max_stdout_bytes;
    let stderr_limit = limits.max_stderr_bytes;
    let stdout_reader = thread::spawn(move || read_bounded(stdout, stdout_limit));
    let stderr_reader = thread::spawn(move || read_bounded(stderr, stderr_limit));

    let started = Instant::now();
    let (status, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status, false),
            Ok(None) if started.elapsed() >= limits.timeout => {
                let status = terminate_and_wait(&mut child)
                    .map_err(|error| format!("Gagal menghentikan proses '{}': {}", spec.program, error))?;
                break (status, true);
            }
            Ok(None) => {
                let remaining = limits.timeout.saturating_sub(started.elapsed());
                thread::sleep(Duration::from_millis(10).min(remaining));
            }
            Err(error) => {
                let _ = terminate_and_wait(&mut child);
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err(format!("Gagal memantau proses '{}': {}", spec.program, error));
            }
        }
    };

    let (stdout, stdout_truncated) = stdout_reader
        .join()
        .map_err(|_| "Pembaca stdout proses berhenti secara tidak terduga.".to_owned())?
        .map_err(|error| format!("Gagal membaca stdout proses '{}': {}", spec.program, error))?;
    let (stderr, stderr_truncated) = stderr_reader
        .join()
        .map_err(|_| "Pembaca stderr proses berhenti secara tidak terduga.".to_owned())?
        .map_err(|error| format!("Gagal membaca stderr proses '{}': {}", spec.program, error))?;

    Ok(ProcessResult {
        status_code: status.code(),
        success: !timed_out && status.success(),
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
        timed_out,
        stdout_truncated,
        stderr_truncated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denied_process_capability_never_spawns() {
        let spec = ProcessSpec { program: "one-test-process-that-must-not-run".into(), args: vec![], working_directory: None };
        let error = run_process(&spec, &CapabilitySet::none()).expect_err("process must be denied");
        assert_eq!(error, "Kemampuan proses belum diberikan kepada program.");
    }

    #[test]
    fn allowed_process_preserves_exit_status_and_streams() {
        let (program, args) = if cfg!(windows) {
            ("cmd".to_owned(), vec!["/C".into(), "echo stdout & echo stderr 1>&2 & exit /B 7".into()])
        } else {
            ("sh".to_owned(), vec!["-c".into(), "printf stdout; printf stderr >&2; exit 7".into()])
        };
        let spec = ProcessSpec { program, args, working_directory: None };
        let result = run_process(&spec, &CapabilitySet::none().allow(Capability::Process)).expect("process capability permits execution");
        assert!(!result.success);
        assert_eq!(result.status_code, Some(7));
        assert!(result.stdout.contains("stdout"));
        assert!(result.stderr.contains("stderr"));
    }

    #[test]
    fn process_timeout_terminates_the_child_and_reports_timeout() {
        let (program, args) = if cfg!(windows) {
            ("ping".to_owned(), vec!["-n".into(), "10".into(), "127.0.0.1".into()])
        } else {
            ("sleep".to_owned(), vec!["5".into()])
        };
        let spec = ProcessSpec { program, args, working_directory: None };
        let limits = ProcessLimits {
            timeout: Duration::from_millis(200),
            max_stdout_bytes: 64,
            max_stderr_bytes: 64,
        };
        let result = run_process_with_limits(
            &spec,
            &CapabilitySet::none().allow(Capability::Process),
            &limits,
        ).expect("allowed process should return a timeout result");
        assert!(result.timed_out);
        assert!(!result.success);
    }

    #[test]
    fn process_output_is_bounded_and_truncation_is_reported() {
        let spec = ProcessSpec {
            program: "python".into(),
            args: vec![
                "-c".into(),
                "import sys; sys.stdout.write('x'*4096); sys.stderr.write('y'*4096)".into(),
            ],
            working_directory: None,
        };
        let limits = ProcessLimits {
            timeout: Duration::from_secs(10),
            max_stdout_bytes: 32,
            max_stderr_bytes: 24,
        };
        let result = run_process_with_limits(
            &spec,
            &CapabilitySet::none().allow(Capability::Process),
            &limits,
        ).expect("allowed process should complete");
        assert!(result.stdout.len() <= 32);
        assert!(result.stderr.len() <= 24);
        assert!(result.stdout_truncated);
        assert!(result.stderr_truncated);
    }
}
