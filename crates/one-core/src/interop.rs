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
    Ok(ProcessResult {
        status_code: output.status.code(),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}
