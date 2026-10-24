use std::io::{Read, Write};
use std::path::{Component, Path};
use std::time::Duration;

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;

fn validate_filesystem_target(target: &str) -> Result<(), AdapterError> {
    if target.is_empty() || target.contains('\0') {
        return Err(AdapterError::InvalidArguments);
    }
    let path = Path::new(target);
    if path.components().any(|component| matches!(component, Component::ParentDir)) {
        return Err(AdapterError::InvalidArguments);
    }
    let mut current = Path::new("").to_path_buf();
    for component in path.components() {
        current.push(component.as_os_str());
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => return Err(AdapterError::InvalidArguments),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(_) => return Err(AdapterError::BackendFailure),
        }
    }
    Ok(())
}

// On Unix platforms, resolve each directory relative to an already-open directory
// descriptor. O_NOFOLLOW applies at every component, eliminating the check/open
// symlink race that static symlink_metadata checks alone cannot prevent.
#[cfg(unix)]
fn open_directory_nofollow(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::fd::{AsRawFd, FromRawFd};

    let root = if path.is_absolute() { c"/".as_ptr() } else { c".".as_ptr() };
    // SAFETY: root is a static NUL-terminated path and open returns a new owned fd.
    let root_fd = unsafe { libc::open(root.cast(), libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC) };
    if root_fd < 0 { return Err(std::io::Error::last_os_error()); }
    // SAFETY: root_fd was freshly returned by open and is now owned by File.
    let mut directory = unsafe { std::fs::File::from_raw_fd(root_fd) };

    for component in path.components() {
        match component {
            Component::RootDir | Component::CurDir => continue,
            Component::ParentDir => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "parent path components are forbidden")),
            Component::Normal(name) => {
                use std::os::unix::ffi::OsStrExt;
                let name = std::ffi::CString::new(name.as_bytes())
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "NUL in path"))?;
                // SAFETY: directory is a valid open directory fd and name is NUL terminated.
                let next_fd = unsafe {
                    libc::openat(directory.as_raw_fd(), name.as_ptr(),
                        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW)
                };
                if next_fd < 0 { return Err(std::io::Error::last_os_error()); }
                // SAFETY: next_fd is a fresh owned fd from openat.
                directory = unsafe { std::fs::File::from_raw_fd(next_fd) };
            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "unsupported path prefix")),
        }
    }
    Ok(directory)
}

#[cfg(unix)]
fn open_read_target(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;

    let parent = path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let name = path.file_name().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "file name is required"))?;
    let name = std::ffi::CString::new(name.as_bytes())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "NUL in path"))?;
    let directory = open_directory_nofollow(parent)?;
    // O_NONBLOCK avoids hanging if an attacker swaps a regular file for a FIFO.
    // SAFETY: directory is an owned directory fd and name is NUL terminated.
    let fd = unsafe {
        libc::openat(directory.as_raw_fd(), name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK)
    };
    if fd < 0 { return Err(std::io::Error::last_os_error()); }
    // SAFETY: fd is a fresh owned fd from openat.
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "target is not a regular file"));
    }
    Ok(file)
}

#[cfg(windows)]
fn open_read_target(path: &Path) -> std::io::Result<std::fs::File> {
    use std::fs::OpenOptions;
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

    let lexical = windows_lexical_absolute(path)?;
    let canonical = std::fs::canonicalize(path)?;
    if windows_normalized_path(&canonical.to_string_lossy())
        != windows_normalized_path(&lexical.to_string_lossy())
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "target resolves through a reparse point or non-canonical path",
        ));
    }
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "target is not a regular non-reparse file",
        ));
    }
    windows_validate_opened_file(&file, &canonical)?;
    Ok(file)
}

#[cfg(not(any(unix, windows)))]
fn open_read_target(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::File::open(path)
}

#[cfg(unix)]
fn open_write_target(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;

    let parent = path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let name = path.file_name().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "file name is required"))?;
    let name = std::ffi::CString::new(name.as_bytes())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "NUL in path"))?;
    let directory = open_directory_nofollow(parent)?;
    // SAFETY: directory is an owned directory fd and name is NUL terminated.
    let fd = unsafe {
        libc::openat(directory.as_raw_fd(), name.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
            0o666)
    };
    if fd < 0 { return Err(std::io::Error::last_os_error()); }
    // SAFETY: fd is a fresh owned fd from openat.
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "target is not a regular file"));
    }
    Ok(file)
}

