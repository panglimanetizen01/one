use one_core::{
    ArtifactFormat, ExecutionProfile, LinkerStrategy, ReferenceTargetBackend, ReferenceToolchain,
    RuntimeClass, TargetArchitecture, TargetBackend, TargetEnvironment, TargetSpec,
    ToolchainCapability, ToolchainOperation, ToolchainSpec, ToolchainTarget, ToolchainValidationError,
};

fn hosted_target() -> TargetSpec {
    TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
}

fn reference_toolchain() -> ToolchainSpec {
    ReferenceToolchain::new().spec()
}

struct RuntimeProbeBackend;

impl TargetBackend for RuntimeProbeBackend {
    fn id(&self) -> &'static str {
        "runtime-probe"
    }

    fn required_runtime(&self) -> RuntimeClass {
        RuntimeClass::Hosted
    }

    fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool {
        matches!(architecture, TargetArchitecture::Aarch64)
    }

    fn supports_profile(&self, profile: ExecutionProfile) -> bool {
        matches!(profile, ExecutionProfile::Hosted)
    }

    fn supports_environment(&self, environment: TargetEnvironment) -> bool {
        matches!(environment, TargetEnvironment::Rtos)
    }
}

#[test]
fn compatible_reference_target_and_toolchain_accept_build() {
    let target = hosted_target();
    let backend = ReferenceTargetBackend::new();
    let toolchain = reference_toolchain();

    toolchain
        .validate_for(&target, &backend, ToolchainOperation::Build)
        .expect("reference toolchain should accept hosted build");
}

#[test]
fn compatible_reference_target_and_toolchain_accept_execute() {
    let target = hosted_target();
    let backend = ReferenceTargetBackend::new();
    let toolchain = reference_toolchain();

    toolchain
        .validate_for(&target, &backend, ToolchainOperation::Execute)
        .expect("reference toolchain should accept hosted execute");
}

#[test]
fn target_architecture_mismatch_is_deterministic() {
    let target = hosted_target();
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
        .expect_err("same architecture mismatch must fail identically");

    assert_eq!(first, second);
    assert!(matches!(
        first,
        ToolchainValidationError::ArchitectureMismatch { .. }
    ));
}

#[test]
fn target_environment_mismatch_is_deterministic() {
    let target = hosted_target();
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
fn backend_toolchain_mismatch_is_deterministic() {
    let target = hosted_target();
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

    let first = toolchain
        .validate_for(&target, &backend, ToolchainOperation::Build)
        .expect_err("toolchain backend mismatch must be rejected");
    let second = toolchain
        .validate_for(&target, &backend, ToolchainOperation::Build)
        .expect_err("same backend mismatch must fail identically");

    assert_eq!(first, second);
    assert!(matches!(
        first,
        ToolchainValidationError::BackendMismatch { .. }
    ));
}

#[test]
fn requested_operation_capability_mismatch_is_deterministic() {
    let target = hosted_target();
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

#[test]
fn repeated_validation_produces_equal_results() {
    let target = hosted_target();
    let backend = ReferenceTargetBackend::new();
    let toolchain = reference_toolchain();

    let first = toolchain.validate_for(&target, &backend, ToolchainOperation::Build);
    let second = toolchain.validate_for(&target, &backend, ToolchainOperation::Build);

    assert_eq!(first, second);
    assert!(first.is_ok());
}

#[test]
fn runtime_incompatibility_is_reported_by_existing_target_backend_contract() {
    let target = TargetSpec::new(
        TargetArchitecture::Aarch64,
        ExecutionProfile::Hosted,
        TargetEnvironment::Rtos,
    )
    .with_backend("runtime-probe");
    let backend = RuntimeProbeBackend;
    let toolchain = ToolchainSpec::new(
        "runtime-probe-toolchain",
        "runtime-probe",
        ToolchainTarget::new(
            "aarch64-unknown-none",
            TargetArchitecture::Aarch64,
            TargetEnvironment::Rtos,
        ),
        LinkerStrategy::PlatformDefault,
        ArtifactFormat::Executable,
        ToolchainCapability::Build,
    );

    let error = toolchain
        .validate_for(&target, &backend, ToolchainOperation::Build)
        .expect_err("existing backend runtime contract must reject incompatible target");

    assert!(matches!(
        error,
        ToolchainValidationError::TargetValidation(_)
    ));
    assert!(error.to_string().contains("runtime mismatch"));
}

#[test]
fn semantic_ir_remains_equivalent_after_toolchain_validation() {
    let program = one_core::parse("nama adalah \"Budi\"\ntampilkan nama\n").expect("source should parse");
    let typed = one_core::analyze(&program).expect("source should analyze");
    let ir = one_core::lower(&typed).expect("source should lower");
    let before = ir.clone();

    let target = hosted_target();
    let backend = ReferenceTargetBackend::new();
    reference_toolchain()
        .validate_for(&target, &backend, ToolchainOperation::Build)
        .expect("toolchain validation should not alter semantic IR");

    assert_eq!(ir, before);
}
