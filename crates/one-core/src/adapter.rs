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

fn filesystem_platforms() -> Vec<PlatformKind> {
    #[cfg(target_os = "linux")]
    {
        vec![PlatformKind::Linux]
    }
    #[cfg(windows)]
    {
        vec![PlatformKind::Windows]
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        Vec::new()
    }
}


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
    pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("filesystem.file.read"), AdapterStatus::Implemented, vec![Capability::FilesystemRead], filesystem_platforms()), authority: None } }
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
    pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("filesystem.file.write"), AdapterStatus::Implemented, vec![Capability::FilesystemWrite], filesystem_platforms()), authority: None } }
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
        FilesystemError::InvalidTarget | FilesystemError::OutsideRoot | FilesystemError::ResolutionFailed => AdapterError::ResolutionFailed,
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


#[cfg(all(test, target_os = "linux"))]
mod filesystem_nofollow_tests {
    use super::*;
    use crate::orchestration::{EffectClass, SemanticCapabilityId};
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("one-e4-{label}-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn read_request(path: &str) -> AdapterRequest {
        AdapterRequest::new(CapabilityRequest::new(
            SemanticCapabilityId::FilesystemRead,
            Capability::FilesystemRead,
            ExecutionScope::target(path),
            EffectClass::Read,
        ))
    }

    fn write_request(path: &str, content: &str) -> AdapterRequest {
        AdapterRequest::new(CapabilityRequest::new(
            SemanticCapabilityId::FilesystemWrite,
            Capability::FilesystemWrite,
            ExecutionScope::target(path),
            EffectClass::Write,
        )).with_argument(AdapterArgument::Text(content.to_owned()))
    }

