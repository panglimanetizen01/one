use std::{
    cell::Cell,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use one_core::{
    ArtifactFormat, CommandToolchainInvoker, LinkerStrategy, PathToolchainResolver,
    ReferenceTargetBackend, TargetArchitecture, TargetEnvironment, TargetSpec, ToolchainCapability,
    ToolchainCommand, ToolchainExecutionError, ToolchainInvocation, ToolchainInvoker,
    ToolchainResolver, ToolchainSpec, ToolchainTarget, execute_toolchain_build,
};

fn toolchain() -> ToolchainSpec {
    ToolchainSpec::new(
        "fixture-toolchain",
        "reference",
        ToolchainTarget::new(
            "x86_64-unknown-linux-gnu",
            TargetArchitecture::X86_64,
            TargetEnvironment::Linux,
        ),
        LinkerStrategy::Named("fixture".into()),
        ArtifactFormat::Executable,
        ToolchainCapability::Build,
    )
}

fn target() -> TargetSpec {
    TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
}

fn fixture_executable() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_toolchain-fixture"))
}

#[test]
fn command_preserves_executable_and_arguments_without_shell_parsing() {
    let executable = fixture_executable();
    let source = Path::new("source file; $(do-not-run)").to_path_buf();
    let artifact = std::env::temp_dir().join(format!(
        "one-gate1-artifact with spaces-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&artifact);
    let command = ToolchainCommand::for_build(&executable, &toolchain(), &source, &artifact);

    assert_eq!(command.executable(), executable.as_path());
    assert_eq!(
        command.args(),
        &[
            OsString::from("--target=x86_64-unknown-linux-gnu"),
            source.into_os_string(),
            OsString::from("-o"),
            artifact.as_os_str().to_owned(),
        ]
    );

    let invocation = CommandToolchainInvoker
        .invoke(&command)
        .expect("fixture invocation should succeed");
    assert!(invocation.stdout().contains("source file; $(do-not-run)"));
    assert!(
        invocation
            .stdout()
            .contains(&format!("output={}", artifact.display()))
    );
    assert!(artifact.is_file(), "fixture should create the requested artifact");
    fs::remove_file(&artifact).expect("temporary artifact should be removed");
}

#[test]
fn missing_executable_is_toolchain_not_found() {
    let missing = std::env::temp_dir().join("one-gate1-definitely-missing-toolchain.exe");
    let _ = fs::remove_file(&missing);
    let resolver = PathToolchainResolver::new(missing.clone());
    let error = resolver
        .resolve(&toolchain(), Path::new("source.one"), Path::new("artifact"))
        .and_then(|command| CommandToolchainInvoker.invoke(&command))
        .expect_err("missing executable must fail");

    assert!(matches!(
        error,
        ToolchainExecutionError::ToolchainNotFound { .. }
    ));

    let start_error = CommandToolchainInvoker
        .invoke(&ToolchainCommand::new(missing, Vec::<OsString>::new()))
        .expect_err("starting a missing executable must fail");
    assert!(matches!(
        start_error,
        ToolchainExecutionError::ToolchainNotFound { .. }
    ));
}

#[test]
fn process_start_failure_is_distinct() {
    let directory = fixture_executable()
        .parent()
        .expect("fixture must have a parent")
        .to_path_buf();
    let command = ToolchainCommand::new(directory, Vec::<OsString>::new());
    let error = CommandToolchainInvoker
        .invoke(&command)
        .expect_err("directory cannot be executed as a process");

    assert!(matches!(
        error,
        ToolchainExecutionError::ProcessStart { .. }
    ));
}

#[test]
fn non_zero_exit_is_distinct_from_start_failure() {
    let fixture = fixture_executable();
    let command = ToolchainCommand::new(fixture, vec![OsString::from("--exit=7")]);
    let error = CommandToolchainInvoker
        .invoke(&command)
        .expect_err("fixture requested non-zero exit");

    match error {
        ToolchainExecutionError::NonZeroExit {
            status,
            stdout,
            stderr,
        } => {
            assert_eq!(status, Some(7));
            assert!(stdout.is_empty());
            assert!(stderr.contains("fixture exit 7"));
        }
        other => panic!("expected non-zero exit, got {other:?}"),
    }
}

struct CountingResolver {
    calls: Cell<u32>,
    command: ToolchainCommand,
}

impl ToolchainResolver for CountingResolver {
    fn resolve(
        &self,
        _toolchain: &ToolchainSpec,
        _source: &Path,
        _artifact: &Path,
    ) -> Result<ToolchainCommand, ToolchainExecutionError> {
        self.calls.set(self.calls.get() + 1);
        Ok(self.command.clone())
    }
}

struct CountingInvoker {
    calls: Cell<u32>,
    result: Result<ToolchainInvocation, ToolchainExecutionError>,
}

impl ToolchainInvoker for CountingInvoker {
    fn invoke(
        &self,
        _command: &ToolchainCommand,
    ) -> Result<ToolchainInvocation, ToolchainExecutionError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

fn success_invocation() -> ToolchainInvocation {
    CommandToolchainInvoker
        .invoke(&ToolchainCommand::new(
            fixture_executable(),
            Vec::<OsString>::new(),
        ))
        .expect("fixture success invocation should succeed")
}

#[test]
fn target_mismatch_is_rejected_before_execution() {
    let target = TargetSpec::hosted(TargetArchitecture::Aarch64, TargetEnvironment::Linux);
    let resolver = CountingResolver {
        calls: Cell::new(0),
        command: ToolchainCommand::new(fixture_executable(), Vec::<OsString>::new()),
    };
    let invoker = CountingInvoker {
        calls: Cell::new(0),
        result: Ok(success_invocation()),
    };

    let error = execute_toolchain_build(
        &target,
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        Path::new("artifact"),
    )
    .expect_err("architecture mismatch must be rejected");

    match error {
        ToolchainExecutionError::Validation { message } => {
            assert!(message.contains("architecture mismatch"));
        }
        other => panic!("expected target validation failure, got {other:?}"),
    }
    assert_eq!(resolver.calls.get(), 0);
    assert_eq!(invoker.calls.get(), 0);
}

#[test]
fn successful_execution_without_artifact_is_failure() {
    let resolver = CountingResolver {
        calls: Cell::new(0),
        command: ToolchainCommand::new(fixture_executable(), Vec::<OsString>::new()),
    };
    let invoker = CountingInvoker {
        calls: Cell::new(0),
        result: Ok(success_invocation()),
    };

    let artifact = std::env::temp_dir().join(format!("one-gate1-missing-artifact-{}", std::process::id()));
    let _ = fs::remove_file(&artifact);
    let error = execute_toolchain_build(
        &target(),
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        &artifact,
    )
    .expect_err("successful process without artifact must fail");

    assert!(matches!(error, ToolchainExecutionError::ArtifactMissing { .. }));
    assert_eq!(resolver.calls.get(), 1);
    assert_eq!(invoker.calls.get(), 1);
}

#[test]
fn failed_toolchain_removes_partial_artifact() {
    let artifact = std::env::temp_dir().join(format!(
        "one-gate1-partial-artifact-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&artifact);
    let resolver = CountingResolver {
        calls: Cell::new(0),
        command: ToolchainCommand::new(
            fixture_executable(),
            vec![
                OsString::from("--exit=7"),
                OsString::from("source.one"),
                OsString::from("-o"),
                artifact.as_os_str().to_owned(),
            ],
        ),
    };
    let invoker = CommandToolchainInvoker;

    let error = execute_toolchain_build(
        &target(),
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        &artifact,
    )
    .expect_err("non-zero toolchain exit must fail the build");

    assert!(matches!(error, ToolchainExecutionError::NonZeroExit { .. }));
    assert!(!artifact.exists(), "partial output must not remain after failure");
    assert_eq!(resolver.calls.get(), 1);
}

#[test]
fn successful_execution_returns_artifact() {
    let resolver = PathToolchainResolver::new(fixture_executable());
    let invoker = CommandToolchainInvoker;
    let artifact = std::env::temp_dir().join(format!(
        "one-gate1-present-artifact-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&artifact);

    let result = execute_toolchain_build(
        &target(),
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        &artifact,
    )
    .expect("real toolchain boundary should create the artifact");

    assert_eq!(result.path, artifact);
    assert_eq!(result.format, ArtifactFormat::Executable);
    assert!(result.path.is_file());
    fs::remove_file(result.path).expect("artifact fixture should be removed");
}

#[test]
fn high_level_build_rejects_non_zero_status_from_custom_invoker() {
    let output = Command::new(fixture_executable())
        .arg("--exit=7")
        .output()
        .expect("fixture should start");
    let failed_invocation = ToolchainInvocation::from_output(output);
    let resolver = CountingResolver {
        calls: Cell::new(0),
        command: ToolchainCommand::new(fixture_executable(), Vec::<OsString>::new()),
    };
    let invoker = CountingInvoker {
        calls: Cell::new(0),
        result: Ok(failed_invocation),
    };
    let artifact = std::env::temp_dir().join(format!(
        "one-gate1-nonzero-artifact-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&artifact);

    let error = execute_toolchain_build(
        &target(),
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        &artifact,
    )
    .expect_err("high-level build must reject a non-zero invocation status");

    match error {
        ToolchainExecutionError::NonZeroExit {
            status,
            stdout,
            stderr,
        } => {
            assert_eq!(status, Some(7));
            assert!(stdout.is_empty());
            assert!(stderr.contains("fixture exit 7"));
        }
        other => panic!("expected non-zero exit classification, got {other:?}"),
    }
    assert_eq!(resolver.calls.get(), 1);
    assert_eq!(invoker.calls.get(), 1);
}

#[test]
fn high_level_build_rejects_preexisting_artifact_to_prevent_stale_success() {
    let resolver = CountingResolver {
        calls: Cell::new(0),
        command: ToolchainCommand::new(fixture_executable(), Vec::<OsString>::new()),
    };
    let invoker = CountingInvoker {
        calls: Cell::new(0),
        result: Ok(success_invocation()),
    };
    let artifact = std::env::temp_dir().join(format!(
        "one-gate1-stale-artifact-{}",
        std::process::id()
    ));
    fs::write(&artifact, b"stale artifact").expect("stale artifact fixture should be writable");

    let error = execute_toolchain_build(
        &target(),
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        &artifact,
    )
    .expect_err("pre-existing output must not be accepted as a fresh artifact");

    assert!(matches!(error, ToolchainExecutionError::Validation { .. }));
    assert_eq!(resolver.calls.get(), 0);
    assert_eq!(invoker.calls.get(), 0);
    assert_eq!(
        fs::read(&artifact).expect("stale artifact must remain untouched"),
        b"stale artifact"
    );
    fs::remove_file(artifact).expect("stale artifact fixture should be removed");
}

#[test]
fn high_level_build_uses_real_resolver_invoker_and_artifact_validation() {
    let resolver = PathToolchainResolver::new(fixture_executable());
    let invoker = CommandToolchainInvoker;
    let artifact = std::env::temp_dir().join(format!("one-gate1-end-to-end-artifact-{}", std::process::id()));
    let _ = fs::remove_file(&artifact);

    let result = execute_toolchain_build(
        &target(),
        &ReferenceTargetBackend::new(),
        &toolchain(),
        &resolver,
        &invoker,
        Path::new("source.one"),
        &artifact,
    )
    .expect("real boundary should produce the fixture artifact");

    assert_eq!(result.path, artifact);
    assert!(result.path.is_file());
    let _ = fs::remove_file(result.path);
}
