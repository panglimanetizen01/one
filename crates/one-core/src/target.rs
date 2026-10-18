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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeClass {
    Hosted,
    Rtos,
    BareMetal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetSpec {
    architecture: TargetArchitecture,
    profile: ExecutionProfile,
    environment: TargetEnvironment,
    runtime: RuntimeClass,
    backend: String,
}

impl TargetSpec {
    pub fn new(
        architecture: TargetArchitecture,
        profile: ExecutionProfile,
        environment: TargetEnvironment,
    ) -> Self {
        let runtime = match environment {
            TargetEnvironment::Windows | TargetEnvironment::Linux => RuntimeClass::Hosted,
            TargetEnvironment::Rtos => RuntimeClass::Rtos,
            TargetEnvironment::BareMetal => RuntimeClass::BareMetal,
        };

        Self {
            architecture,
            profile,
            environment,
            runtime,
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

    pub const fn runtime(&self) -> RuntimeClass {
        self.runtime
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
    RuntimeMismatch { expected: RuntimeClass, actual: RuntimeClass },
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
            Self::RuntimeMismatch { expected, actual } => {
                write!(formatter, "runtime mismatch: expected {expected:?}, got {actual:?}")
            }
        }
    }
}

pub trait TargetBackend {
    fn id(&self) -> &'static str;

    fn required_runtime(&self) -> RuntimeClass;

    fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool;

    fn supports_profile(&self, profile: ExecutionProfile) -> bool;

    fn supports_environment(&self, environment: TargetEnvironment) -> bool;

    fn validate(&self, target: &TargetSpec) -> Result<(), TargetValidationError> {
        if target.backend() != self.id() {
            return Err(TargetValidationError::BackendMismatch {
                expected: self.id().into(),
                actual: target.backend().into(),
            });
        }

        if !self.supports_architecture(target.architecture()) {
            return Err(TargetValidationError::UnsupportedArchitecture(
                target.architecture().clone(),
            ));
        }

        if !self.supports_profile(target.profile()) {
            return Err(TargetValidationError::UnsupportedProfile(target.profile()));
        }

        if !self.supports_environment(target.environment()) {
            return Err(TargetValidationError::UnsupportedEnvironment(target.environment()));
        }

        if target.runtime() != self.required_runtime() {
            return Err(TargetValidationError::RuntimeMismatch {
                expected: self.required_runtime(),
                actual: target.runtime(),
            });
        }

        Ok(())
    }
}

pub struct ReferenceTargetBackend;

impl ReferenceTargetBackend {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ReferenceTargetBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetBackend for ReferenceTargetBackend {
    fn id(&self) -> &'static str {
        "reference"
    }

    fn required_runtime(&self) -> RuntimeClass {
        RuntimeClass::Hosted
    }

    fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool {
        matches!(
            architecture,
            TargetArchitecture::X86_64 | TargetArchitecture::Aarch64 | TargetArchitecture::Arm
        )
    }

    fn supports_profile(&self, profile: ExecutionProfile) -> bool {
        matches!(profile, ExecutionProfile::Hosted)
    }

    fn supports_environment(&self, environment: TargetEnvironment) -> bool {
        matches!(environment, TargetEnvironment::Windows | TargetEnvironment::Linux)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_backend_identity_and_runtime_are_stable() {
        let backend = ReferenceTargetBackend::new();
        assert_eq!(backend.id(), "reference");
        assert_eq!(backend.required_runtime(), RuntimeClass::Hosted);
    }

    #[test]
    fn backend_mismatch_is_deterministic() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
            .with_backend("other");
        let first = ReferenceTargetBackend::new().validate(&target);
        let second = ReferenceTargetBackend::new().validate(&target);
        assert_eq!(first, second);
        assert!(matches!(first, Err(TargetValidationError::BackendMismatch { .. })));
    }
}