#[cfg(windows)]
fn open_write_target(path: &Path) -> std::io::Result<std::fs::File> {
    use std::fs::{self, OpenOptions};
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

    match fs::symlink_metadata(path) {
        Ok(existing) => {
            if existing.file_type().is_symlink() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "target must not be a symlink",
                ));
            }
            let lexical = windows_lexical_absolute(path)?;
            let canonical = fs::canonicalize(path)?;
            if windows_normalized_path(&canonical.to_string_lossy())
                != windows_normalized_path(&lexical.to_string_lossy())
            {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "target resolves through a reparse point or non-canonical path",
                ));
            }
            let file = OpenOptions::new()
                .write(true)
                .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
                .open(path)?;
            let metadata = file.metadata()?;
            if !metadata.is_file() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "target is not a regular non-reparse file",
                ));
            }
            windows_validate_opened_file(&file, &canonical)?;
            file.set_len(0)?;
            Ok(file)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let lexical = windows_lexical_absolute(path)?;
            let parent = path.parent().filter(|value| !value.as_os_str().is_empty()).unwrap_or(Path::new("."));
            let lexical_parent = windows_lexical_absolute(parent)?;
            let canonical_parent = fs::canonicalize(parent)?;
            if windows_normalized_path(&canonical_parent.to_string_lossy())
                != windows_normalized_path(&lexical_parent.to_string_lossy())
            {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "parent resolves through a reparse point or non-canonical path",
                ));
            }
            let name = path.file_name().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "file name is required")
            })?;
            let expected = canonical_parent.join(name);
            if windows_normalized_path(&expected.to_string_lossy())
                != windows_normalized_path(&lexical.to_string_lossy())
            {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "target parent changed during path resolution",
                ));
            }
            let file = OpenOptions::new()
                .write(true)
                // GENERIC_WRITE | DELETE permits cleanup by the already-open handle if
                // the parent is swapped between validation and creation.
                .access_mode(0x4000_0000 | 0x0001_0000)
                .create_new(true)
                .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
                .open(path)?;
            if let Err(validation_error) = windows_validate_opened_file(&file, &expected) {
                windows_delete_opened_file(&file);
                return Err(validation_error);
            }
            Ok(file)
        }
        Err(error) => Err(error),
    }
}

#[cfg(not(any(unix, windows)))]
fn open_write_target(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path)
}

#[cfg(windows)]
fn windows_lexical_absolute(path: &Path) -> std::io::Result<std::path::PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut normalized = std::path::PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "parent path components are forbidden",
                ));
            }
            Component::Normal(name) => normalized.push(name),
        }
    }
    Ok(normalized)
}

#[cfg(windows)]
fn windows_normalized_path(path: &str) -> String {
    let value = path.replace('/', "\\").to_ascii_lowercase();
    let value = if let Some(rest) = value.strip_prefix("\\\\?\\unc\\") {
        format!("\\\\{rest}")
    } else if let Some(rest) = value.strip_prefix("\\\\?\\") {
        rest.to_owned()
    } else {
        value
    };
    value.trim_end_matches('\\').to_owned()
}

#[cfg(windows)]
fn windows_final_path(file: &std::fs::File) -> std::io::Result<String> {
    use std::os::windows::io::AsRawHandle;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetFinalPathNameByHandleW(
            file: *mut std::ffi::c_void,
            path: *mut u16,
            length: u32,
            flags: u32,
        ) -> u32;
    }

    let mut buffer = vec![0u16; 32_768];
    // SAFETY: the handle is live and buffer is writable for its declared length.
    let length = unsafe {
        GetFinalPathNameByHandleW(
            file.as_raw_handle() as *mut std::ffi::c_void,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            0,
        )
    };
    if length == 0 || length as usize >= buffer.len() {
        return Err(std::io::Error::last_os_error());
    }
    buffer.truncate(length as usize);
    String::from_utf16(&buffer).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-16 final handle path")
    })
}

