use one_core::{
    normalize_result, parse, lower, analyze, AdapterId, AdapterResult, Capability, CapabilityRequest,
    EffectClass, ExecutionPolicy, ExecutionScope, FileReadAdapter, FileWriteAdapter, Interpreter,
    NormalizationExpectation, NormalizedValue, OrchestrationError, Orchestrator, PlatformKind,
    SemanticCapabilityId, ScopeRule, ToolAdapter,
};
use one_core::adapter::{AdapterError, AdapterMetadata, AdapterRequest};
use one_core::governance::PolicyDecision;
use one_core::orchestration_audit::{AuditEvidence, AuditEvent};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("source should parse");
    let typed = analyze(&program).expect("source should analyze");
    lower(&typed).expect("source should lower").clone()
}

fn filesystem_read_request(path: &str) -> CapabilityRequest {
    CapabilityRequest::new(
        SemanticCapabilityId::FilesystemRead,
        Capability::FilesystemRead,
        ExecutionScope::target(path),
        EffectClass::Read,
    )
}

#[test]
fn source_to_runtime_filesystem_read_succeeds_with_explicit_capability_and_scope() {
    let path = std::env::current_dir().unwrap().join("Cargo.toml");
    let path = path.to_string_lossy().replace('\\', "/");
    let ir = compile(&format!("baca berkas \"{path}\"\n"));
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::FilesystemRead)
            .allow_scope(Capability::FilesystemRead, ScopeRule::Target(path.clone())),
    );

    let result = interpreter.execute(&ir).expect("filesystem read should execute");

    assert!(!result.output.is_empty());
    assert!(result.output.iter().any(|line| line.contains("[package]")));
}

#[test]
fn source_filesystem_read_is_denied_without_capability() {
    let path = std::env::current_dir().unwrap().join("Cargo.toml");
    let path = path.to_string_lossy().replace('\\', "/");
    let ir = compile(&format!("baca berkas \"{path}\"\n"));
    let mut interpreter = Interpreter::with_orchestration_policy(ExecutionPolicy::default());

    let error = interpreter.execute(&ir).expect_err("filesystem read must be denied by default");

    assert_eq!(error.code, "N3209");
    assert!(error.message.contains("MissingCapability"));
}

#[test]
fn source_filesystem_read_is_denied_when_target_is_out_of_scope() {
    let allowed = std::env::current_dir().unwrap().join("Cargo.toml");
    let actual = std::env::current_dir().unwrap().join("README.md");
    let allowed = allowed.to_string_lossy().replace('\\', "/");
    let actual = actual.to_string_lossy().replace('\\', "/");
    let ir = compile(&format!("baca berkas \"{actual}\"\n"));
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::FilesystemRead)
            .allow_scope(Capability::FilesystemRead, ScopeRule::Target(allowed)),
    );

    let error = interpreter.execute(&ir).expect_err("out-of-scope target must be denied");

    assert_eq!(error.code, "N3209");
    assert!(error.message.contains("ScopeViolation"));
}

#[test]
fn filesystem_orchestrator_reports_unavailable_adapter_without_invocation() {
    let path = "Cargo.toml";
    let request = filesystem_read_request(path);
    let policy = ExecutionPolicy::default()
        .grant(Capability::FilesystemRead)
        .allow_scope(Capability::FilesystemRead, ScopeRule::Target(path.into()));
    let mut orchestrator = Orchestrator::new(policy);
    let mut audit = Vec::new();

    let error = orchestrator
        .execute_with_audit(&request, PlatformKind::Any, &mut audit)
        .expect_err("no registered adapter must be reported");

    assert_eq!(error, OrchestrationError::NoCompatibleAdapter);
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event(), AuditEvent::Denied);
    assert_eq!(audit[0].evidence(), AuditEvidence::DecisionOnly);
    assert!(!audit[0].attempted_execution());
}

#[test]
fn filesystem_orchestrator_reports_adapter_failure_and_records_observed_failure() {
    let path = "one-filesystem-vertical-slice-must-not-exist.one";
    let request = filesystem_read_request(path);
    let policy = ExecutionPolicy::default()
        .grant(Capability::FilesystemRead)
        .allow_scope(Capability::FilesystemRead, ScopeRule::Target(path.into()));
    let mut orchestrator = Orchestrator::new(policy);
    orchestrator.register(Box::new(FileReadAdapter::new()));
    let mut audit = Vec::new();

    let error = orchestrator
        .execute_with_audit(&request, PlatformKind::Any, &mut audit)
        .expect_err("missing file must reach the adapter and fail");

    assert_eq!(error, OrchestrationError::Adapter(AdapterError::BackendFailure));
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event(), AuditEvent::InvocationFinished);
    assert_eq!(audit[0].evidence(), AuditEvidence::ObservedFailure);
    assert!(audit[0].attempted_execution());
}

#[test]
fn filesystem_result_normalization_is_deterministic_and_typed() {
    let first = normalize_result(
        AdapterId::new("filesystem.file.read"),
        AdapterResult { success: true, output: b"halo\ndunia".to_vec() },
        NormalizationExpectation::Utf8Text,
    ).expect("UTF-8 output should normalize");
    let second = normalize_result(
        AdapterId::new("filesystem.file.read"),
        AdapterResult { success: true, output: b"halo\ndunia".to_vec() },
        NormalizationExpectation::Utf8Text,
    ).expect("same UTF-8 output should normalize");

    assert_eq!(first, second);
    assert_eq!(first.value(), &NormalizedValue::Text("halo\ndunia".into()));
}

#[test]
fn filesystem_write_adapter_declares_write_capability_only() {
    let adapter = FileWriteAdapter::new();
    assert!(adapter.metadata().declares(Capability::FilesystemWrite));
    assert!(!adapter.metadata().declares(Capability::FilesystemRead));
}

#[allow(dead_code)]
struct UnusedFailureAdapter {
    metadata: AdapterMetadata,
}

impl ToolAdapter for UnusedFailureAdapter {
    fn metadata(&self) -> &AdapterMetadata { &self.metadata }
    fn invoke(&self, _request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        Err(AdapterError::BackendFailure)
    }
}

#[test]
fn policy_decision_type_remains_stable_for_filesystem_denial() {
    assert_eq!(
        PolicyDecision::Deny(one_core::DenyReason::MissingCapability),
        PolicyDecision::Deny(one_core::DenyReason::MissingCapability),
    );
}
