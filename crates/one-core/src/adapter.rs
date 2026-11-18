use std::time::Duration;
use std::sync::Arc;
use crate::filesystem::{FilesystemAuthority, FilesystemError};

use crate::capability::{Capability, CapabilitySet};
use crate::interop::{run_process, ProcessSpec};
use crate::network_runtime::{execute_http11, HttpRequest, RealNetworkRuntime};
use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, ExecutionScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlatformKind { Any, Linux, Windows, Android, Macos }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterMetadata { id: AdapterId, status: AdapterStatus, capabilities: Vec<Capability>, platforms: Vec<PlatformKind> }
impl AdapterMetadata { pub fn new(id: AdapterId, status: AdapterStatus, capabilities: Vec<Capability>, platforms: Vec<PlatformKind>) -> Self { Self { id, status, capabilities, platforms } } pub fn id(&self) -> &AdapterId { &self.id } pub const fn status(&self) -> AdapterStatus { self.status } pub fn declares(&self, capability: Capability) -> bool { self.capabilities.contains(&capability) } pub fn available_on(&self, platform: PlatformKind) -> bool { self.platforms.contains(&PlatformKind::Any) || self.platforms.contains(&platform) } pub fn validate_request(&self, request: &CapabilityRequest) -> Result<(), AdapterError> { request.validate().map_err(AdapterError::InvalidRequest)?; if !self.declares(request.capability) { return Err(AdapterError::UndeclaredCapability); } Ok(()) } }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterArgument { Text(String), Integer(i64), Boolean(bool), Bytes(Vec<u8>), TextList(Vec<String>) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRequest { capability: CapabilityRequest, arguments: Vec<AdapterArgument> }
impl AdapterRequest { pub fn new(capability: CapabilityRequest) -> Self { Self { capability, arguments: Vec::new() } } pub fn with_argument(mut self, argument: AdapterArgument) -> Self { self.arguments.push(argument); self } pub const fn capability(&self) -> &CapabilityRequest { &self.capability } pub fn arguments(&self) -> &[AdapterArgument] { &self.arguments } }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResult { pub success: bool, pub output: Vec<u8> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError { InvalidRequest(crate::orchestration::OrchestrationValidationError), UndeclaredCapability, InvalidArguments, UnsupportedPlatform, ResolutionFailed, BackendFailure }
pub trait ToolAdapter { fn metadata(&self) -> &AdapterMetadata; fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError>; }

pub struct FileReadAdapter { metadata: AdapterMetadata, authority: Option<Arc<FilesystemAuthority>> }
impl FileReadAdapter {
    pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("filesystem.file.read"), AdapterStatus::Implemented, vec![Capability::FilesystemRead], vec![PlatformKind::Any]), authority: None } }
    pub fn with_authority(authority: Arc<FilesystemAuthority>) -> Self { Self { authority: Some(authority), ..Self::new() } }
}
impl Default for FileReadAdapter { fn default() -> Self { Self::new() } }
impl ToolAdapter for FileReadAdapter {
    fn metadata(&self) -> &AdapterMetadata { &self.metadata }
    fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        self.metadata.validate_request(request.capability())?;
        if !request.arguments().is_empty() { return Err(AdapterError::InvalidArguments); }
        let path = match &request.capability().scope { ExecutionScope::Target(value) if !value.is_empty() => value, _ => return Err(AdapterError::InvalidRequest(crate::orchestration::OrchestrationValidationError::EmptyScope)) };
        let authority = self.authority.as_ref().ok_or(AdapterError::UnsupportedPlatform)?;
        authority.read(path).map(|output| AdapterResult { success: true, output }).map_err(map_filesystem_error)
    }
}