#[cfg(windows)]
fn windows_validate_opened_file(
    file: &std::fs::File,
    expected: &Path,
) -> std::io::Result<()> {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "opened target is not a regular non-reparse file",
        ));
    }
    let actual = windows_normalized_path(&windows_final_path(file)?);
    if actual != windows_normalized_path(&expected.to_string_lossy()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "opened handle resolved to a different path",
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn windows_delete_opened_file(file: &std::fs::File) {
    use std::os::windows::io::AsRawHandle;
    #[repr(C)]
    struct FileDispositionInfo {
        delete_file: u8,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn SetFileInformationByHandle(
            file: *mut std::ffi::c_void,
            class: i32,
            information: *const std::ffi::c_void,
            size: u32,
        ) -> i32;
    }
    let information = FileDispositionInfo { delete_file: 1 };
    // SAFETY: information is a correctly sized FILE_DISPOSITION_INFO and the handle is live.
    let _ = unsafe {
        SetFileInformationByHandle(
            file.as_raw_handle() as *mut std::ffi::c_void,
            4,
            &information as *const FileDispositionInfo as *const std::ffi::c_void,
            std::mem::size_of::<FileDispositionInfo>() as u32,
        )
    };
}

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
pub enum AdapterError { InvalidRequest(crate::orchestration::OrchestrationValidationError), UndeclaredCapability, InvalidArguments, UnsupportedPlatform, ResolutionFailed, ResourceLimitExceeded, BackendFailure }
pub trait ToolAdapter { fn metadata(&self) -> &AdapterMetadata; fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError>; }

pub struct FileReadAdapter { metadata: AdapterMetadata }
impl FileReadAdapter { pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("filesystem.file.read"), AdapterStatus::Implemented, vec![Capability::FilesystemRead], vec![PlatformKind::Any]) } } }
impl Default for FileReadAdapter { fn default() -> Self { Self::new() } }
impl ToolAdapter for FileReadAdapter { fn metadata(&self) -> &AdapterMetadata { &self.metadata } fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> { self.metadata.validate_request(request.capability())?; if !request.arguments().is_empty() { return Err(AdapterError::InvalidArguments); } let path = match &request.capability().scope { ExecutionScope::Target(value) if !value.is_empty() => value, _ => return Err(AdapterError::InvalidRequest(crate::orchestration::OrchestrationValidationError::EmptyScope)) }; validate_filesystem_target(path)?; (|| { let file = open_read_target(Path::new(path)).map_err(|_| AdapterError::BackendFailure)?; if file.metadata().map_err(|_| AdapterError::BackendFailure)?.len() > MAX_FILE_BYTES { return Err(AdapterError::BackendFailure); } let mut output = Vec::new(); file.take(MAX_FILE_BYTES + 1).read_to_end(&mut output).map_err(|_| AdapterError::BackendFailure)?; if output.len() as u64 > MAX_FILE_BYTES { return Err(AdapterError::BackendFailure); } Ok(AdapterResult { success: true, output }) })() } }

