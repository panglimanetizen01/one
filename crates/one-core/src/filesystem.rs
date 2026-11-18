//! Host-owned filesystem authority for Issue #104.
//!
//! Linux reads use one openat2 resolution rooted at a retained directory
//! descriptor. Windows uses retained native handles and component-relative
//! NtCreateFile opens. The root path is trusted host
//! configuration, never a request supplied by the language program.

use std::fs::File;
use std::io::Read;
#[cfg(windows)]
use std::io::Write;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemError {
    InvalidRoot, InvalidTarget, OutsideRoot, UnsupportedPlatform,
    ResolutionFailed, NotRegularFile, HardLinkedFile, ReadLimitExceeded,
    WriteLimitExceeded, IoFailure,
}

/// Immutable authority created by the host for one project root.
pub struct FilesystemAuthority {
    root: File,
    canonical_root: PathBuf,
    max_read_bytes: u64,
    max_write_bytes: u64,
}

#[cfg(windows)]
fn normalize_windows_verbatim_prefix(path: &Path) -> PathBuf {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};

    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    const VERBATIM_PREFIX: [u16; 4] = [92, 92, 63, 92];
    const VERBATIM_UNC_PREFIX: [u16; 8] = [92, 92, 63, 92, 85, 78, 67, 92];

    if wide.starts_with(&VERBATIM_UNC_PREFIX) {
        let mut normalized = vec![92, 92];
        normalized.extend_from_slice(&wide[VERBATIM_UNC_PREFIX.len()..]);
        PathBuf::from(OsString::from_wide(&normalized))
    } else if wide.starts_with(&VERBATIM_PREFIX) {
        PathBuf::from(OsString::from_wide(&wide[VERBATIM_PREFIX.len()..]))
    } else {
        path.to_path_buf()
    }
}

impl FilesystemAuthority {
    /// Open and retain the host-selected project root. The root is trusted
    /// host configuration; never pass a program-controlled target here.
    pub fn open(root: impl AsRef<Path>, max_read_bytes: u64, max_write_bytes: u64) -> Result<Self, FilesystemError> {
        #[cfg(not(any(target_os = "linux", windows)))]
        {
            let _ = (root, max_read_bytes, max_write_bytes);
            return Err(FilesystemError::UnsupportedPlatform);
        }
        #[cfg(target_os = "linux")]
        {
            if max_read_bytes == 0 || max_write_bytes == 0 { return Err(FilesystemError::InvalidRoot); }
            let canonical_root = std::fs::canonicalize(root).map_err(|_| FilesystemError::InvalidRoot)?;
            let root = File::open(&canonical_root).map_err(|_| FilesystemError::InvalidRoot)?;
            if !root.metadata().map_err(|_| FilesystemError::InvalidRoot)?.is_dir() {
                return Err(FilesystemError::InvalidRoot);
            }
            Ok(Self { root, canonical_root, max_read_bytes, max_write_bytes })
        }
        #[cfg(windows)]
        {
            if max_read_bytes == 0 || max_write_bytes == 0 { return Err(FilesystemError::InvalidRoot); }
            let canonical_root = std::fs::canonicalize(root).map_err(|_| FilesystemError::InvalidRoot)?;
            let root = windows_native::open_root(&canonical_root)?;
            Ok(Self { root, canonical_root, max_read_bytes, max_write_bytes })
        }
    }

    pub fn root(&self) -> &Path { &self.canonical_root }
    pub const fn max_read_bytes(&self) -> u64 { self.max_read_bytes }
    pub const fn max_write_bytes(&self) -> u64 { self.max_write_bytes }