pub struct FileWriteAdapter { metadata: AdapterMetadata, authority: Option<Arc<FilesystemAuthority>> }
impl FileWriteAdapter {
    pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("filesystem.file.write"), AdapterStatus::Implemented, vec![Capability::FilesystemWrite], vec![PlatformKind::Any]), authority: None } }
    pub fn with_authority(authority: Arc<FilesystemAuthority>) -> Self { Self { authority: Some(authority), ..Self::new() } }
}
impl Default for FileWriteAdapter { fn default() -> Self { Self::new() } }
impl ToolAdapter for FileWriteAdapter {
    fn metadata(&self) -> &AdapterMetadata { &self.metadata }
    fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        self.metadata.validate_request(request.capability())?;
        let content = match request.arguments() { [AdapterArgument::Text(value)] => value.as_bytes().to_vec(), _ => return Err(AdapterError::InvalidArguments) };
        let path = match &request.capability().scope { ExecutionScope::Target(value) if !value.is_empty() => value, _ => return Err(AdapterError::InvalidRequest(crate::orchestration::OrchestrationValidationError::EmptyScope)) };
        let authority = self.authority.as_ref().ok_or(AdapterError::UnsupportedPlatform)?;
        authority.write(path, &content).map(|()| AdapterResult { success: true, output: content }).map_err(map_filesystem_error)
    }
}

fn map_filesystem_error(error: FilesystemError) -> AdapterError {
    match error {
        FilesystemError::UnsupportedPlatform => AdapterError::UnsupportedPlatform,
        FilesystemError::InvalidTarget | FilesystemError::OutsideRoot => AdapterError::ResolutionFailed,
        _ => AdapterError::BackendFailure,
    }
}

pub struct ProcessAdapter { metadata: AdapterMetadata }
impl ProcessAdapter { pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("process.execute"), AdapterStatus::Implemented, vec![Capability::Process], vec![PlatformKind::Any]) } } }
impl Default for ProcessAdapter { fn default() -> Self { Self::new() } }
impl ToolAdapter for ProcessAdapter { fn metadata(&self) -> &AdapterMetadata { &self.metadata } fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> { self.metadata.validate_request(request.capability())?; if !request.arguments().is_empty() { return Err(AdapterError::InvalidArguments); } let program = match &request.capability().scope { ExecutionScope::Target(value) if !value.is_empty() => value.clone(), _ => return Err(AdapterError::InvalidRequest(crate::orchestration::OrchestrationValidationError::EmptyScope)) }; let spec = ProcessSpec { program, args: Vec::new(), working_directory: None }; let result = run_process(&spec, &CapabilitySet::none().allow(Capability::Process)).map_err(|_| AdapterError::BackendFailure)?; let output = if result.stdout.is_empty() { result.stderr } else { result.stdout }; Ok(AdapterResult { success: result.success, output: output.into_bytes() }) } }

pub struct HttpGetAdapter { metadata: AdapterMetadata }
impl HttpGetAdapter { pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("network.http.get"), AdapterStatus::Implemented, vec![Capability::NetworkConnect], vec![PlatformKind::Any]) } } }
impl Default for HttpGetAdapter { fn default() -> Self { Self::new() } }
impl ToolAdapter for HttpGetAdapter {
    fn metadata(&self) -> &AdapterMetadata { &self.metadata }
    fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        self.metadata.validate_request(request.capability())?;
        if !request.arguments().is_empty() { return Err(AdapterError::InvalidArguments); }
        let target = match &request.capability().scope { ExecutionScope::Target(value) if !value.is_empty() => value, _ => return Err(AdapterError::InvalidRequest(crate::orchestration::OrchestrationValidationError::EmptyScope)) };
        let (authority, path) = parse_http_target(target)?;
        let mut runtime = RealNetworkRuntime::new();
        let response = execute_http11(&mut runtime, HttpRequest::get(authority, path), &CapabilitySet::none().allow(Capability::NetworkConnect), Duration::from_secs(5)).map_err(|_| AdapterError::BackendFailure)?;
        if !(200..300).contains(&response.status) { return Err(AdapterError::BackendFailure); }
        Ok(AdapterResult { success: true, output: response.body })
    }
}
fn parse_http_target(target: &str) -> Result<(String, String), AdapterError> {
    let rest = target.strip_prefix("http://").ok_or(AdapterError::InvalidArguments)?;
    if rest.contains('#') || rest.contains('\r') || rest.contains('\n') || rest.contains('@') { return Err(AdapterError::InvalidArguments); }
    let authority_end = rest.find('/').unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    if authority.is_empty() { return Err(AdapterError::InvalidArguments); }
    if let Some((host, port)) = authority.rsplit_once(':') && (host.is_empty() || port.is_empty() || port.parse::<u16>().is_err()) { return Err(AdapterError::InvalidArguments); }
    let path = if authority_end == rest.len() { "/" } else { &rest[authority_end..] };
    Ok((authority.to_owned(), path.to_owned()))
}

