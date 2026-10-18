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
    RuntimeMismatch {
        expected: RuntimeClass,
        actual: RuntimeClass,
    },
    UnsupportedArchitecture(TargetArchitecture),
    UnsupportedProfile(ExecutionProfile),
    UnsupportedEnvironment(TargetEnvironment),
}

impl fmt::Display for TargetValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendMismatch { expected, actual } => {
                write!(formatter, "backend mismatch: expected {expected}, got {actual}")
            }
            Self::RuntimeMismatch { expected, actual } => {
                write!(formatter, "runtime mismatch: expected {expected:?}, got {actual:?}")
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainTarget {
    triple: String,
    architecture: TargetArchitecture,
    environment: TargetEnvironment,
}

impl ToolchainTarget {
    pub fn new(
        triple: impl Into<String>,
        architecture: TargetArchitecture,
        environment: TargetEnvironment,
    ) -> Self {
        Self {
            triple: triple.into(),
            architecture,
            environment,
        }
    }

    pub fn triple(&self) -> &str {
        &self.triple
    }

    pub const fn architecture(&self) -> &TargetArchitecture {
        &self.architecture
    }

    pub const fn environment(&self) -> TargetEnvironment {
        self.environment
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkerStrategy {
    PlatformDefault,
    Named(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactFormat {
    Executable,
    StaticLibrary,
    DynamicLibrary,
    Object,
    FirmwareImage,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainCapability {
    None,
    Build,
    Execute,
    BuildAndExecute,
}

impl ToolchainCapability {
    fn supports(self, operation: ToolchainOperation) -> bool {
        matches!(
            (self, operation),
            (Self::Build, ToolchainOperation::Build)
                | (Self::Execute, ToolchainOperation::Execute)
                | (Self::BuildAndExecute, ToolchainOperation::Build)
                | (Self::BuildAndExecute, ToolchainOperation::Execute)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainOperation {
    Build,
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainSpec {
    id: String,
    backend: String,
    target: ToolchainTarget,
    linker: LinkerStrategy,
    artifact: ArtifactFormat,
    capability: ToolchainCapability,
}

impl ToolchainSpec {
    pub fn new(
        id: impl Into<String>,
        backend: impl Into<String>,
        target: ToolchainTarget,
        linker: LinkerStrategy,
        artifact: ArtifactFormat,
        capability: ToolchainCapability,
    ) -> Self {
        Self {
            id: id.into(),
            backend: backend.into(),
            target,
            linker,
            artifact,
            capability,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn backend(&self) -> &str {
        &self.backend
    }

    pub const fn target(&self) -> &ToolchainTarget {
        &self.target
    }

    pub const fn linker(&self) -> &LinkerStrategy {
        &self.linker
    }

    pub const fn artifact(&self) -> &ArtifactFormat {
        &self.artifact
    }

    pub const fn capability(&self) -> ToolchainCapability {
        self.capability
    }

    pub fn validate_for<B: TargetBackend>(
        &self,
        target: &TargetSpec,
        backend: &B,
        operation: ToolchainOperation,
    ) -> Result<(), ToolchainValidationError> {
        backend
            .validate(target)
            .map_err(ToolchainValidationError::TargetValidation)?;

        if target.backend() != self.backend {
            return Err(ToolchainValidationError::BackendMismatch {
                expected: target.backend().into(),
                actual: self.backend.clone(),
            });
        }

        if target.architecture() != self.target.architecture() {
            return Err(ToolchainValidationError::ArchitectureMismatch {
                expected: target.architecture().clone(),
                actual: self.target.architecture().clone(),
            });
        }

        if target.environment() != self.target.environment() {
            return Err(ToolchainValidationError::EnvironmentMismatch {
                expected: target.environment(),
                actual: self.target.environment(),
            });
        }

        if !self.capability.supports(operation) {
            return Err(ToolchainValidationError::UnsupportedOperation {
                capability: self.capability,
                operation,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolchainValidationError {
    TargetValidation(TargetValidationError),
    BackendMismatch {
        expected: String,
        actual: String,
    },
    ArchitectureMismatch {
        expected: TargetArchitecture,
        actual: TargetArchitecture,
    },
    EnvironmentMismatch {
        expected: TargetEnvironment,
        actual: TargetEnvironment,
    },
    UnsupportedOperation {
        capability: ToolchainCapability,
        operation: ToolchainOperation,
    },
}

impl fmt::Display for ToolchainValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TargetValidation(error) => write!(formatter, "target validation failed: {error}"),
            Self::BackendMismatch { expected, actual } => {
                write!(formatter, "toolchain backend mismatch: expected {expected}, got {actual}")
            }
            Self::ArchitectureMismatch { expected, actual } => write!(
                formatter,
                "toolchain architecture mismatch: expected {expected:?}, got {actual:?}"
            ),
            Self::EnvironmentMismatch { expected, actual } => write!(
                formatter,
                "toolchain environment mismatch: expected {expected:?}, got {actual:?}"
            ),
            Self::UnsupportedOperation {
                capability,
                operation,
            } => write!(
                formatter,
                "toolchain capability mismatch: {capability:?} does not support {operation:?}"
            ),
        }
    }
}

pub struct ReferenceToolchain;

impl ReferenceToolchain {
    pub const fn new() -> Self {
        Self
    }

    pub fn spec(&self) -> ToolchainSpec {
        ToolchainSpec::new(
            "reference-hosted",
            "reference",
            ToolchainTarget::new(
                "x86_64-unknown-linux-gnu",
                TargetArchitecture::X86_64,
                TargetEnvironment::Linux,
            ),
            LinkerStrategy::PlatformDefault,
            ArtifactFormat::Executable,
            ToolchainCapability::BuildAndExecute,
        )
    }
}

impl Default for ReferenceToolchain {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn toolchain_model_has_stable_identity_and_target_metadata() {
        let first = ReferenceToolchain::new().spec();
        let second = ReferenceToolchain::new().spec();

        assert_eq!(first, second);
        assert_eq!(first.id(), "reference-hosted");
        assert_eq!(first.backend(), "reference");
        assert_eq!(first.target().triple(), "x86_64-unknown-linux-gnu");
        assert_eq!(first.target().architecture(), &TargetArchitecture::X86_64);
        assert_eq!(first.target().environment(), TargetEnvironment::Linux);
    }

    #[test]
    fn toolchain_model_exposes_linker_artifact_and_capability() {
        let spec = ReferenceToolchain::new().spec();

        assert_eq!(spec.linker(), &LinkerStrategy::PlatformDefault);
        assert_eq!(spec.artifact(), &ArtifactFormat::Executable);
        assert_eq!(spec.capability(), ToolchainCapability::BuildAndExecute);
    }

    #[test]
    fn reference_toolchain_accepts_hosted_build_and_execute() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
        let backend = ReferenceTargetBackend::new();
        let toolchain = ReferenceToolchain::new().spec();

        assert!(toolchain
            .validate_for(&target, &backend, ToolchainOperation::Build)
            .is_ok());
        assert!(toolchain
            .validate_for(&target, &backend, ToolchainOperation::Execute)
            .is_ok());
    }

    #[test]
    fn toolchain_validation_order_is_deterministic() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
            .with_backend("other");
        let backend = ReferenceTargetBackend::new();
        let toolchain = ReferenceToolchain::new().spec();

        let first = toolchain.validate_for(&target, &backend, ToolchainOperation::Build);
        let second = toolchain.validate_for(&target, &backend, ToolchainOperation::Build);

        assert_eq!(first, second);
        assert!(matches!(
            first,
            Err(ToolchainValidationError::TargetValidation(
                TargetValidationError::BackendMismatch { .. }
            ))
        ));
    }

    #[test]
    fn toolchain_backend_mismatch_is_rejected_after_target_validation() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
        let backend = ReferenceTargetBackend::new();
        let toolchain = ToolchainSpec::new(
            "other-hosted",
            "other",
            ToolchainTarget::new(
                "x86_64-unknown-linux-gnu",
                TargetArchitecture::X86_64,
                TargetEnvironment::Linux,
            ),
            LinkerStrategy::PlatformDefault,
            ArtifactFormat::Executable,
            ToolchainCapability::BuildAndExecute,
        );

        let error = toolchain
            .validate_for(&target, &backend, ToolchainOperation::Build)
            .expect_err("toolchain backend mismatch must be rejected");

        assert!(matches!(
            error,
            ToolchainValidationError::BackendMismatch { .. }
        ));
    }

    #[test]
    fn toolchain_architecture_mismatch_is_rejected_deterministically() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
        let backend = ReferenceTargetBackend::new();
        let toolchain = ToolchainSpec::new(
            "reference-aarch64",
            "reference",
            ToolchainTarget::new(
                "aarch64-unknown-linux-gnu",
                TargetArchitecture::Aarch64,
                TargetEnvironment::Linux,
            ),
            LinkerStrategy::PlatformDefault,
            ArtifactFormat::Executable,
            ToolchainCapability::BuildAndExecute,
        );

        let first = toolchain
            .validate_for(&target, &backend, ToolchainOperation::Build)
            .expect_err("architecture mismatch must be rejected");
        let second = toolchain
            .validate_for(&target, &backend, ToolchainOperation::Build)
            .expect_err("same mismatch must fail identically");

        assert_eq!(first, second);
        assert!(matches!(
            first,
            ToolchainValidationError::ArchitectureMismatch { .. }
        ));
    }

    #[test]
    fn toolchain_environment_mismatch_is_rejected_deterministically() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
        let backend = ReferenceTargetBackend::new();
        let toolchain = ToolchainSpec::new(
            "reference-windows",
            "reference",
            ToolchainTarget::new(
                "x86_64-pc-windows-msvc",
                TargetArchitecture::X86_64,
                TargetEnvironment::Windows,
            ),
            LinkerStrategy::PlatformDefault,
            ArtifactFormat::Executable,
            ToolchainCapability::BuildAndExecute,
        );

        let error = toolchain
            .validate_for(&target, &backend, ToolchainOperation::Build)
            .expect_err("environment mismatch must be rejected");

        assert!(matches!(
            error,
            ToolchainValidationError::EnvironmentMismatch { .. }
        ));
    }

    #[test]
    fn build_only_toolchain_rejects_execute_deterministically() {
        let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
        let backend = ReferenceTargetBackend::new();
        let toolchain = ToolchainSpec::new(
            "reference-build-only",
            "reference",
            ToolchainTarget::new(
                "x86_64-unknown-linux-gnu",
                TargetArchitecture::X86_64,
                TargetEnvironment::Linux,
            ),
            LinkerStrategy::PlatformDefault,
            ArtifactFormat::Executable,
            ToolchainCapability::Build,
        );

        let first = toolchain
            .validate_for(&target, &backend, ToolchainOperation::Execute)
            .expect_err("build-only toolchain must reject execute");
        let second = toolchain
            .validate_for(&target, &backend, ToolchainOperation::Execute)
            .expect_err("same capability mismatch must fail identically");

        assert_eq!(first, second);
        assert!(matches!(
            first,
            ToolchainValidationError::UnsupportedOperation { .. }
        ));
    }
}