    #[test]
    fn descriptor_relative_read_rejects_symlink_component_without_reading_outside() {
        let root = test_root("read-link");
        let outside = root.with_extension("outside");
        std::fs::write(&outside, b"OUTSIDE_SENTINEL").unwrap();
        symlink(&outside, root.join("escape")).unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileReadAdapter::with_authority(authority);

        let result = adapter.invoke(&read_request("escape"));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&outside).unwrap(), b"OUTSIDE_SENTINEL");
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_file(outside);
    }

    #[test]
    fn descriptor_relative_read_rejects_parent_traversal_without_reading_outside() {
        let root = test_root("read-parent");
        let outside = root.parent().unwrap().join(format!("one-e4-outside-{}", std::process::id()));
        std::fs::write(&outside, b"OUTSIDE_SENTINEL").unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileReadAdapter::with_authority(authority);

        let result = adapter.invoke(&read_request(&("../".to_owned() + outside.file_name().unwrap().to_str().unwrap())));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&outside).unwrap(), b"OUTSIDE_SENTINEL");
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_file(outside);
    }

    #[test]
    fn descriptor_relative_write_rejects_outside_target_without_side_effects() {
        let root = test_root("write-outside");
        let outside = root.with_extension("outside");
        let original = b"DO_NOT_MODIFY";
        std::fs::write(&outside, original).unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);

        let result = adapter.invoke(&write_request(outside.to_str().unwrap(), "ATTACK"));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&outside).unwrap(), original);
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 0);
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_file(outside);
    }

    #[test]
    fn descriptor_relative_write_creates_and_atomically_replaces_in_root() {
        let root = test_root("write-atomic");
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);

        let created = adapter.invoke(&write_request("created.txt", "FIRST"));
        assert!(created.as_ref().is_ok_and(|result| result.success));
        assert_eq!(std::fs::read(root.join("created.txt")).unwrap(), b"FIRST");

        let absolute_target = root.join("created.txt");
        let replaced = adapter.invoke(&write_request(absolute_target.to_str().unwrap(), "SECOND"));
        assert!(replaced.as_ref().is_ok_and(|result| result.success));
        assert_eq!(std::fs::read(&absolute_target).unwrap(), b"SECOND");
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);

        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn descriptor_relative_write_rejects_symlink_target_without_side_effects() {
        let root = test_root("write-symlink");
        let outside = root.with_extension("outside");
        std::fs::write(&outside, b"OUTSIDE_SENTINEL").unwrap();
        symlink(&outside, root.join("escape.txt")).unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);

        let result = adapter.invoke(&write_request("escape.txt", "ATTACK"));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&outside).unwrap(), b"OUTSIDE_SENTINEL");
        assert_eq!(std::fs::read_link(root.join("escape.txt")).unwrap(), outside);
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);

        drop(adapter);
        std::fs::remove_file(root.join("escape.txt")).unwrap();
        std::fs::remove_dir(root).unwrap();
        std::fs::remove_file(outside).unwrap();
    }

    #[test]
    fn descriptor_relative_write_rejects_oversized_content_without_side_effects() {
        let root = test_root("write-limit");
        let target = root.join("target.txt");
        std::fs::write(&target, b"KEEP_ORIGINAL").unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);

        let result = adapter.invoke(&write_request("target.txt", "TOO_LONG"));

        assert_eq!(result, Err(AdapterError::BackendFailure));
        assert_eq!(std::fs::read(&target).unwrap(), b"KEEP_ORIGINAL");
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);

        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn descriptor_relative_write_fails_closed_for_nested_parent() {
        let root = test_root("write-nested");
        let nested = root.join("nested");
        std::fs::create_dir(&nested).unwrap();
        std::fs::write(nested.join("target.txt"), b"KEEP_ORIGINAL").unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);

        let result = adapter.invoke(&write_request("nested/target.txt", "ATTACK"));

        assert_eq!(result, Err(AdapterError::UnsupportedPlatform));
        assert_eq!(std::fs::read(nested.join("target.txt")).unwrap(), b"KEEP_ORIGINAL");
        assert_eq!(std::fs::read_dir(&nested).unwrap().count(), 1);

        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn descriptor_relative_walk_rejects_parent_traversal_without_side_effects() {
        let root = test_root("walk-parent");
        let outside_dir = root.parent().unwrap().join(format!("one-e4-outside-dir-{}", std::process::id()));
        std::fs::create_dir_all(&outside_dir).unwrap();
        let sentinel = outside_dir.join("sentinel.txt");
        std::fs::write(&sentinel, b"UNCHANGED").unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileReadAdapter::with_authority(authority);

        let target = format!("../{}/sentinel.txt", outside_dir.file_name().unwrap().to_string_lossy());
        let result = adapter.invoke(&read_request(&target));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"UNCHANGED");
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 0);
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(outside_dir);
    }

    #[test]
    fn descriptor_relative_read_race_never_follows_swapped_parent_symlink() {
        let root = test_root("read-race");
        let parent = root.join("parent");
        let outside = root.with_extension("outside");
        std::fs::create_dir(&parent).unwrap();
        std::fs::create_dir(&outside).unwrap();
        std::fs::write(parent.join("sentinel.txt"), b"INSIDE_SENTINEL").unwrap();
        std::fs::write(outside.join("sentinel.txt"), b"OUTSIDE_SENTINEL").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = Arc::new(FileReadAdapter::with_authority(authority));
        let swap_root = root.clone();
        let swap_outside = outside.clone();
        let swapper = std::thread::spawn(move || {
            let parent = swap_root.join("parent");
            let held = swap_root.join("parent-held");
            let outside = swap_outside;
            for _ in 0..500 {
                if std::fs::rename(&parent, &held).is_err() { continue; }
                if symlink(&outside, &parent).is_ok() {
                    std::thread::yield_now();
                    let _ = std::fs::remove_file(&parent);
                }
                let _ = std::fs::rename(&held, &parent);
            }
        });

        for _ in 0..2000 {
            if let Ok(result) = adapter.invoke(&read_request("parent/sentinel.txt")) {
                assert_eq!(result.output, b"INSIDE_SENTINEL", "a swapped parent symlink exposed outside bytes");
            }
        }
        swapper.join().unwrap();

        assert_eq!(std::fs::read(outside.join("sentinel.txt")).unwrap(), b"OUTSIDE_SENTINEL");
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(outside);
    }

    #[test]
    fn descriptor_relative_read_rejects_mount_boundary_without_reading_outside() {
        use std::os::unix::fs::MetadataExt;

        let root_path = PathBuf::from("/dev");
        let mount_path = PathBuf::from("/dev/shm");
        let root_metadata = std::fs::metadata(&root_path).expect("/dev must exist");
        let mount_metadata = std::fs::metadata(&mount_path).expect("/dev/shm must exist");
        assert_ne!(
            root_metadata.dev(),
            mount_metadata.dev(),
            "/dev/shm must be a distinct mount for this regression"
        );

        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let file_name = format!("one-e4-read-mount-{}-{nonce}", std::process::id());
        let outside = mount_path.join(&file_name);
        std::fs::write(&outside, b"OUTSIDE_MOUNT_SENTINEL").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root_path, 4096, 4096).unwrap());
        let adapter = FileReadAdapter::with_authority(authority);
        let relative = format!("shm/{file_name}");
        let result = adapter.invoke(&read_request(&relative));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&outside).unwrap(), b"OUTSIDE_MOUNT_SENTINEL");

        drop(adapter);
        std::fs::remove_file(outside).unwrap();
    }

    #[test]
    fn final_component_mount_boundary_denial_does_not_truncate_file() {
        use std::ffi::CString;
        use std::fs::File;
        use std::os::fd::AsRawFd;
        use std::os::unix::ffi::OsStrExt;
        use std::os::unix::fs::MetadataExt;
        use std::path::Path;

        // /dev/shm is a separate tmpfs mount below /dev on the Ubuntu
        // verification runner. The final file already contains a sentinel.
        let root_path = Path::new("/dev");
        let mount_path = Path::new("/dev/shm");
        let root_metadata = std::fs::metadata(root_path).expect("/dev must exist");
        let mount_metadata = std::fs::metadata(mount_path).expect("/dev/shm must exist");
        assert_ne!(
            root_metadata.dev(),
            mount_metadata.dev(),
            "/dev/shm must be a distinct mount for this regression"
        );

        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let file_name = format!("one-e4-mount-boundary-{}-{nonce}", std::process::id());
        let sentinel = mount_path.join(&file_name);
        let relative = PathBuf::from("shm").join(&file_name);
        let original = b"DO_NOT_TRUNCATE_MOUNT_SENTINEL";
        std::fs::write(&sentinel, original).unwrap();

        // Exercise the kernel's exact final-component resolution boundary
        // with O_TRUNC. RESOLVE_NO_XDEV must reject the mount crossing before
        // the final file is opened or truncated.
        #[repr(C)]
        struct OpenHow {
            flags: u64,
            mode: u64,
            resolve: u64,
        }
        const RESOLVE_NO_XDEV: u64 = 0x01;
        const RESOLVE_NO_SYMLINKS: u64 = 0x04;
        const RESOLVE_BENEATH: u64 = 0x08;

        let root = File::open(root_path).unwrap();
        let c_path = CString::new(relative.as_os_str().as_bytes()).unwrap();
        let how = OpenHow {
            flags: (libc::O_WRONLY | libc::O_TRUNC | libc::O_CLOEXEC) as u64,
            mode: 0,
            resolve: RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV,
        };
        let fd = unsafe {
            libc::syscall(
                libc::SYS_openat2,
                root.as_raw_fd(),
                c_path.as_ptr(),
                &how as *const OpenHow,
                std::mem::size_of::<OpenHow>(),
            ) as libc::c_int
        };
        if fd >= 0 {
            unsafe { libc::close(fd); }
            let _ = std::fs::remove_file(&sentinel);
            panic!("openat2 crossed /dev/shm with RESOLVE_NO_XDEV and O_TRUNC");
        }
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::EXDEV),
            "mount-boundary resolution must fail with EXDEV"
        );

        // Nested Linux writes remain fail-closed until the implementation
        // can prevent a retained subdirectory handle from being moved outside
        // the trusted root. The target must remain unchanged.
        let authority = Arc::new(FilesystemAuthority::open(root_path, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);
        let result = adapter.invoke(&write_request(relative.to_str().unwrap(), "ATTACK"));
        assert_eq!(result, Err(AdapterError::UnsupportedPlatform));
        assert_eq!(std::fs::read(&sentinel).unwrap(), original);
        assert_eq!(
            std::fs::metadata(&sentinel).unwrap().len(),
            original.len() as u64
        );
        std::fs::remove_file(sentinel).unwrap();
    }

}


