use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TargetArchitecture {
    X86_64,
    Aarch64,
    Arm,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionProfile {
    Hosted,
    Constrained,
    Firmware,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetEnvironment {
    Windows,
    Linux,
    Rtos,
    BareMetal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetSpec {
    architecture: TargetArchitecture,
    profile: ExecutionProfile,
    environment: TargetEnvironment,
    backend: String,
}

impl TargetSpec {
    pub fn new(
        architecture: TargetArchitecture,
        profile: ExecutionProfile,
        environment: TargetEnvironment,
    ) -> Self {
        Self {
            architecture,
            profile,
            environment,
            backend: "reference".into(),
        }
    }

    pub fn hosted(architecture: TargetArchitecture, environment: TargetEnvironment) -> Self {
        Self::new(architecture, ExecutionProfile::Hosted, environment)
    }

    pub fn with_backend(mut self, backend: impl Into<String>) -> Self {
        self.backend = backend.into();
        self
    }

    pub const fn architecture(&self) -> &TargetArchitecture {
        &self.architecture
    }

    pub const fn profile(&self) -> ExecutionProfile {
        self.profile
    }

    pub const fn environment(&self) -> TargetEnvironment {
        self.environment
    }

    pub fn backend(&self) -> &str {
        &self.backend
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetValidationError {
    BackendMismatch { expected: String, actual: String },
    UnsupportedArchitecture(TargetArchitecture),
    UnsupportedProfile(ExecutionProfile),
    UnsupportedEnvironment(TargetEnvironment),
    IncompatibleProfileEnvironment { profile: ExecutionProfile, environment: TargetEnvironment },
}

impl fmt::Display for TargetValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendMismatch { expected, actual } => {
                write!(formatter, "backend mismatch: expected {expected}, got {actual}")
            }
            Self::UnsupportedArchitecture(architecture) => {
                write!(formatter, "unsupported architecture: {architecture:?}")
            }
            Self::UnsupportedProfile(profile) => {
                write!(formatter, "unsupported execution profile: {profile:?}")
            }
            Self::UnsupportedEnvironment(environment) => {
                write!(formatter, "unsupported target environment: {environment:?}")
            }
            Self::IncompatibleProfileEnvironment { profile, environment } => {
                write!(formatter, "incompatible execution profile/environment: {profile:?}/{environment:?}")
            }
        }
    }
}

pub struct TargetBackend;

impl TargetBackend {
    pub const fn reference() -> Self {
        Self
    }

    pub const fn id(&self) -> &'static str {
        "reference"
    }

    pub fn validate(&self, target: &TargetSpec) -> Result<(), TargetValidationError> {
        if target.backend() != self.id() {
            return Err(TargetValidationError::BackendMismatch {
                expected: self.id().into(),
                actual: target.backend().into(),
            });
        }

        match target.architecture() {
            TargetArchitecture::X86_64 | TargetArchitecture::Aarch64 | TargetArchitecture::Arm => {}
            TargetArchitecture::Other(_) => {
                return Err(TargetValidationError::UnsupportedArchitecture(target.architecture().clone()));
            }
        }

        match target.profile() {
            ExecutionProfile::Hosted => {}
            ExecutionProfile::Constrained | ExecutionProfile::Firmware => {
                return Err(TargetValidationError::UnsupportedProfile(target.profile()));
            }
        }

        match target.environment() {
            TargetEnvironment::Windows | TargetEnvironment::Linux => {}
            TargetEnvironment::Rtos | TargetEnvironment::BareMetal => {
                return Err(TargetValidationError::UnsupportedEnvironment(target.environment()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_backend_identity_is_stable() {
        assert_eq!(TargetBackend::reference().id(), "reference");
    }

    #[test]
    fn backend_mismatch_is_deterministic() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
            .with_backend("other");
        let first = TargetBackend::reference().validate(&target);
        let second = TargetBackend::reference().validate(&target);
        assert_eq!(first, second);
        assert!(matches!(first, Err(TargetValidationError::BackendMismatch { .. })));
    }
}
