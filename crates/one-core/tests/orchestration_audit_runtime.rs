use one_core::{
    AdapterError, AdapterId, AdapterMetadata, AdapterRequest, AdapterResult, AdapterStatus, AuditEvent,
    AuditRecord, Capability, CapabilityRequest, EffectClass, ExecutionPolicy, ExecutionScope, Orchestrator,
    PlatformKind, ScopeRule, SemanticCapabilityId, ToolAdapter,
};

struct TestAdapter;

impl ToolAdapter for TestAdapter {
    fn metadata(&self) -> &AdapterMetadata {
        static METADATA: std::sync::OnceLock<AdapterMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| AdapterMetadata::new(
            AdapterId::new("network.native"),
            AdapterStatus::Implemented,
            vec![Capability::NetworkConnect],
            vec![PlatformKind::Linux],
        ))
    }

    fn invoke(&self, _request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        Ok(AdapterResult { success: true, output: b"ok".to_vec() })
    }
}

fn request() -> CapabilityRequest {
    CapabilityRequest::new(
        SemanticCapabilityId::NetworkConnect,
        Capability::NetworkConnect,
        ExecutionScope::target("server.example"),
        EffectClass::Connect,
    )
}

fn policy() -> ExecutionPolicy {
    ExecutionPolicy::default()
        .grant(Capability::NetworkConnect)
        .allow_scope(Capability::NetworkConnect, ScopeRule::Target("server.example".into()))
}

#[test]
fn orchestrator_audit_distinguishes_denial_from_execution() {
    let mut orchestrator = Orchestrator::new(ExecutionPolicy::default());
    orchestrator.register(Box::new(TestAdapter));
    let mut audit = Vec::<AuditRecord>::new();
    assert!(orchestrator.execute_with_audit(&request(), PlatformKind::Linux, &mut audit).is_err());
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event(), AuditEvent::Denied);
    assert!(!audit[0].attempted_execution());
}

#[test]
fn orchestrator_audit_records_successful_invocation_as_observed() {
    let mut orchestrator = Orchestrator::new(policy());
    orchestrator.register(Box::new(TestAdapter));
    let mut audit = Vec::<AuditRecord>::new();
    orchestrator.execute_with_audit(&request(), PlatformKind::Linux, &mut audit).expect("request should execute");
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event(), AuditEvent::InvocationFinished);
    assert!(audit[0].attempted_execution());
}