    /// Read a regular, single-link file through the retained root handle.
    /// Absolute targets are accepted only when beneath the trusted root.
    pub fn read(&self, target: impl AsRef<Path>) -> Result<Vec<u8>, FilesystemError> {
        let relative = self.relative_target(target.as_ref())?;
        let file = self.open_relative(&relative)?;
        let metadata = file.metadata().map_err(|_| FilesystemError::IoFailure)?;
        if !metadata.is_file() { return Err(FilesystemError::NotRegularFile); }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.nlink() != 1 { return Err(FilesystemError::HardLinkedFile); }
        }
        if metadata.len() > self.max_read_bytes { return Err(FilesystemError::ReadLimitExceeded); }
        let mut bytes = Vec::with_capacity(metadata.len().min(self.max_read_bytes) as usize);
        file.take(self.max_read_bytes.saturating_add(1)).read_to_end(&mut bytes)
            .map_err(|_| FilesystemError::IoFailure)?;
        if bytes.len() as u64 > self.max_read_bytes { return Err(FilesystemError::ReadLimitExceeded); }
        Ok(bytes)
    }

    /// Write through a retained native handle. Windows opens every path
    /// component relative to a retained parent handle and validates the final
    /// object before truncating it. Other platforms remain fail-closed.
    pub fn write(&self, target: impl AsRef<Path>, bytes: &[u8]) -> Result<(), FilesystemError> {
        let relative = self.relative_target(target.as_ref())?;
        if bytes.len() as u64 > self.max_write_bytes {
            return Err(FilesystemError::WriteLimitExceeded);
        }
        #[cfg(windows)]
        {
            let mut file = windows_native::open_relative(&self.root, &relative, true)?;
            file.set_len(0).map_err(|_| FilesystemError::IoFailure)?;
            file.write_all(bytes).map_err(|_| FilesystemError::IoFailure)?;
            file.flush().map_err(|_| FilesystemError::IoFailure)?;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            let _ = relative;
            Err(FilesystemError::UnsupportedPlatform)
        }
    }

    fn relative_target(&self, target: &Path) -> Result<PathBuf, FilesystemError> {
        #[cfg(windows)]
        let relative = if target.is_absolute() {
            let normalized_root = normalize_windows_verbatim_prefix(&self.canonical_root);
            let normalized_target = normalize_windows_verbatim_prefix(target);
            normalized_target
                .strip_prefix(&normalized_root)
                .map_err(|_| FilesystemError::OutsideRoot)?
                .to_path_buf()
        } else {
            target.to_path_buf()
        };
        #[cfg(not(windows))]
        let relative = if target.is_absolute() {
            target.strip_prefix(&self.canonical_root).map_err(|_| FilesystemError::OutsideRoot)?.to_path_buf()
        } else {
            target.to_path_buf()
        };
        let mut normalized = PathBuf::new();
        for component in relative.components() {
            match component {
                Component::Normal(part) => normalized.push(part),
                Component::CurDir => {},
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => return Err(FilesystemError::OutsideRoot),
            }
        }
        if normalized.as_os_str().is_empty() { return Err(FilesystemError::InvalidTarget); }
        Ok(normalized)
    }

    #[cfg(target_os = "linux")]
    fn open_relative(&self, relative: &Path) -> Result<File, FilesystemError> {
        use std::ffi::CString;
        use std::os::fd::{AsRawFd, FromRawFd};
        use std::os::unix::ffi::OsStrExt;

        #[repr(C)]
        struct OpenHow { flags: u64, mode: u64, resolve: u64 }
        const RESOLVE_NO_XDEV: u64 = 0x01;
        const RESOLVE_NO_SYMLINKS: u64 = 0x04;
        const RESOLVE_BENEATH: u64 = 0x08;

        let path = CString::new(relative.as_os_str().as_bytes()).map_err(|_| FilesystemError::InvalidTarget)?;
        let how = OpenHow {
            flags: (libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW) as u64,
            mode: 0,
            resolve: RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV,
        };
        // SAFETY: path is NUL-terminated, OpenHow matches the kernel ABI, and
        // self.root owns a live directory descriptor for this call.
        let fd = unsafe {
            libc::syscall(libc::SYS_openat2, self.root.as_raw_fd(), path.as_ptr(),
                &how as *const OpenHow, std::mem::size_of::<OpenHow>()) as libc::c_int
        };
        if fd < 0 {
            // Preserve the distinction between an ordinary missing/unreadable
            // file and a kernel-enforced confinement denial. In particular,
            // ELOOP/EXDEV/EAGAIN are fail-closed path-resolution outcomes;
            // ENOSYS/EPERM/EINVAL mean this kernel/security profile cannot
            // provide the required openat2 contract.
            return match std::io::Error::last_os_error().raw_os_error() {
                Some(libc::ELOOP) | Some(libc::EXDEV) | Some(libc::EAGAIN) => {
                    Err(FilesystemError::ResolutionFailed)
                }
                Some(libc::ENOSYS) | Some(libc::EPERM) | Some(libc::EINVAL) => {
                    Err(FilesystemError::UnsupportedPlatform)
                }
                _ => Err(FilesystemError::IoFailure),
            };
        }
        // SAFETY: successful openat2 returns a new owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    #[cfg(windows)]
    fn open_relative(&self, relative: &Path) -> Result<File, FilesystemError> {
        windows_native::open_relative(&self.root, relative, false)
    }

    #[cfg(not(any(target_os = "linux", windows)))]
    fn open_relative(&self, _relative: &Path) -> Result<File, FilesystemError> {
        Err(FilesystemError::UnsupportedPlatform)
    }
}


