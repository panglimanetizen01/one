use crate::capability::Capability;
use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlatformKind {
    Any,
    Linux,
    Windows,
    Android,
    Macos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterMetadata {
    id: AdapterId,
    status: AdapterStatus,
    capabilities: Vec<Capability>,
    platforms: Vec<PlatformKind>,
}

impl AdapterMetadata {
    pub fn new(id: AdapterId, status: AdapterStatus, capabilities: Vec<Capability>, platforms: Vec<PlatformKind>) -> Self {
        Self { id, status, capabilities, platforms }
    }
    pub fn id(&self) -> &AdapterId { &self.id }
    pub const fn status(&self) -> AdapterStatus { self.status }
    pub fn declares(&self, capability: Capability) -> bool { self.capabilities.contains(&capability) }
    pub fn available_on(&self, platform: PlatformKind) -> bool { self.platforms.contains(&PlatformKind::Any) || self.platforms.contains(&platform) }
    pub fn validate_request(&self, request: &CapabilityRequest) -> Result<(), AdapterError> {
        request.validate().map_err(AdapterError::InvalidRequest)?;
        if !self.declares(request.capability) { return Err(AdapterError::UndeclaredCapability); }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterArgument {
    Text(String),
    Integer(i64),
    Boolean(bool),
    Bytes(Vec<u8>),
    TextList(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRequest {
    capability: CapabilityRequest,
    arguments: Vec<AdapterArgument>,
}

impl AdapterRequest {
    pub fn new(capability: CapabilityRequest) -> Self { Self { capability, arguments: Vec::new() } }
    pub fn with_argument(mut self, argument: AdapterArgument) -> Self { self.arguments.push(argument); self }
    pub const fn capability(&self) -> &CapabilityRequest { &self.capability }
    pub fn arguments(&self) -> &[AdapterArgument] { &self.arguments }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResult {
    pub success: bool,
    pub output: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    InvalidRequest(crate::orchestration::OrchestrationValidationError),
    UndeclaredCapability,
    InvalidArguments,
    UnsupportedPlatform,
}

pub trait ToolAdapter {
    fn metadata(&self) -> &AdapterMetadata;
    fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;
    use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId};

    fn request() -> CapabilityRequest { CapabilityRequest::new(SemanticCapabilityId::NetworkConnect, Capability::NetworkConnect, ExecutionScope::target("server.example"), EffectClass::Connect) }
    fn metadata() -> AdapterMetadata { AdapterMetadata::new(AdapterId::new("network.native"), AdapterStatus::Implemented, vec![Capability::NetworkConnect], vec![PlatformKind::Linux, PlatformKind::Windows]) }

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
        assert!(metadata.validate_request(&request()).is_ok());
        let undeclared = CapabilityRequest::new(SemanticCapabilityId::NetworkAdmin, Capability::NetworkAdmin, ExecutionScope::target("server.example"), EffectClass::Admin);
        assert_eq!(metadata.validate_request(&undeclared), Err(AdapterError::UndeclaredCapability));
    }

    #[test]
    fn adapter_request_contains_typed_arguments_not_command_text() {
        let request = AdapterRequest::new(request()).with_argument(AdapterArgument::Text("server.example".into()));
        assert_eq!(request.arguments(), &[AdapterArgument::Text("server.example".into())]);
    }
}
