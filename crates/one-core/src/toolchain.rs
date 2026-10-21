use std::{
    ffi::OsString,
    fmt, io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Output},
};

use crate::{
    native_backend::NativeArtifact,
    target::{TargetBackend, TargetSpec, ToolchainOperation, ToolchainSpec},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainCommand {
    executable: PathBuf,
    args: Vec<OsString>,
}

impl ToolchainCommand {
    pub fn new(executable: impl Into<PathBuf>, args: impl Into<Vec<OsString>>) -> Self {
        Self {
            executable: executable.into(),
            args: args.into(),
        }
    }

    pub fn for_build(
        executable: impl Into<PathBuf>,
        toolchain: &ToolchainSpec,
        source: &Path,
        artifact: &Path,
    ) -> Self {
        Self::new(
            executable,
            vec![
                OsString::from(format!("--target={}", toolchain.target().triple())),
                source.as_os_str().to_owned(),
                OsString::from("-o"),
                artifact.as_os_str().to_owned(),
            ],
        )
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
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

impl ToolchainInvocation {
    pub fn from_output(output: Output) -> Self {
        Self {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }

    pub fn status(&self) -> ExitStatus {
        self.status
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn success(&self) -> bool {
        self.status.success()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolchainExecutionError {
    ToolchainNotFound {
        executable: PathBuf,
    },
    ProcessStart {
        executable: PathBuf,
        message: String,
    },
    NonZeroExit {
        status: Option<i32>,
        stdout: String,
        stderr: String,
    },
    ArtifactMissing {
        path: PathBuf,
    },
    Validation {
        message: String,
    },
}

impl fmt::Display for ToolchainExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolchainNotFound { executable } => {
                write!(formatter, "toolchain not found: {}", executable.display())
            }
            Self::ProcessStart {
                executable,
                message,
            } => write!(
                formatter,
                "toolchain process start failed for {}: {message}",
                executable.display()
            ),
            Self::NonZeroExit {
                status,
                stdout,
                stderr,
            } => write!(
                formatter,
                "toolchain exited unsuccessfully ({status:?}); stdout={stdout:?}; stderr={stderr:?}"
            ),
            Self::ArtifactMissing { path } => {
                write!(formatter, "toolchain artifact missing: {}", path.display())
            }
            Self::Validation { message } => {
                write!(formatter, "toolchain validation failed: {message}")
            }
        }
    }
}

impl std::error::Error for ToolchainExecutionError {}

pub trait ToolchainResolver {
    fn resolve(
        &self,
        toolchain: &ToolchainSpec,
        source: &Path,
        artifact: &Path,
    ) -> Result<ToolchainCommand, ToolchainExecutionError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathToolchainResolver {
    executable: PathBuf,
}

impl PathToolchainResolver {
    pub fn new(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
        }
    }

    pub fn executable(&self) -> &Path {
        &self.executable
    }
}

impl ToolchainResolver for PathToolchainResolver {
    fn resolve(
        &self,
        toolchain: &ToolchainSpec,
        source: &Path,
        artifact: &Path,
    ) -> Result<ToolchainCommand, ToolchainExecutionError> {
        if !self.executable.is_file() {
            return Err(ToolchainExecutionError::ToolchainNotFound {
                executable: self.executable.clone(),
            });
        }

        Ok(ToolchainCommand::for_build(
            &self.executable,
            toolchain,
            source,
            artifact,
        ))
    }
}

pub trait ToolchainInvoker {
    fn invoke(
        &self,
        command: &ToolchainCommand,
    ) -> Result<ToolchainInvocation, ToolchainExecutionError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandToolchainInvoker;

impl ToolchainInvoker for CommandToolchainInvoker {
    fn invoke(
        &self,
        command: &ToolchainCommand,
    ) -> Result<ToolchainInvocation, ToolchainExecutionError> {
        let output = Command::new(command.executable())
            .args(command.args())
            .output()
            .map_err(|error| classify_start_error(command.executable(), error))?;
        let invocation = ToolchainInvocation::from_output(output);

        if !invocation.success() {
            return Err(ToolchainExecutionError::NonZeroExit {
                status: invocation.status().code(),
                stdout: invocation.stdout().to_owned(),
                stderr: invocation.stderr().to_owned(),
            });
        }

        Ok(invocation)
    }
}

fn classify_start_error(executable: &Path, error: io::Error) -> ToolchainExecutionError {
    if error.kind() == io::ErrorKind::NotFound {
        ToolchainExecutionError::ToolchainNotFound {
            executable: executable.to_path_buf(),
        }
    } else {
        ToolchainExecutionError::ProcessStart {
            executable: executable.to_path_buf(),
            message: error.to_string(),
        }
    }
}

fn remove_partial_artifact(path: &Path) {
    if path.is_file() {
        let _ = std::fs::remove_file(path);
    }
}

pub fn execute_toolchain_build<R, I>(
    target: &TargetSpec,
    backend: &impl TargetBackend,
    toolchain: &ToolchainSpec,
    resolver: &R,
    invoker: &I,
    source: &Path,
    artifact: &Path,
) -> Result<NativeArtifact, ToolchainExecutionError>
where
    R: ToolchainResolver,
    I: ToolchainInvoker,
{
    toolchain
        .validate_for(target, backend, ToolchainOperation::Build)
        .map_err(|error| ToolchainExecutionError::Validation {
            message: error.to_string(),
        })?;

    if artifact.exists() {
        return Err(ToolchainExecutionError::Validation {
            message: format!(
                "output path already exists; refusing to accept a stale artifact: {}",
                artifact.display()
            ),
        });
    }

    let command = resolver.resolve(toolchain, source, artifact)?;
    let invocation = match invoker.invoke(&command) {
        Ok(invocation) => invocation,
        Err(error) => {
            remove_partial_artifact(artifact);
            return Err(error);
        }
    };
    if !invocation.success() {
        let error = ToolchainExecutionError::NonZeroExit {
            status: invocation.status().code(),
            stdout: invocation.stdout().to_owned(),
            stderr: invocation.stderr().to_owned(),
        };
        remove_partial_artifact(artifact);
        return Err(error);
    }

    if !artifact.is_file() {
        return Err(ToolchainExecutionError::ArtifactMissing {
            path: artifact.to_path_buf(),
        });
    }

    Ok(NativeArtifact {
        path: artifact.to_path_buf(),
        format: toolchain.artifact().clone(),
    })
}