#[cfg(all(test, target_os = "windows"))]
mod filesystem_windows_tests {
    use super::*;
    use crate::orchestration::{EffectClass, SemanticCapabilityId};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!(
            "one-e4-win-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn read_request(path: &str) -> AdapterRequest {
        AdapterRequest::new(CapabilityRequest::new(
            SemanticCapabilityId::FilesystemRead,
            Capability::FilesystemRead,
            ExecutionScope::target(path),
            EffectClass::Read,
        ))
    }

    fn write_request(path: &str, content: &str) -> AdapterRequest {
        AdapterRequest::new(CapabilityRequest::new(
            SemanticCapabilityId::FilesystemWrite,
            Capability::FilesystemWrite,
            ExecutionScope::target(path),
            EffectClass::Write,
        ))
        .with_argument(AdapterArgument::Text(content.to_owned()))
    }

    fn create_junction(link: &Path, target: &Path) {
        use std::os::windows::process::CommandExt;

        // Pass the complete command line verbatim to cmd.exe. Normal Rust
        // argument quoting around a command string makes cmd.exe parse the
        // nested path quotes incorrectly on Windows runners.
        let command = format!(
            "/C mklink /J \"{}\" \"{}\"", 
            link.display(),
            target.display()
        );
        let output = Command::new("cmd.exe")
            .raw_arg(command)
            .output()
            .expect("cmd.exe must be available on Windows");
        assert!(
            output.status.success(),
            "junction creation failed: {}; stdout={}; stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    #[test]
    fn filesystem_reader_rejects_directory_junction_escape() {
        let root = test_root("junction-read");
        let outside = root.with_extension("outside");
        std::fs::create_dir_all(&outside).unwrap();
        let sentinel = outside.join("sentinel.txt");
        std::fs::write(&sentinel, b"OUTSIDE_SENTINEL").unwrap();
        let junction = root.join("junction");
        create_junction(&junction, &outside);

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileReadAdapter::with_authority(authority);
        let result = adapter.invoke(&read_request("junction\\sentinel.txt"));

        assert_eq!(result, Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"OUTSIDE_SENTINEL");
        drop(adapter);
        std::fs::remove_dir(&junction).unwrap();
        std::fs::remove_dir_all(&root).unwrap();
        std::fs::remove_dir_all(outside).unwrap();
    }

    #[test]
    fn filesystem_adapter_reads_absolute_target_inside_project_root() {
        let root = test_root("absolute-read");
        let target = root.join("inside.txt");
        std::fs::write(&target, b"INSIDE_SENTINEL").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileReadAdapter::with_authority(authority);
        let result = adapter.invoke(&read_request(target.to_str().unwrap())).unwrap();

        assert!(result.success);
        assert_eq!(result.output, b"INSIDE_SENTINEL");
        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn filesystem_adapters_confine_absolute_targets_to_project_root_without_side_effects() {
        let root = test_root("absolute-confine");
        let inside = root.join("inside.txt");
        let outside = root.with_extension("outside.txt");
        std::fs::write(&inside, b"INSIDE_UNCHANGED").unwrap();
        std::fs::write(&outside, b"OUTSIDE_UNCHANGED").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let reader = FileReadAdapter::with_authority(Arc::clone(&authority));
        let writer = FileWriteAdapter::with_authority(authority);

        assert_eq!(
            reader.invoke(&read_request(outside.to_str().unwrap())),
            Err(AdapterError::ResolutionFailed)
        );
        assert_eq!(
            writer.invoke(&write_request(outside.to_str().unwrap(), "ATTACK")),
            Err(AdapterError::ResolutionFailed)
        );
        assert_eq!(std::fs::read(&inside).unwrap(), b"INSIDE_UNCHANGED");
        assert_eq!(std::fs::read(&outside).unwrap(), b"OUTSIDE_UNCHANGED");
        drop(reader);
        drop(writer);
        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_file(outside).unwrap();
    }

    #[test]
    fn filesystem_write_adapter_writes_text_target() {
        let root = test_root("write-text");
        let target = root.join("target.txt");
        std::fs::write(&target, b"old").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);
        let result = adapter.invoke(&write_request("target.txt", "new text")).unwrap();

        assert!(result.success);
        assert_eq!(result.output, b"new text");
        assert_eq!(std::fs::read(&target).unwrap(), b"new text");
        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn filesystem_write_adapter_preserves_target_when_atomic_replace_is_denied() {
        use std::fs::OpenOptions;
        use std::os::windows::fs::OpenOptionsExt;

        let root = test_root("atomic-write-denied");
        let target = root.join("target.txt");
        std::fs::write(&target, b"KEEP_ORIGINAL").unwrap();

        // Keep the existing destination open without FILE_SHARE_DELETE.
        // The adapter may inspect/read it, but Windows must deny replacement
        // while this independent handle remains open.
        let blocker = OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(0x0000_0001 | 0x0000_0002) // FILE_SHARE_READ | FILE_SHARE_WRITE
            .open(&target)
            .unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);
        let result = adapter.invoke(&write_request("target.txt", "REPLACEMENT"));

        assert_eq!(result, Err(AdapterError::BackendFailure));
        assert_eq!(std::fs::read(&target).unwrap(), b"KEEP_ORIGINAL");
        let entries = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>();
        assert_eq!(
            entries,
            vec![std::ffi::OsString::from("target.txt")],
            "temporary sibling must be removed after a denied atomic replace"
        );

        drop(blocker);
        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn filesystem_write_adapter_rejects_oversized_content() {
        let root = test_root("write-limit");
        let target = root.join("target.txt");
        std::fs::write(&target, b"KEEP_ORIGINAL").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4).unwrap());
        let adapter = FileWriteAdapter::with_authority(authority);
        let result = adapter.invoke(&write_request("target.txt", "content exceeds limit"));

        assert_eq!(result, Err(AdapterError::BackendFailure));
        assert_eq!(std::fs::read(&target).unwrap(), b"KEEP_ORIGINAL");
        drop(adapter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn filesystem_adapters_reject_parent_directory_traversal() {
        let root = test_root("parent-traversal");
        let outside = root.with_extension("outside.txt");
        std::fs::write(&outside, b"OUTSIDE_UNCHANGED").unwrap();

        let authority = Arc::new(FilesystemAuthority::open(&root, 4096, 4096).unwrap());
        let reader = FileReadAdapter::with_authority(Arc::clone(&authority));
        let writer = FileWriteAdapter::with_authority(authority);
        let traversal = "..\\outside.txt";

        assert_eq!(reader.invoke(&read_request(traversal)), Err(AdapterError::ResolutionFailed));
        assert_eq!(writer.invoke(&write_request(traversal, "ATTACK")), Err(AdapterError::ResolutionFailed));
        assert_eq!(std::fs::read(&outside).unwrap(), b"OUTSIDE_UNCHANGED");
        drop(reader);
        drop(writer);
        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_file(outside).unwrap();
    }
}


#[cfg(test)]
mod filesystem_platform_metadata_tests {
    use super::*;

    #[test]
    fn filesystem_adapters_advertise_only_platforms_with_native_backends() {
        let reader = FileReadAdapter::new();
        let writer = FileWriteAdapter::new();

        #[cfg(target_os = "linux")]
        {
            for (platform, expected) in [
                (PlatformKind::Linux, true),
                (PlatformKind::Windows, false),
                (PlatformKind::Android, false),
                (PlatformKind::Macos, false),
            ] {
                assert_eq!(reader.metadata().available_on(platform), expected, "reader metadata for {platform:?}");
                assert_eq!(writer.metadata().available_on(platform), expected, "writer metadata for {platform:?}");
            }
        }

        #[cfg(windows)]
        {
            for (platform, expected) in [
                (PlatformKind::Linux, false),
                (PlatformKind::Windows, true),
                (PlatformKind::Android, false),
                (PlatformKind::Macos, false),
            ] {
                assert_eq!(reader.metadata().available_on(platform), expected, "reader metadata for {platform:?}");
                assert_eq!(writer.metadata().available_on(platform), expected, "writer metadata for {platform:?}");
            }
        }

        #[cfg(not(any(target_os = "linux", windows)))]
        {
            for platform in [PlatformKind::Linux, PlatformKind::Windows, PlatformKind::Android, PlatformKind::Macos] {
                assert!(!reader.metadata().available_on(platform), "unsupported reader platform {platform:?} must not be advertised");
                assert!(!writer.metadata().available_on(platform), "unsupported writer platform {platform:?} must not be advertised");
            }
        }
    }
}