#[cfg(windows)]
mod windows_native {
    use super::FilesystemError;
    use std::ffi::c_void;
    use std::fs::File;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    use std::path::{Component, Path};

    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
    };

    const OBJ_CASE_INSENSITIVE: u32 = 0x0000_0040;
    const OBJ_DONT_REPARSE: u32 = 0x0000_1000;

    const FILE_LIST_DIRECTORY: u32 = 0x0000_0001;
    const FILE_READ_DATA: u32 = 0x0000_0001;
    const FILE_WRITE_DATA: u32 = 0x0000_0002;
    const FILE_TRAVERSE: u32 = 0x0000_0020;
    const FILE_READ_ATTRIBUTES: u32 = 0x0000_0080;
    const SYNCHRONIZE: u32 = 0x0010_0000;

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;

    const FILE_OPEN: u32 = 1;
    const FILE_OPEN_IF: u32 = 3;
    const FILE_DIRECTORY_FILE: u32 = 0x0000_0001;
    const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;
    const FILE_NON_DIRECTORY_FILE: u32 = 0x0000_0040;
    const FILE_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x0000_0080;

    const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x0000_0010;
    const FILE_ATTRIBUTE_DEVICE: u32 = 0x0000_0040;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

    const STATUS_REPARSE_POINT_ENCOUNTERED: u32 = 0xC000_050B;
    const STATUS_OBJECT_NAME_INVALID: u32 = 0xC000_0033;
    const STATUS_OBJECT_PATH_SYNTAX_BAD: u32 = 0xC000_003B;

    #[repr(C)]
    struct UnicodeString {
        length: u16,
        maximum_length: u16,
        buffer: *mut u16,
    }

    #[repr(C)]
    struct ObjectAttributes {
        length: u32,
        root_directory: HANDLE,
        object_name: *mut UnicodeString,
        attributes: u32,
        security_descriptor: *mut c_void,
        security_quality_of_service: *mut c_void,
    }

    #[repr(C)]
    union IoStatusValue {
        status: i32,
        pointer: *mut c_void,
    }

    #[repr(C)]
    struct IoStatusBlock {
        value: IoStatusValue,
        information: usize,
    }

    #[link(name = "ntdll")]
    unsafe extern "system" {
        fn NtCreateFile(
            file_handle: *mut HANDLE,
            desired_access: u32,
            object_attributes: *const ObjectAttributes,
            io_status_block: *mut IoStatusBlock,
            allocation_size: *const i64,
            file_attributes: u32,
            share_access: u32,
            create_disposition: u32,
            create_options: u32,
            ea_buffer: *const c_void,
            ea_length: u32,
        ) -> i32;
    }

    fn status_error(status: i32) -> FilesystemError {
        match status as u32 {
            STATUS_REPARSE_POINT_ENCOUNTERED => FilesystemError::ResolutionFailed,
            STATUS_OBJECT_NAME_INVALID | STATUS_OBJECT_PATH_SYNTAX_BAD => FilesystemError::InvalidTarget,
            _ => FilesystemError::IoFailure,
        }
    }

    fn file_info(file: &File) -> Result<BY_HANDLE_FILE_INFORMATION, FilesystemError> {
        let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
        let ok = unsafe {
            GetFileInformationByHandle(file.as_raw_handle() as HANDLE, &mut info)
        };
        if ok == 0 { return Err(FilesystemError::IoFailure); }
        Ok(info)
    }

    fn open_named(
        name: &[u16],
        root: HANDLE,
        attributes: u32,
        desired_access: u32,
        share_access: u32,
        disposition: u32,
        options: u32,
    ) -> Result<File, FilesystemError> {
        let byte_length = name.len().checked_mul(2).ok_or(FilesystemError::InvalidTarget)?;
        if byte_length == 0 || byte_length > (u16::MAX as usize - 2) {
            return Err(FilesystemError::InvalidTarget);
        }
        let mut name_buffer = name.to_vec();
        let mut unicode = UnicodeString {
            length: byte_length as u16,
            maximum_length: byte_length as u16,
            buffer: name_buffer.as_mut_ptr(),
        };
        let object_attributes = ObjectAttributes {
            length: std::mem::size_of::<ObjectAttributes>() as u32,
            root_directory: root,
            object_name: &mut unicode,
            attributes,
            security_descriptor: std::ptr::null_mut(),
            security_quality_of_service: std::ptr::null_mut(),
        };
        let mut io_status = IoStatusBlock {
            value: IoStatusValue { status: 0 },
            information: 0,
        };
        let mut handle: HANDLE = std::ptr::null_mut();
        let status = unsafe {
            NtCreateFile(
                &mut handle,
                desired_access,
                &object_attributes,
                &mut io_status,
                std::ptr::null(),
                FILE_ATTRIBUTE_NORMAL,
                share_access,
                disposition,
                options,
                std::ptr::null(),
                0,
            )
        };
        if status < 0 { return Err(status_error(status)); }
        if handle.is_null() { return Err(FilesystemError::IoFailure); }
        Ok(unsafe { File::from_raw_handle(handle) })
    }

    fn absolute_nt_path(path: &Path) -> Result<Vec<u16>, FilesystemError> {
        let source: Vec<u16> = path.as_os_str().encode_wide().collect();
        if source.is_empty() { return Err(FilesystemError::InvalidRoot); }
        let mut result = Vec::with_capacity(source.len() + 8);
        // Convert Win32 extended paths (\\?\C:\...) to the NT object-manager
        // namespace (\??\C:\...). UNC paths become \??\UNC\server\share\...
        if source.starts_with(&[92, 92, 63, 92]) {
            result.extend_from_slice(&[92, 63, 63, 92]);
            result.extend_from_slice(&source[4..]);
        } else if source.starts_with(&[92, 92]) {
            result.extend_from_slice(&[92, 63, 63, 92, 85, 78, 67, 92]);
            result.extend_from_slice(&source[2..]);
        } else {
            result.extend_from_slice(&[92, 63, 63, 92]);
            result.extend_from_slice(&source);
        }
        Ok(result)
    }

    pub fn open_root(path: &Path) -> Result<File, FilesystemError> {
        let name = absolute_nt_path(path)?;
        let root = open_named(
            &name,
            std::ptr::null_mut(),
            OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
            FILE_LIST_DIRECTORY | FILE_TRAVERSE | FILE_READ_ATTRIBUTES | SYNCHRONIZE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            FILE_OPEN,
            FILE_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_OPEN_REPARSE_POINT,
        )?;
        let info = file_info(&root)?;
        if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
            || info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0
        {
            return Err(FilesystemError::InvalidRoot);
        }
        Ok(root)
    }

    pub fn open_relative(root: &File, relative: &Path, for_write: bool) -> Result<File, FilesystemError> {
        let root_info = file_info(root)?;
        if root_info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
            || root_info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0
        {
            return Err(FilesystemError::InvalidRoot);
        }

        let components: Vec<_> = relative.components().collect();
        if components.is_empty() { return Err(FilesystemError::InvalidTarget); }
        let mut parents: Vec<File> = Vec::new();
        let mut parent_handle = root.as_raw_handle() as HANDLE;

        for (index, component) in components.iter().enumerate() {
            let Component::Normal(part) = component else {
                return Err(FilesystemError::OutsideRoot);
            };
            let wide: Vec<u16> = part.encode_wide().collect();
            if wide.is_empty()
                || wide.iter().any(|unit| *unit == 0 || *unit == 92 || *unit == 47)
            {
                return Err(FilesystemError::InvalidTarget);
            }
            let is_final = index + 1 == components.len();
            let desired_access = if is_final {
                (if for_write { FILE_WRITE_DATA } else { FILE_READ_DATA })
                    | FILE_READ_ATTRIBUTES
                    | SYNCHRONIZE
            } else {
                FILE_LIST_DIRECTORY | FILE_TRAVERSE | FILE_READ_ATTRIBUTES | SYNCHRONIZE
            };
            let share_access = if is_final { FILE_SHARE_READ } else { FILE_SHARE_READ | FILE_SHARE_WRITE };
            let disposition = if is_final && for_write { FILE_OPEN_IF } else { FILE_OPEN };
            let options = FILE_SYNCHRONOUS_IO_NONALERT
                | FILE_OPEN_REPARSE_POINT
                | if is_final { FILE_NON_DIRECTORY_FILE } else { FILE_DIRECTORY_FILE };

            let file = open_named(
                &wide,
                parent_handle,
                OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
                desired_access,
                share_access,
                disposition,
                options,
            )?;
            let info = file_info(&file)?;
            if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(FilesystemError::ResolutionFailed);
            }
            if info.dwVolumeSerialNumber != root_info.dwVolumeSerialNumber {
                return Err(FilesystemError::ResolutionFailed);
            }

            if is_final {
                if info.dwFileAttributes & (FILE_ATTRIBUTE_DIRECTORY | FILE_ATTRIBUTE_DEVICE) != 0 {
                    return Err(FilesystemError::NotRegularFile);
                }
                if info.nNumberOfLinks != 1 {
                    return Err(FilesystemError::HardLinkedFile);
                }
                return Ok(file);
            }

            if info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                return Err(FilesystemError::ResolutionFailed);
            }
            parents.push(file);
            parent_handle = parents.last().expect("parent handle was just retained").as_raw_handle() as HANDLE;
        }
        Err(FilesystemError::InvalidTarget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn authority_rejects_zero_limits_and_non_directory_roots() {
        assert_eq!(FilesystemAuthority::open(std::env::temp_dir(), 0, 1).err(), Some(FilesystemError::InvalidRoot));
        let file = std::env::temp_dir().join("one-filesystem-authority-root-file");
        std::fs::write(&file, b"x").unwrap();
        assert_eq!(FilesystemAuthority::open(&file, 10, 10).err(), Some(FilesystemError::InvalidRoot));
        let _ = std::fs::remove_file(file);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn authority_rejects_parent_traversal_and_outside_absolute_targets() {
        let root = std::env::temp_dir();
        let authority = FilesystemAuthority::open(&root, 1024, 1024).unwrap();
        assert_eq!(authority.relative_target(Path::new("../outside")), Err(FilesystemError::OutsideRoot));
        assert_eq!(authority.relative_target(&root.parent().unwrap().join("outside")), Err(FilesystemError::OutsideRoot));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn authority_reads_only_a_regular_file_beneath_retained_root() {
        let root = std::env::temp_dir().join(format!("one-fs-authority-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("inside.txt"), b"inside").unwrap();
        let authority = FilesystemAuthority::open(&root, 32, 32).unwrap();
        assert_eq!(authority.read("inside.txt").unwrap(), b"inside");
        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn authority_rejects_symlink_components() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("one-fs-authority-link-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let outside = root.with_extension("outside");
        std::fs::write(&outside, b"outside").unwrap();
        symlink(&outside, root.join("escape")).unwrap();
        let authority = FilesystemAuthority::open(&root, 32, 32).unwrap();
        assert_eq!(authority.read("escape"), Err(FilesystemError::ResolutionFailed));
        let _ = std::fs::remove_dir_all(root);
        let _ = std::fs::remove_file(outside);
    }
}
