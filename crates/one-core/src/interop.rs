use std::process::{Command, Stdio};

use crate::capability::{Capability, CapabilitySet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSpec { pub program: String, pub args: Vec<String>, pub working_directory: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult { pub status_code: Option<i32>, pub success: bool, pub stdout: String, pub stderr: String }

pub fn run_process(spec: &ProcessSpec, capabilities: &CapabilitySet) -> Result<ProcessResult, String> {
    if !capabilities.permits(Capability::Process) { return Err("Kemampuan proses belum diberikan kepada program.".into()); }
    let mut command = Command::new(&spec.program);
    command.args(&spec.args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(dir) = &spec.working_directory { command.current_dir(dir); }
    let output = command.output().map_err(|error| format!("Gagal menjalankan proses '{}': {}", spec.program, error))?;
    Ok(ProcessResult { status_code: output.status.code(), success: output.status.success(), stdout: String::from_utf8_lossy(&output.stdout).into_owned(), stderr: String::from_utf8_lossy(&output.stderr).into_owned() })
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn process_timeout_terminates_the_child_and_reports_timeout() {
        use std::time::Duration;
        let (program, args) = if cfg!(windows) {
            ("cmd".to_owned(), vec!["/C".into(), "ping -n 10 127.0.0.1 >NUL".into()])
        } else {
            ("sh".to_owned(), vec!["-c".into(), "sleep 5".into()])
        };
        let spec = ProcessSpec { program, args, working_directory: None };
        let limits = ProcessLimits { timeout: Duration::from_millis(200), max_stdout_bytes: 64, max_stderr_bytes: 64 };
        let result = run_process_with_limits(&spec, &CapabilitySet::none().allow(Capability::Process), &limits)
            .expect("allowed process should return a timeout result");
        assert!(result.timed_out);
        assert!(!result.success);
    }

    #[test]
    fn process_output_is_bounded_and_truncation_is_reported() {
        let spec = ProcessSpec {
            program: "python".into(),
            args: vec!["-c".into(), "import sys; sys.stdout.write('x'*4096); sys.stderr.write('y'*4096)".into()],
            working_directory: None,
        };
        let limits = ProcessLimits { timeout: std::time::Duration::from_secs(10), max_stdout_bytes: 32, max_stderr_bytes: 24 };
        let result = run_process_with_limits(&spec, &CapabilitySet::none().allow(Capability::Process), &limits)
            .expect("allowed process should complete");
        assert!(result.stdout.len() <= 32);
        assert!(result.stderr.len() <= 24);
        assert!(result.stdout_truncated);
        assert!(result.stderr_truncated);
    }

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
}
