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
