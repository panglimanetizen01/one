#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;

    #[test]
    fn orchestration_identifiers_are_stable_and_ordered() {
        assert_eq!(SemanticCapabilityId::NetworkConnect.as_str(), "network.connect");
        assert!(SemanticCapabilityId::NetworkConnect < SemanticCapabilityId::ProcessExecute);
        assert!(AdapterStatus::Verified > AdapterStatus::Implemented);
        assert!(AdapterStatus::Supported > AdapterStatus::Verified);
    }

    #[test]
    fn execution_scope_is_typed_and_deterministic() {
        let scope = ExecutionScope::target("server.example");
        assert_eq!(scope.as_str(), "target:server.example");
        assert_eq!(scope, ExecutionScope::target("server.example"));
        assert_ne!(scope, ExecutionScope::target("other.example"));
    }

    #[test]
    fn capability_request_validates_required_fields() {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::NetworkConnect,
            Capability::NetworkConnect,
            ExecutionScope::target("server.example"),
            EffectClass::Connect,
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn capability_request_rejects_incompatible_effect_and_capability() {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::NetworkConnect,
            Capability::NetworkConnect,
            ExecutionScope::target("server.example"),
            EffectClass::Admin,
        );

        assert_eq!(
            request.validate(),
            Err(OrchestrationValidationError::IncompatibleCapabilityEffect)
        );
    }
}
