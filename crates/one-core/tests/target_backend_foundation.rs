use one_core::{
    ExecutionProfile, TargetArchitecture, TargetBackend, TargetEnvironment, TargetSpec,
};

#[test]
fn hosted_target_is_accepted_by_declared_backend() {
    let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux);
    let backend = TargetBackend::reference();

    backend.validate(&target).expect("reference backend should accept hosted x86_64 Linux");
}

#[test]
fn constrained_and_firmware_profiles_are_representable_without_semantic_fork() {
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

    assert_eq!(constrained.profile(), ExecutionProfile::Constrained);
    assert_eq!(firmware.profile(), ExecutionProfile::Firmware);
}

#[test]
fn backend_identity_mismatch_is_rejected_deterministically() {
    let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
        .with_backend("other");
    let backend = TargetBackend::reference();

    let first = backend.validate(&target).expect_err("backend mismatch must be rejected");
    let second = backend.validate(&target).expect_err("same mismatch must fail identically");

    assert_eq!(first, second);
    assert!(first.to_string().contains("backend mismatch"));
}

#[test]
fn unsupported_architecture_is_rejected_deterministically() {
    let target = TargetSpec::new(
        TargetArchitecture::Other("example-arch".into()),
        ExecutionProfile::Hosted,
        TargetEnvironment::Linux,
    );
    let backend = TargetBackend::reference();

    let first = backend.validate(&target).expect_err("unsupported architecture must be rejected");
    let second = backend.validate(&target).expect_err("same target must fail identically");

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
    let backend = TargetBackend::reference();

    let error = backend.validate(&target).expect_err("reference backend must reject unsupported firmware profile");

    assert!(error.to_string().contains("profile"));
}

#[test]
fn unsupported_environment_is_rejected_deterministically() {
    let target = TargetSpec::new(
        TargetArchitecture::Aarch64,
        ExecutionProfile::Hosted,
        TargetEnvironment::BareMetal,
    );
    let backend = TargetBackend::reference();

    let error = backend.validate(&target).expect_err("reference backend must reject bare-metal environment");

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

    TargetBackend::reference().validate(&hosted).expect("hosted target should validate");
    let _ = TargetBackend::reference().validate(&constrained);

    assert_eq!(ir.canonical_text(), before);
}
