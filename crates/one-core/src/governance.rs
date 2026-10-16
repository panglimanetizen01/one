#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;
    use crate::orchestration::{CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId};

    fn request(scope: ExecutionScope) -> CapabilityRequest {
        CapabilityRequest::new(
            SemanticCapabilityId::NetworkConnect,
            Capability::NetworkConnect,
            scope,
            EffectClass::Connect,
        )
    }

    #[test]
    fn missing_capability_is_denied() {
        let policy = ExecutionPolicy::default();
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::Deny(DenyReason::MissingCapability));
    }

    #[test]
    fn explicit_capability_grant_allows_matching_scope() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Target("server.example".into()));
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::Allow);
    }

    #[test]
    fn out_of_scope_target_is_denied() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Target("server.example".into()));
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("other.example"))), PolicyDecision::Deny(DenyReason::OutOfScope));
    }

    #[test]
    fn explicit_denial_overrides_grant() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .deny(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Any);
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::Deny(DenyReason::ExplicitlyDenied));
    }

    #[test]
    fn approval_requirement_is_deterministic() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .require_approval(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Any);
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::RequireApproval);
    }

    #[test]
    fn localized_wording_cannot_change_policy_for_same_request() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Any);
        let id_request = request(ExecutionScope::target("server.example"));
        let en_request = request(ExecutionScope::target("server.example"));
        assert_eq!(policy.evaluate(&id_request), policy.evaluate(&en_request));
    }
}
