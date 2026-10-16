#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;
    use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId};

    fn request() -> CapabilityRequest {
        CapabilityRequest::new(
            SemanticCapabilityId::NetworkConnect,
            Capability::NetworkConnect,
            ExecutionScope::target("server.example"),
            EffectClass::Connect,
        )
    }

    fn metadata() -> AdapterMetadata {
        AdapterMetadata::new(
            AdapterId::new("network.native"),
            AdapterStatus::Implemented,
            vec![Capability::NetworkConnect],
            vec![PlatformKind::Linux, PlatformKind::Windows],
        )
    }

    #[test]
    fn adapter_identity_and_capability_metadata_are_stable() {
        let metadata = metadata();
        assert_eq!(metadata.id().as_str(), "network.native");
        assert!(metadata.declares(Capability::NetworkConnect));
        assert!(!metadata.declares(Capability::NetworkAdmin));
    }

    #[test]
    fn adapter_platform_compatibility_is_explicit() {
        let metadata = metadata();
        assert!(metadata.available_on(PlatformKind::Linux));
        assert!(metadata.available_on(PlatformKind::Windows));
        assert!(!metadata.available_on(PlatformKind::Android));
    }

    #[test]
    fn adapter_request_requires_declared_capability() {
        let metadata = metadata();
        let request = request();
        assert!(metadata.validate_request(&request).is_ok());

        let undeclared = CapabilityRequest::new(
            SemanticCapabilityId::NetworkAdmin,
            Capability::NetworkAdmin,
            ExecutionScope::target("server.example"),
            EffectClass::Admin,
        );
        assert_eq!(metadata.validate_request(&undeclared), Err(AdapterError::UndeclaredCapability));
    }

    #[test]
    fn adapter_request_contains_typed_arguments_not_command_text() {
        let request = AdapterRequest::new(request()).with_argument(AdapterArgument::Text("server.example".into()));
        assert_eq!(request.arguments(), &[AdapterArgument::Text("server.example".into())]);
    }
}