pub struct FileWriteAdapter { metadata: AdapterMetadata }
impl FileWriteAdapter { pub fn new() -> Self { Self { metadata: AdapterMetadata::new(AdapterId::new("filesystem.file.write"), AdapterStatus::Implemented, vec![Capability::FilesystemWrite], vec![PlatformKind::Any]) } } }
impl Default for FileWriteAdapter { fn default() -> Self { Self::new() } }
impl ToolAdapter for FileWriteAdapter { fn metadata(&self) -> &AdapterMetadata { &self.metadata } fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> { self.metadata.validate_request(request.capability())?; let content = match request.arguments() { [AdapterArgument::Text(value)] => value.as_bytes().to_vec(), _ => return Err(AdapterError::InvalidArguments) }; let path = match &request.capability().scope { ExecutionScope::Target(value) if !value.is_empty() => value, _ => return Err(AdapterError::InvalidRequest(crate::orchestration::OrchestrationValidationError::EmptyScope)) }; validate_filesystem_target(path)?; if content.len() as u64 > MAX_FILE_BYTES { return Err(AdapterError::InvalidArguments); } open_write_target(Path::new(path)).and_then(|mut file| file.write_all(&content)).map(|()| AdapterResult { success: true, output: content }).map_err(|_| AdapterError::BackendFailure) } }

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
mod tests { use super::*; use crate::capability::Capability; use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId}; fn request() -> CapabilityRequest { CapabilityRequest::new(SemanticCapabilityId::NetworkConnect, Capability::NetworkConnect, ExecutionScope::target("server.example"), EffectClass::Connect) } fn metadata() -> AdapterMetadata { AdapterMetadata::new(AdapterId::new("network.native"), AdapterStatus::Implemented, vec![Capability::NetworkConnect], vec![PlatformKind::Linux, PlatformKind::Windows]) } #[test] fn adapter_identity_and_capability_metadata_are_stable() { let metadata = metadata(); assert_eq!(metadata.id().as_str(), "network.native"); assert!(metadata.declares(Capability::NetworkConnect)); assert!(!metadata.declares(Capability::NetworkAdmin)); } #[test] fn adapter_platform_compatibility_is_explicit() { let metadata = metadata(); assert!(metadata.available_on(PlatformKind::Linux)); assert!(metadata.available_on(PlatformKind::Windows)); assert!(!metadata.available_on(PlatformKind::Android)); } #[test] fn adapter_request_requires_declared_capability() { let metadata = metadata(); assert!(metadata.validate_request(&request()).is_ok()); let undeclared = CapabilityRequest::new(SemanticCapabilityId::NetworkAdmin, Capability::NetworkAdmin, ExecutionScope::target("server.example"), EffectClass::Admin); assert_eq!(metadata.validate_request(&undeclared), Err(AdapterError::UndeclaredCapability)); } #[test] fn adapter_request_contains_typed_arguments_not_command_text() { let request = AdapterRequest::new(request()).with_argument(AdapterArgument::Text("server.example".into())); assert_eq!(request.arguments(), &[AdapterArgument::Text("server.example".into())]); } #[test] fn adapter_errors_distinguish_backend_resolution_failure_from_platform_unavailability() { assert_ne!(AdapterError::ResolutionFailed, AdapterError::UnsupportedPlatform); } #[test] fn filesystem_read_adapter_succeeds_for_existing_file() { let adapter = FileReadAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemRead, Capability::FilesystemRead, ExecutionScope::target("Cargo.toml"), EffectClass::Read); let result = adapter.invoke(&AdapterRequest::new(request)).expect("existing file should be readable"); assert!(result.success); assert!(!result.output.is_empty()); } #[test] fn filesystem_read_adapter_classifies_missing_backend_as_backend_failure() { let adapter = FileReadAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemRead, Capability::FilesystemRead, ExecutionScope::target("this-file-must-not-exist.one"), EffectClass::Read); assert_eq!(adapter.invoke(&AdapterRequest::new(request)), Err(AdapterError::BackendFailure)); } 

