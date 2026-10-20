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
    pub fn new(executable: PathBuf, args: Vec<OsString>) -> Self { Self { executable, args } }
    pub fn executable(&self) -> &Path { &self.executable }
    pub fn args(&self) -> &[OsString] { &self.args }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainInvocation {
    status: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl ToolchainInvocation {
    fn from_output(output: Output) -> Self {
        Self { status: output.status.code().unwrap_or(-1), stdout: output.stdout, stderr: output.stderr }
    }
    pub fn status(&self) -> i32 { self.status }
    pub fn stdout(&self) -> &[u8] { &self.stdout }
    pub fn stderr(&self) -> &[u8] { &self.stderr }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolchainExecutionError {
    ToolchainNotFound { executable: PathBuf },
    ProcessStart { executable: PathBuf, message: String },
    ProcessExited { status: i32, stdout: Vec<u8>, stderr: Vec<u8> },
    ArtifactMissing { artifact: PathBuf },
    Validation(ToolchainValidationError),
}

impl fmt::Display for ToolchainExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolchainNotFound { executable } => write!(f, "toolchain executable not found: {}", executable.display()),
            Self::ProcessStart { executable, message } => write!(f, "failed to start toolchain executable {}: {message}", executable.display()),
            Self::ProcessExited { status, .. } => write!(f, "toolchain process exited with status {status}"),
            Self::ArtifactMissing { artifact } => write!(f, "toolchain artifact missing: {}", artifact.display()),
            Self::Validation(error) => write!(f, "toolchain validation failed: {error}"),
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
    pub fn new(compiler: PathBuf, linker: Option<PathBuf>) -> Self { Self { compiler, linker } }
}

impl ToolchainResolver for PathToolchainResolver {
    fn resolve(&self, spec: &ToolchainSpec) -> Result<ToolchainCommand, ToolchainExecutionError> {
        if !self.compiler.is_file() {
            return Err(ToolchainExecutionError::ToolchainNotFound { executable: self.compiler.clone() });
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
    fn invoke(&self, command: &ToolchainCommand) -> Result<ToolchainInvocation, ToolchainExecutionError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CommandToolchainInvoker;
impl CommandToolchainInvoker { pub const fn new() -> Self { Self } }

impl ToolchainInvoker for CommandToolchainInvoker {
    fn invoke(&self, command: &ToolchainCommand) -> Result<ToolchainInvocation, ToolchainExecutionError> {
        let output = Command::new(command.executable()).args(command.args()).output()
            .map_err(|error| ToolchainExecutionError::ProcessStart {
                executable: command.executable().to_path_buf(), message: error.to_string()
            })?;
        let invocation = ToolchainInvocation::from_output(output);
        if invocation.status() != 0 {
            return Err(ToolchainExecutionError::ProcessExited {
                status: invocation.status(), stdout: invocation.stdout.clone(), stderr: invocation.stderr.clone()
            });
        }
        Ok(invocation)
    }
}

pub fn execute_toolchain_build<R, I>(
    resolver: &R,
    invoker: &I,
    spec: &ToolchainSpec,
    artifact: PathBuf,
) -> Result<ToolchainInvocation, ToolchainExecutionError>
where
    R: ToolchainResolver,
    I: ToolchainInvoker,
{
    let command = resolver.resolve(spec)?;
    let invocation = invoker.invoke(&command)?;
    if !artifact.is_file() {
        return Err(ToolchainExecutionError::ArtifactMissing { artifact });
    }
    Ok(invocation)
}
