use std::ffi::OsString;
use std::path::PathBuf;

use one_core::{
    ToolchainCommand, ToolchainExecutionError, ToolchainInvoker, ToolchainOperation,
    ToolchainResolver, ToolchainSpec,
};

#[test]
fn command_preserves_executable_and_arguments_without_shell_parsing() {
    let command = ToolchainCommand::new(
        PathBuf::from("/tmp/toolchain with spaces/compiler"),
        vec![
            OsString::from("--target=x86_64-unknown-linux-gnu"),
            OsString::from("input file.c"),
        ],
    );

    assert_eq!(
        command.executable(),
        PathBuf::from("/tmp/toolchain with spaces/compiler").as_path()
    );
    assert_eq!(
        command.args(),
        &[
            OsString::from("--target=x86_64-unknown-linux-gnu"),
            OsString::from("input file.c"),
        ]
    );
}

#[test]
fn missing_toolchain_executable_is_a_structured_resolution_failure() {
    let resolver = one_core::PathToolchainResolver::new(
        PathBuf::from("/definitely/missing/one-toolchain-compiler"),
        None,
    );
    let spec = reference_toolchain_spec();

    let error = resolver.resolve(&spec).expect_err("missing compiler must fail");

    assert!(matches!(
        error,
        ToolchainExecutionError::ToolchainNotFound { .. }
    ));
}

#[test]
fn non_zero_process_exit_is_distinguished_from_process_start_failure() {
    let invoker = one_core::CommandToolchainInvoker::new();
    let command = ToolchainCommand::new(
        PathBuf::from("/bin/sh"),
        vec![OsString::from("-c"), OsString::from("exit 7")],
    );

    let error = invoker.invoke(&command).expect_err("exit 7 must fail");

    assert!(matches!(
        error,
        ToolchainExecutionError::ProcessExited { status: 7, .. }
    ));
}

#[test]
fn target_mismatch_is_rejected_before_execution() {
    let spec = reference_toolchain_spec();
    let backend = one_core::ReferenceTargetBackend::new();
    let target = one_core::TargetSpec::hosted(
        one_core::TargetArchitecture::Aarch64,
        one_core::TargetEnvironment::Linux,
    );

    let error = spec
        .validate_for(&target, &backend, ToolchainOperation::Build)
        .expect_err("x86_64 toolchain cannot build an Aarch64 target");

    assert!(matches!(
        error,
        one_core::ToolchainValidationError::ArchitectureMismatch { .. }
    ));
}

#[test]
fn successful_toolchain_execution_requires_artifact() {
    let resolver = one_core::PathToolchainResolver::new(PathBuf::from("/bin/true"), None);
    let invoker = one_core::CommandToolchainInvoker::new();
    let spec = reference_toolchain_spec();
    let artifact = PathBuf::from("/tmp/one-test-missing-artifact");

    let error = one_core::execute_toolchain_build(&resolver, &invoker, &spec, artifact)
        .expect_err("successful process without artifact must fail");

    assert!(matches!(
        error,
        ToolchainExecutionError::ArtifactMissing { .. }
    ));
}

#[test]
fn successful_toolchain_execution_accepts_created_artifact() {
    let artifact = std::env::temp_dir().join(format!(
        "one-toolchain-artifact-{}",
        std::process::id()
    ));
    let command = one_core::ToolchainCommand::new(
        PathBuf::from("/bin/sh"),
        vec![
            OsString::from("-c"),
            OsString::from("printf artifact > \"$1\""),
            OsString::from("one-toolchain-test"),
            artifact.as_os_str().to_owned(),
        ],
    );
    let invoker = one_core::CommandToolchainInvoker::new();
    let invocation = invoker.invoke(&command).expect("fixture process must succeed");

    assert_eq!(invocation.status(), 0);
    assert!(artifact.is_file(), "fixture must create the artifact");
    std::fs::remove_file(&artifact).expect("fixture artifact cleanup must succeed");
}

fn reference_toolchain_spec() -> ToolchainSpec {
    one_core::ReferenceToolchain::new().spec()
}