#[cfg(test)]
mod tests { use super::*; use crate::capability::Capability; use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId}; fn request() -> CapabilityRequest { CapabilityRequest::new(SemanticCapabilityId::NetworkConnect, Capability::NetworkConnect, ExecutionScope::target("server.example"), EffectClass::Connect) } fn metadata() -> AdapterMetadata { AdapterMetadata::new(AdapterId::new("network.native"), AdapterStatus::Implemented, vec![Capability::NetworkConnect], vec![PlatformKind::Linux, PlatformKind::Windows]) } #[test] fn adapter_identity_and_capability_metadata_are_stable() { let metadata = metadata(); assert_eq!(metadata.id().as_str(), "network.native"); assert!(metadata.declares(Capability::NetworkConnect)); assert!(!metadata.declares(Capability::NetworkAdmin)); } #[test] fn adapter_platform_compatibility_is_explicit() { let metadata = metadata(); assert!(metadata.available_on(PlatformKind::Linux)); assert!(metadata.available_on(PlatformKind::Windows)); assert!(!metadata.available_on(PlatformKind::Android)); } #[test] fn adapter_request_requires_declared_capability() { let metadata = metadata(); assert!(metadata.validate_request(&request()).is_ok()); let undeclared = CapabilityRequest::new(SemanticCapabilityId::NetworkAdmin, Capability::NetworkAdmin, ExecutionScope::target("server.example"), EffectClass::Admin); assert_eq!(metadata.validate_request(&undeclared), Err(AdapterError::UndeclaredCapability)); } #[test] fn adapter_request_contains_typed_arguments_not_command_text() { let request = AdapterRequest::new(request()).with_argument(AdapterArgument::Text("server.example".into())); assert_eq!(request.arguments(), &[AdapterArgument::Text("server.example".into())]); } #[test] fn adapter_errors_distinguish_backend_resolution_failure_from_platform_unavailability() { assert_ne!(AdapterError::ResolutionFailed, AdapterError::UnsupportedPlatform); } #[test] fn filesystem_read_adapter_fails_closed_without_host_authority() { let adapter = FileReadAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemRead, Capability::FilesystemRead, ExecutionScope::target("Cargo.toml"), EffectClass::Read); assert_eq!(adapter.invoke(&AdapterRequest::new(request)), Err(AdapterError::UnsupportedPlatform)); } #[cfg(target_os = "linux")] #[test] fn filesystem_read_adapter_maps_missing_target_to_backend_failure() { let authority = Arc::new(FilesystemAuthority::open(std::env::temp_dir(), 1024, 1024).unwrap()); let adapter = FileReadAdapter::with_authority(authority); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemRead, Capability::FilesystemRead, ExecutionScope::target("this-file-must-not-exist.one"), EffectClass::Read); assert_eq!(adapter.invoke(&AdapterRequest::new(request)), Err(AdapterError::BackendFailure)); } #[test] fn filesystem_write_adapter_fails_closed_without_host_authority() { let adapter = FileWriteAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemWrite, Capability::FilesystemWrite, ExecutionScope::target("one-filesystem-write-adapter-test.txt"), EffectClass::Write); assert_eq!(adapter.invoke(&AdapterRequest::new(request).with_argument(AdapterArgument::Text("adapter write".into()))), Err(AdapterError::UnsupportedPlatform)); } #[test] fn process_adapter_executes_scoped_program_and_returns_output() { let adapter = ProcessAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::ProcessExecute, Capability::Process, ExecutionScope::target("rustc"), EffectClass::Execute); let result = adapter.invoke(&AdapterRequest::new(request)).expect("rustc should be executable in the verification environment"); assert!(result.success); assert!(!result.output.is_empty()); } #[test] fn http_get_adapter_rejects_non_http_target() { let adapter = HttpGetAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::NetworkConnect, Capability::NetworkConnect, ExecutionScope::target("https://example.com/"), EffectClass::Connect); assert_eq!(adapter.invoke(&AdapterRequest::new(request)), Err(AdapterError::InvalidArguments)); } }
