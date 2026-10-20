use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::target::{ToolchainSpec, ToolchainValidationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainCommand {
    executable: PathBuf,
    args: Vec<OsString>,
}

impl ToolchainCommand {
    pub fn new(executable: PathBuf, args: Vec<OsString>) -> Self {
        Self { executable, args }
    }

    pub fn executable(&self) -> &Path {
        &self.executable
    }

    pub fn args(&self) -> &[OsString] {
        &self.args
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainInvocation {
    status: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl ToolchainInvocation {
    fn from_output(output: Output) -> Self {
        Self {
            status: output.status.code().unwrap_or(-1),
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }

    pub fn status(&self) -> i32 {
        self.status
    }

    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    pub fn stderr(&self) -> &[u8] {
        &self.stderr
    }
}

#[derive(Debug)]
pub enum ToolchainExecutionError {
    ToolchainNotFound { executable: PathBuf },
    ProcessStart { executable: PathBuf, message: String },
    ProcessExited { status: i32, stdout: Vec<u8>, stderr: Vec<u8> },
    Validation(ToolchainValidationError),
}

impl PartialEq for ToolchainExecutionError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::ToolchainNotFound { executable: left },
                Self::ToolchainNotFound { executable: right },
            ) => left == right,
            (
                Self::ProcessStart {
                    executable: left_path,
                    message: left_message,
                },
                Self::ProcessStart {
                    executable: right_path,
                    message: right_message,
                },
            ) => left_path == right_path && left_message == right_message,
            (
                Self::ProcessExited {
                    status: left_status,
                    stdout: left_stdout,
                    stderr: left_stderr,
                },
                Self::ProcessExited {
                    status: right_status,
                    stdout: right_stdout,
                    stderr: right_stderr,
                },
            ) => {
                left_status == right_status
                    && left_stdout == right_stdout
                    && left_stderr == right_stderr
            }
            (Self::Validation(left), Self::Validation(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for ToolchainExecutionError {}

impl fmt::Display for ToolchainExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolchainNotFound { executable } => {
                write!(formatter, "toolchain executable not found: {}", executable.display())
            }
            Self::ProcessStart { executable, message } => write!(
                formatter,
                "failed to start toolchain executable {}: {message}",
                executable.display()
            ),
            Self::ProcessExited { status, .. } => {
                write!(formatter, "toolchain process exited with status {status}")
            }
            Self::Validation(error) => write!(formatter, "toolchain validation failed: {error}"),
        }
    }
}

impl std::error::Error for ToolchainExecutionError {}

pub trait ToolchainResolver {
    fn resolve(&self, spec: &ToolchainSpec) -> Result<ToolchainCommand, ToolchainExecutionError>;
}

#[derive(Debug, Clone)]
pub struct PathToolchainResolver {
    compiler: PathBuf,
    linker: Option<PathBuf>,
}

impl PathToolchainResolver {
    pub fn new(compiler: PathBuf, linker: Option<PathBuf>) -> Self {
        Self { compiler, linker }
    }
}

impl ToolchainResolver for PathToolchainResolver {
    fn resolve(&self, spec: &ToolchainSpec) -> Result<ToolchainCommand, ToolchainExecutionError> {
        if !self.compiler.is_file() {
            return Err(ToolchainExecutionError::ToolchainNotFound {
                executable: self.compiler.clone(),
            });
        }

        let mut args = vec![OsString::from(format!("--target={}", spec.target().triple()))];
        if let Some(linker) = &self.linker {
            args.push(OsString::from("-fuse-ld"));
            args.push(linker.as_os_str().to_owned());
        }

        Ok(ToolchainCommand::new(self.compiler.clone(), args))
    }
}

pub trait ToolchainInvoker {
    fn invoke(
        &self,
        command: &ToolchainCommand,
    ) -> Result<ToolchainInvocation, ToolchainExecutionError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CommandToolchainInvoker;

impl CommandToolchainInvoker {
    pub const fn new() -> Self {
        Self
    }
}

impl ToolchainInvoker for CommandToolchainInvoker {
    fn invoke(
        &self,
        command: &ToolchainCommand,
    ) -> Result<ToolchainInvocation, ToolchainExecutionError> {
        let output = Command::new(command.executable())
            .args(command.args())
            .output()
            .map_err(|error| ToolchainExecutionError::ProcessStart {
                executable: command.executable().to_path_buf(),
                message: error.to_string(),
            })?;

        let invocation = ToolchainInvocation::from_output(output);
        if invocation.status() != 0 {
            return Err(ToolchainExecutionError::ProcessExited {
                status: invocation.status(),
                stdout: invocation.stdout.clone(),
                stderr: invocation.stderr.clone(),
            });
        }

        Ok(invocation)
    }
}
