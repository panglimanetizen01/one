use one_core::{analyze, lower, parse, AdapterId, AdapterResult, Capability, CapabilityRequest, EffectClass, ExecutionPolicy, ExecutionScope, Interpreter, NormalizationExpectation, NormalizedValue, OrchestrationError, Orchestrator, PlatformKind, ScopeRule, SemanticCapabilityId};
use one_core::adapter::{AdapterError, AdapterMetadata, AdapterRequest, ProcessAdapter, ToolAdapter};
use one_core::governance::PolicyDecision;
use one_core::orchestration_audit::{AuditEvidence, AuditEvent};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("source should parse");
    let typed = analyze(&program).expect("source should analyze");
    lower(&typed).expect("source should lower").clone()
}

fn process_request(program: &str) -> CapabilityRequest {
    CapabilityRequest::new(
        SemanticCapabilityId::ProcessExecute,
        Capability::Process,
        ExecutionScope::target(program),
        EffectClass::Execute,
    )
}

#[test]
fn source_to_runtime_process_executes_with_explicit_capability_and_scope() {
    let ir = compile("jalankan proses \"rustc\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::Process)
            .allow_scope(Capability::Process, ScopeRule::Target("rustc".into())),
    );
    let result = interpreter.execute(&ir).expect("process should execute");
    assert!(result.output.iter().any(|line| line.contains("rustc")));
}

#[test]
fn source_process_is_denied_without_capability() {
    let ir = compile("jalankan proses \"rustc\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(ExecutionPolicy::default());
    let error = interpreter.execute(&ir).expect_err("process must be denied by default");
    assert_eq!(error.code, "N3209");
    assert!(error.message.contains("MissingCapability"));
}

#[test]
fn source_process_is_denied_when_target_is_out_of_scope() {
    let ir = compile("jalankan proses \"rustc\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::Process)
            .allow_scope(Capability::Process, ScopeRule::Target("other-program".into())),
    );
    let error = interpreter.execute(&ir).expect_err("out-of-scope process must be denied");
    assert_eq!(error.code, "N3209");
    assert!(error.message.contains("OutOfScope"));
}

#[test]
fn process_orchestrator_reports_unavailable_adapter_without_invocation() {
    let request = process_request("rustc");
    let policy = ExecutionPolicy::default()
        .grant(Capability::Process)
        .allow_scope(Capability::Process, ScopeRule::Target("rustc".into()));
    let orchestrator = Orchestrator::new(policy);
    let mut audit = Vec::new();
    let error = orchestrator.execute_with_audit(&request, PlatformKind::Any, &mut audit)
        .expect_err("no registered process adapter must be reported");
    assert_eq!(error, OrchestrationError::NoCompatibleAdapter);
    assert_eq!(audit[0].event(), AuditEvent::Denied);
    assert_eq!(audit[0].evidence(), AuditEvidence::DecisionOnly);
    assert!(!audit[0].attempted_execution());
}

#[test]
fn process_adapter_failure_is_observed_and_audited() {
    let request = process_request("one-process-vertical-slice-must-not-exist");
    let policy = ExecutionPolicy::default()
        .grant(Capability::Process)
        .allow_scope(Capability::Process, ScopeRule::Target("one-process-vertical-slice-must-not-exist".into()));
    let mut orchestrator = Orchestrator::new(policy);
    orchestrator.register(Box::new(ProcessAdapter::new()));
    let mut audit = Vec::new();
    let error = orchestrator.execute_with_audit(&request, PlatformKind::Any, &mut audit)
        .expect_err("missing executable must reach adapter and fail");
    assert_eq!(error, OrchestrationError::Adapter(AdapterError::BackendFailure));
    assert_eq!(audit[0].event(), AuditEvent::InvocationFinished);
    assert_eq!(audit[0].evidence(), AuditEvidence::ObservedFailure);
    assert!(audit[0].attempted_execution());
}

#[test]
fn process_adapter_output_normalization_is_deterministic() {
    let first = one_core::normalize_result(
        AdapterId::new("process.execute"),
        AdapterResult { success: true, output: b"rustc 1.x".to_vec() },
        NormalizationExpectation::Utf8Text,
    ).expect("valid UTF-8 should normalize");
    let second = one_core::normalize_result(
        AdapterId::new("process.execute"),
        AdapterResult { success: true, output: b"rustc 1.x".to_vec() },
        NormalizationExpectation::Utf8Text,
    ).expect("same output should normalize");
    assert_eq!(first, second);
    assert_eq!(first.value(), &NormalizedValue::Text("rustc 1.x".into()));
}

struct DeterministicProcessAdapter { metadata: AdapterMetadata }
impl ToolAdapter for DeterministicProcessAdapter {
    fn metadata(&self) -> &AdapterMetadata { &self.metadata }
    fn invoke(&self, _request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        Ok(AdapterResult { success: true, output: b"ok".to_vec() })
    }
}

#[test]
fn process_adapter_selection_is_deterministic_by_adapter_id() {
    let request = process_request("rustc");
    let policy = ExecutionPolicy::default()
        .grant(Capability::Process)
        .allow_scope(Capability::Process, ScopeRule::Target("rustc".into()));
    let mut orchestrator = Orchestrator::new(policy);
    for id in ["process.z", "process.a"] {
        orchestrator.register(Box::new(DeterministicProcessAdapter {
            metadata: AdapterMetadata::new(AdapterId::new(id), one_core::AdapterStatus::Implemented, vec![Capability::Process], vec![PlatformKind::Any]),
        }));
    }
    let mut audit = Vec::new();
    let (selected, _) = orchestrator.execute_with_audit(&request, PlatformKind::Any, &mut audit)
        .map(|result| (audit[0].adapter().cloned().expect("adapter id must be audited"), result))
        .expect("compatible process adapter should execute");
    assert_eq!(selected.as_str(), "process.a");
}

#[test]
fn process_policy_denial_remains_a_decision_without_execution() {
    let request = process_request("rustc");
    let orchestrator = Orchestrator::new(ExecutionPolicy::default());
    let mut audit = Vec::new();
    assert_eq!(
        orchestrator.execute_with_audit(&request, PlatformKind::Any, &mut audit),
        Err(OrchestrationError::PolicyDenied(PolicyDecision::Deny(one_core::DenyReason::MissingCapability)))
    );
    assert_eq!(audit[0].evidence(), AuditEvidence::DecisionOnly);
    assert!(!audit[0].attempted_execution());
}
