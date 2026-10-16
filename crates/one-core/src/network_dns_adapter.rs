#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        normalize_result, AdapterRequest, AuditRecord, Capability, CapabilityRequest, EffectClass,
        ExecutionPolicy, ExecutionScope, NormalizationExpectation, Orchestrator, PlatformKind,
        ScopeRule, SemanticCapabilityId,
    };

    fn request() -> CapabilityRequest {
        CapabilityRequest::new(
            SemanticCapabilityId::NetworkDiscover,
            Capability::NetworkDiscover,
            ExecutionScope::target("127.0.0.1"),
            EffectClass::Discover,
        )
    }

    #[test]
    fn dns_adapter_has_explicit_metadata_and_typed_request_boundary() {
        let adapter = DnsResolveAdapter::new();
        assert_eq!(adapter.metadata().id().as_str(), "network.dns.resolve");
        assert!(adapter.metadata().declares(Capability::NetworkDiscover));
        assert!(adapter.metadata().available_on(PlatformKind::Linux));
        assert!(adapter.metadata().available_on(PlatformKind::Windows));
        assert!(adapter.metadata().validate_request(&request()).is_ok());
        let request = AdapterRequest::new(request());
        assert!(request.arguments().is_empty());
    }

    #[test]
    fn real_dns_adapter_resolves_loopback_without_external_command_text() {
        let adapter = DnsResolveAdapter::new();
        let result = adapter.invoke(&AdapterRequest::new(request())).expect("loopback should resolve");
        assert!(result.success);
        assert!(!result.output.is_empty());
        assert!(std::str::from_utf8(&result.output).expect("text").contains("127.0.0.1"));
    }

    #[test]
    fn dns_adapter_end_to_end_uses_policy_normalization_and_audit() {
        let mut orchestrator = Orchestrator::new(
            ExecutionPolicy::default()
                .grant(Capability::NetworkDiscover)
                .allow_scope(Capability::NetworkDiscover, ScopeRule::Target("127.0.0.1".into())),
        );
        orchestrator.register(Box::new(DnsResolveAdapter::new()));
        let mut audit = Vec::<AuditRecord>::new();
        let result = orchestrator
            .execute_with_audit(&request(), PlatformKind::Linux, &mut audit)
            .expect("governed DNS request should execute");
        let normalized = normalize_result(
            crate::AdapterId::new("network.dns.resolve"),
            result,
            NormalizationExpectation::Utf8Text,
        ).expect("DNS output should normalize");
        assert!(matches!(normalized.value(), crate::NormalizedValue::Text(_)));
        assert_eq!(audit.len(), 1);
        assert!(audit[0].attempted_execution());
    }
}