#[test] fn filesystem_adapters_reject_parent_directory_traversal() { assert_eq!(validate_filesystem_target("../Cargo.toml"), Err(AdapterError::InvalidArguments)); }
#[cfg(unix)] #[test] fn filesystem_adapters_reject_symlink_targets() { use std::os::unix::fs::symlink; let base = std::env::temp_dir().join("one-filesystem-symlink-target-test"); let target = base.with_extension("target"); let link = base.with_extension("link"); std::fs::write(&target, "secret").expect("target should be created"); symlink(&target, &link).expect("symlink should be created"); assert_eq!(validate_filesystem_target(&link.display().to_string()), Err(AdapterError::InvalidArguments)); let _ = std::fs::remove_file(link); let _ = std::fs::remove_file(target); }
#[test] fn filesystem_read_adapter_rejects_oversized_files() { let path = std::env::temp_dir().join("one-filesystem-read-adapter-oversize-test.bin"); std::fs::write(&path, vec![0u8; MAX_FILE_BYTES as usize + 1]).expect("oversized fixture should be written"); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemRead, Capability::FilesystemRead, ExecutionScope::target(path.display().to_string()), EffectClass::Read); assert_eq!(FileReadAdapter::new().invoke(&AdapterRequest::new(request)), Err(AdapterError::BackendFailure)); let _ = std::fs::remove_file(path); }
#[test] fn filesystem_write_adapter_rejects_oversized_content() { let path = std::env::temp_dir().join("one-filesystem-write-adapter-oversize-test.bin"); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemWrite, Capability::FilesystemWrite, ExecutionScope::target(path.display().to_string()), EffectClass::Write); assert_eq!(FileWriteAdapter::new().invoke(&AdapterRequest::new(request).with_argument(AdapterArgument::Text("x".repeat(MAX_FILE_BYTES as usize + 1)))), Err(AdapterError::InvalidArguments)); assert!(!path.exists()); } #[test] fn filesystem_write_adapter_writes_text_target() { let path = std::env::temp_dir().join("one-filesystem-write-adapter-test.txt"); let adapter = FileWriteAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::FilesystemWrite, Capability::FilesystemWrite, ExecutionScope::target(path.display().to_string()), EffectClass::Write); let result = adapter.invoke(&AdapterRequest::new(request).with_argument(AdapterArgument::Text("adapter write".into()))).expect("write should succeed"); assert!(result.success); assert_eq!(std::fs::read_to_string(&path).unwrap(), "adapter write"); let _ = std::fs::remove_file(path); } #[test] fn process_adapter_executes_scoped_program_and_returns_output() { let adapter = ProcessAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::ProcessExecute, Capability::Process, ExecutionScope::target("rustc"), EffectClass::Execute); let result = adapter.invoke(&AdapterRequest::new(request)).expect("rustc should be executable in the verification environment"); assert!(result.success); assert!(!result.output.is_empty()); } #[test] fn http_get_adapter_rejects_non_http_target() { let adapter = HttpGetAdapter::new(); let request = CapabilityRequest::new(SemanticCapabilityId::NetworkConnect, Capability::NetworkConnect, ExecutionScope::target("https://example.com/"), EffectClass::Connect); assert_eq!(adapter.invoke(&AdapterRequest::new(request)), Err(AdapterError::InvalidArguments)); } }

#[cfg(all(test, unix))]
mod filesystem_nofollow_tests {
    use super::*;
    use std::os::unix::fs::symlink;

    fn test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("one-fs-nofollow-{label}-{}", std::process::id()))
    }

    #[test]
    fn descriptor_relative_read_rejects_symlinked_parent_even_without_precheck() {
        let root = test_root("parent");
        let real = root.join("real");
        let link = root.join("link");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&real).expect("create real directory");
        std::fs::write(real.join("secret.txt"), b"secret").expect("create target");
        symlink(&real, &link).expect("create directory symlink");

        assert!(open_read_target(&link.join("secret.txt")).is_err());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn descriptor_relative_read_rejects_final_symlink() {
        let root = test_root("final-read");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create directory");
        std::fs::write(root.join("real.txt"), b"secret").expect("create target");
        symlink(root.join("real.txt"), root.join("link.txt")).expect("create file symlink");

        assert!(open_read_target(&root.join("link.txt")).is_err());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn descriptor_relative_write_rejects_final_symlink_without_modifying_target() {
        let root = test_root("final-write");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create directory");
        std::fs::write(root.join("real.txt"), b"preserve").expect("create target");
        symlink(root.join("real.txt"), root.join("link.txt")).expect("create file symlink");

        assert!(open_write_target(&root.join("link.txt")).is_err());
        assert_eq!(std::fs::read(root.join("real.txt")).expect("read target"), b"preserve");
        let _ = std::fs::remove_dir_all(root);
    }

}

#[cfg(all(test, windows))]
mod filesystem_windows_reparse_tests {
    use super::*;

    #[test]
    fn filesystem_reader_rejects_directory_junction_escape() {
        let root = std::env::temp_dir().join(format!("one-fs-junction-{}", std::process::id()));
        let outside = root.join("outside");
        let junction = root.join("junction");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&outside).expect("create outside directory");
        std::fs::write(outside.join("secret.txt"), b"secret").expect("create target");

        let status = std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&junction)
            .arg(&outside)
            .status()
            .expect("cmd should be available on Windows");
        assert!(status.success(), "directory junction must be created for the filesystem regression");

        assert!(open_read_target(&junction.join("secret.txt")).is_err());
        let _ = std::fs::remove_dir(&junction);
        let _ = std::fs::remove_dir_all(root);
    }
}
