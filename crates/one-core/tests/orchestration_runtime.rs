use one_core::{
    AdapterError, AdapterId, AdapterMetadata, AdapterRequest, AdapterResult, AdapterStatus, Capability,
    CapabilityRequest, EffectClass, ExecutionPolicy, ExecutionScope, OrchestrationError, Orchestrator,
    PlatformKind, PolicyDecision, ScopeRule, SemanticCapabilityId, ToolAdapter,
};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

struct TestAdapter {
    metadata: AdapterMetadata,
    invocations: Arc<AtomicUsize>,
}

impl ToolAdapter for TestAdapter {
    fn metadata(&self) -> &AdapterMetadata { &self.metadata }

    fn invoke(&self, _request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        self.invocations.fetch_add(1, Ordering::SeqCst);
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

fn adapter(id: &str, invocations: Arc<AtomicUsize>) -> TestAdapter {
    TestAdapter {
        metadata: AdapterMetadata::new(
            AdapterId::new(id),
            AdapterStatus::Implemented,
            vec![Capability::NetworkConnect],
            vec![PlatformKind::Linux],
        ),
        invocations,
    }
}

fn policy() -> ExecutionPolicy {
    ExecutionPolicy::default()
        .grant(Capability::NetworkConnect)
        .allow_scope(Capability::NetworkConnect, ScopeRule::Target("server.example".into()))
}

#[test]
fn denied_request_never_reaches_adapter_invocation() {
    let invocations = Arc::new(AtomicUsize::new(0));
    let mut orchestrator = Orchestrator::new(ExecutionPolicy::default());
    orchestrator.register(Box::new(adapter("network.native", Arc::clone(&invocations))));

    assert_eq!(
        orchestrator.execute(&request(), PlatformKind::Linux),
        Err(OrchestrationError::PolicyDenied(PolicyDecision::Deny(one_core::DenyReason::MissingCapability)))
    );
    assert_eq!(invocations.load(Ordering::SeqCst), 0);
}

#[test]
fn approved_request_selects_compatible_adapter_and_invokes_once() {
    let invocations = Arc::new(AtomicUsize::new(0));
    let mut orchestrator = Orchestrator::new(policy());
    orchestrator.register(Box::new(adapter("network.native", Arc::clone(&invocations))));

    let result = orchestrator.execute(&request(), PlatformKind::Linux).expect("request should execute");
    assert!(result.success);
    assert_eq!(result.output, b"ok");
    assert_eq!(invocations.load(Ordering::SeqCst), 1);
}

#[test]
fn materially_incompatible_adapter_is_rejected_before_invocation() {
    let invocations = Arc::new(AtomicUsize::new(0));
    let mut orchestrator = Orchestrator::new(policy());
    orchestrator.register(Box::new(TestAdapter {
        metadata: AdapterMetadata::new(
            AdapterId::new("network.admin-only"),
            AdapterStatus::Implemented,
            vec![Capability::NetworkAdmin],
            vec![PlatformKind::Linux],
        ),
        invocations: Arc::clone(&invocations),
    }));

    assert_eq!(orchestrator.execute(&request(), PlatformKind::Linux), Err(OrchestrationError::NoCompatibleAdapter));
    assert_eq!(invocations.load(Ordering::SeqCst), 0);
}
