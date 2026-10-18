use one_core::{
    ExecutionProfile, ReferenceTargetBackend, RuntimeClass, TargetArchitecture, TargetBackend,
    TargetEnvironment, TargetSpec,
};

#[test]
fn hosted_target_is_accepted_by_declared_backend() {
    let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
    let backend = ReferenceTargetBackend::new();

    backend
        .validate(&target)
        .expect("reference backend should accept hosted x86_64 Linux");
}

#[test]
fn backend_contract_declares_identity_and_required_runtime() {
    let backend = ReferenceTargetBackend::new();
    let contract: &dyn TargetBackend = &backend;

    assert_eq!(contract.id(), "reference");
    assert_eq!(contract.required_runtime(), RuntimeClass::Hosted);
}

#[test]
fn target_model_distinguishes_runtime_requirements() {
    let hosted = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
    let constrained = TargetSpec::new(
        TargetArchitecture::Aarch64,
        ExecutionProfile::Constrained,
        TargetEnvironment::Rtos,
    );
    let firmware = TargetSpec::new(
        TargetArchitecture::Arm,
        ExecutionProfile::Firmware,
        TargetEnvironment::BareMetal,
    );

    assert_eq!(hosted.runtime(), RuntimeClass::Hosted);
    assert_eq!(constrained.runtime(), RuntimeClass::Rtos);
    assert_eq!(firmware.runtime(), RuntimeClass::BareMetal);
    assert_eq!(constrained.profile(), ExecutionProfile::Constrained);
    assert_eq!(firmware.profile(), ExecutionProfile::Firmware);
}

#[test]
fn backend_identity_mismatch_is_rejected_deterministically() {
    let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
        .with_backend("other");
    let backend = ReferenceTargetBackend::new();

    let first = backend
        .validate(&target)
        .expect_err("backend mismatch must be rejected");
    let second = backend
        .validate(&target)
        .expect_err("same mismatch must fail identically");

    assert_eq!(first, second);
    assert!(first.to_string().contains("backend mismatch"));
}

#[test]
fn required_runtime_mismatch_is_rejected_deterministically() {
    struct RtosBackend;

    impl TargetBackend for RtosBackend {
        fn id(&self) -> &'static str {
            "rtos-test"
        }

        fn required_runtime(&self) -> RuntimeClass {
            RuntimeClass::Rtos
        }

        fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool {
            matches!(architecture, TargetArchitecture::Aarch64)
        }

        fn supports_profile(&self, profile: ExecutionProfile) -> bool {
            matches!(profile, ExecutionProfile::Constrained)
        }

        fn supports_environment(&self, environment: TargetEnvironment) -> bool {
            matches!(environment, TargetEnvironment::Rtos)
        }
    }

    let target = TargetSpec::new(
        TargetArchitecture::Aarch64,
        ExecutionProfile::Constrained,
        TargetEnvironment::Linux,
    )
    .with_backend("rtos-test");
    let backend = RtosBackend;

    let first = backend
        .validate(&target)
        .expect_err("runtime mismatch must be rejected");
    let second = backend
        .validate(&target)
        .expect_err("same runtime mismatch must fail identically");

    assert_eq!(first, second);
    assert!(first.to_string().contains("runtime"));
}

#[test]
fn unsupported_architecture_is_rejected_deterministically() {
    let target = TargetSpec::new(
        TargetArchitecture::Other("example-arch".into()),
        ExecutionProfile::Hosted,
        TargetEnvironment::Linux,
    );
    let backend = ReferenceTargetBackend::new();

    let first = backend
        .validate(&target)
        .expect_err("unsupported architecture must be rejected");
    let second = backend
        .validate(&target)
        .expect_err("same target must fail identically");

    assert_eq!(first, second);
    assert!(first.to_string().contains("architecture"));
}

#[test]
fn unsupported_execution_profile_is_rejected_deterministically() {
    let target = TargetSpec::new(
        TargetArchitecture::X86_64,
        ExecutionProfile::Firmware,
        TargetEnvironment::Linux,
    );
    let backend = ReferenceTargetBackend::new();

    let error = backend
        .validate(&target)
        .expect_err("reference backend must reject unsupported firmware profile");

    assert!(error.to_string().contains("profile"));
}

#[test]
fn unsupported_environment_is_rejected_deterministically() {
    let target = TargetSpec::new(
        TargetArchitecture::Aarch64,
        ExecutionProfile::Hosted,
        TargetEnvironment::BareMetal,
    );
    let backend = ReferenceTargetBackend::new();

    let error = backend
        .validate(&target)
        .expect_err("reference backend must reject bare-metal environment");

    assert!(error.to_string().contains("environment"));
}

#[test]
fn target_selection_does_not_mutate_semantic_ir() {
    let program = one_core::parse("nama adalah \"Budi\"\ntampilkan nama\n").expect("source should parse");
    let typed = one_core::analyze(&program).expect("source should analyze");
    let ir = one_core::lower(&typed).expect("source should lower");
    let before = ir.canonical_text();

    let hosted = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
    let constrained = TargetSpec::new(
        TargetArchitecture::Aarch64,
        ExecutionProfile::Constrained,
        TargetEnvironment::Rtos,
    );
    let backend = ReferenceTargetBackend::new();

    backend
        .validate(&hosted)
        .expect("hosted target should validate");
    let _ = backend.validate(&constrained);

    assert_eq!(ir.canonical_text(), before);
}
