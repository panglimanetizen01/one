//! Host-owned filesystem authority for Issue #104.
//!
//! Linux reads use one openat2 resolution rooted at a retained directory
//! descriptor. Windows uses retained native handles and component-relative
//! NtCreateFile opens. The root path is trusted host
//! configuration, never a request supplied by the language program.

use std::fs::File;
use std::io::Read;
#[cfg(target_os = "linux")]
use std::io::Write;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemError {
    InvalidRoot, InvalidTarget, OutsideRoot, UnsupportedPlatform,
    ResolutionFailed, NotRegularFile, HardLinkedFile, NotFound, ReadLimitExceeded,
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
        #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "macos", windows)))]
        {
            let _ = (root, max_read_bytes, max_write_bytes);
            return Err(FilesystemError::UnsupportedPlatform);
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

            if max_read_bytes == 0 || max_write_bytes == 0 {
                return Err(FilesystemError::InvalidRoot);
            }

            // Open the supplied root itself without following a final symlink.
            // Canonicalize only after acquiring the handle, then compare inode
            // identity so a concurrent path replacement fails closed.
            let root_path = root.as_ref();
            let root = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
                .open(root_path)
                .map_err(|_| FilesystemError::InvalidRoot)?;
            let root_metadata = root.metadata().map_err(|_| FilesystemError::InvalidRoot)?;
            if !root_metadata.is_dir() {
                return Err(FilesystemError::InvalidRoot);
            }

            let canonical_root =
                std::fs::canonicalize(root_path).map_err(|_| FilesystemError::InvalidRoot)?;
            let path_metadata =
                std::fs::metadata(&canonical_root).map_err(|_| FilesystemError::InvalidRoot)?;
            if path_metadata.dev() != root_metadata.dev()
                || path_metadata.ino() != root_metadata.ino()
            {
                return Err(FilesystemError::InvalidRoot);
            }

            Ok(Self { root, canonical_root, max_read_bytes, max_write_bytes })
        }
        #[cfg(target_os = "android")]
        {
            use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

            if max_read_bytes == 0 || max_write_bytes == 0 {
                return Err(FilesystemError::InvalidRoot);
            }
            let root_path = root.as_ref();
            let root = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
                .open(root_path)
                .map_err(|_| FilesystemError::InvalidRoot)?;
            let root_metadata = root.metadata().map_err(|_| FilesystemError::InvalidRoot)?;
            if !root_metadata.is_dir() { return Err(FilesystemError::InvalidRoot); }
            let canonical_root = std::fs::canonicalize(root_path).map_err(|_| FilesystemError::InvalidRoot)?;
            let path_metadata = std::fs::metadata(&canonical_root).map_err(|_| FilesystemError::InvalidRoot)?;
            if path_metadata.dev() != root_metadata.dev() || path_metadata.ino() != root_metadata.ino() {
                return Err(FilesystemError::InvalidRoot);
            }
            android_native::mount_id(&root).map_err(|_| FilesystemError::UnsupportedPlatform)?;
            Ok(Self { root, canonical_root, max_read_bytes, max_write_bytes })
        }
        #[cfg(target_os = "macos")]
        {
            use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
            if max_read_bytes == 0 || max_write_bytes == 0 { return Err(FilesystemError::InvalidRoot); }
            let root_path = root.as_ref();
            let root = std::fs::OpenOptions::new().read(true)
                .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
                .open(root_path).map_err(|_| FilesystemError::InvalidRoot)?;
            let root_metadata = root.metadata().map_err(|_| FilesystemError::InvalidRoot)?;
            if !root_metadata.is_dir() { return Err(FilesystemError::InvalidRoot); }
            let canonical_root = std::fs::canonicalize(root_path).map_err(|_| FilesystemError::InvalidRoot)?;
            let path_metadata = std::fs::metadata(&canonical_root).map_err(|_| FilesystemError::InvalidRoot)?;
            if path_metadata.dev() != root_metadata.dev() || path_metadata.ino() != root_metadata.ino() {
                return Err(FilesystemError::InvalidRoot);
            }
            Ok(Self { root, canonical_root, max_read_bytes, max_write_bytes })
        }
        #[cfg(windows)]
        {
            if max_read_bytes == 0 || max_write_bytes == 0 {
                return Err(FilesystemError::InvalidRoot);
            }
            let canonical_root = windows_native::canonical_root_path(root.as_ref())?;
            let root = windows_native::open_root(&canonical_root)?;
            if !windows_native::root_matches_path(&root, &canonical_root)? {
                return Err(FilesystemError::InvalidRoot);
            }
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
        let initial_capacity = usize::try_from(metadata.len().min(self.max_read_bytes))
            .map_err(|_| FilesystemError::ReadLimitExceeded)?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(initial_capacity)
            .map_err(|_| FilesystemError::IoFailure)?;
        file.take(self.max_read_bytes.saturating_add(1)).read_to_end(&mut bytes)
            .map_err(|_| FilesystemError::IoFailure)?;
        if bytes.len() as u64 > self.max_read_bytes { return Err(FilesystemError::ReadLimitExceeded); }
        Ok(bytes)
    }

    /// Atomically replace a regular, single-link file beneath the retained
    /// root handle. Linux and Windows use platform-native handle-relative
    /// operations; unsupported platforms remain fail-closed.
    pub fn write(&self, target: impl AsRef<Path>, bytes: &[u8]) -> Result<(), FilesystemError> {
        let relative = self.relative_target(target.as_ref())?;
        if bytes.len() as u64 > self.max_write_bytes {
            return Err(FilesystemError::WriteLimitExceeded);
        }
        #[cfg(windows)]
        {
            windows_native::write_relative(&self.root, &relative, bytes)
        }
        #[cfg(target_os = "linux")]
        {
            linux_write_relative(&self.root, &relative, bytes)
        }
        #[cfg(target_os = "android")]
        {
            android_native::write_relative(&self.root, &relative, bytes)
        }
        #[cfg(target_os = "macos")]
        {
            macos_native::write_relative(&self.root, &relative, bytes)
        }
        #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "macos", windows)))]
        {
            let _ = relative;
            Err(FilesystemError::UnsupportedPlatform)
        }
    }

    fn relative_target(&self, target: &Path) -> Result<PathBuf, FilesystemError> {
        #[cfg(windows)]
        let relative = if target.is_absolute() {
            // Canonicalize only the parent directory, then retain the final
            // component as a name. This normalizes Windows 8.3 aliases and
            // casing without resolving a final-component reparse point.
            let parent = target.parent().ok_or(FilesystemError::InvalidTarget)?;
            let file_name = target.file_name().ok_or(FilesystemError::InvalidTarget)?;
            let canonical_parent = std::fs::canonicalize(parent)
                .map_err(|_| FilesystemError::InvalidTarget)?;
            let normalized_root = normalize_windows_verbatim_prefix(&self.canonical_root);
            let normalized_parent = normalize_windows_verbatim_prefix(&canonical_parent);
            let relative_parent = normalized_parent
                .strip_prefix(&normalized_root)
                .map_err(|_| FilesystemError::OutsideRoot)?;
            relative_parent.join(file_name)
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
            flags: (libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK) as u64,
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

    #[cfg(target_os = "android")]
    fn open_relative(&self, relative: &Path) -> Result<File, FilesystemError> {
        android_native::open_relative(&self.root, relative)
    }

    #[cfg(target_os = "macos")]
    fn open_relative(&self, relative: &Path) -> Result<File, FilesystemError> {
        macos_native::open_relative(&self.root, relative)
    }

    #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "macos", windows)))]
    fn open_relative(&self, _relative: &Path) -> Result<File, FilesystemError> {
        Err(FilesystemError::UnsupportedPlatform)
    }
}



#[cfg(target_os = "linux")]
struct LinuxTempFileGuard {
    parent_fd: std::os::fd::RawFd,
    name: std::ffi::CString,
    committed: bool,
}

#[cfg(target_os = "linux")]
impl LinuxTempFileGuard {
    fn cleanup(&mut self) -> Result<(), FilesystemError> {
        if self.committed {
            return Ok(());
        }
        let result = unsafe { libc::unlinkat(self.parent_fd, self.name.as_ptr(), 0) };
        if result != 0 {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::ENOENT) {
                return Err(FilesystemError::IoFailure);
            }
        }
        self.committed = true;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
impl Drop for LinuxTempFileGuard {
    fn drop(&mut self) {
        if !self.committed {
            unsafe {
                libc::unlinkat(self.parent_fd, self.name.as_ptr(), 0);
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn linux_openat2(
    directory_fd: std::os::fd::RawFd,
    path: &std::ffi::CString,
    flags: i32,
    mode: u64,
    resolve: u64,
) -> Result<File, std::io::Error> {
    use std::os::fd::FromRawFd;

    #[repr(C)]
    struct OpenHow {
        flags: u64,
        mode: u64,
        resolve: u64,
    }

    let how = OpenHow {
        flags: flags as u64,
        mode,
        resolve,
    };
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            directory_fd,
            path.as_ptr(),
            &how as *const OpenHow,
            std::mem::size_of::<OpenHow>(),
        ) as libc::c_int
    };
    if fd < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

#[cfg(target_os = "linux")]
fn linux_filesystem_error(error: std::io::Error) -> FilesystemError {
    match error.raw_os_error() {
        Some(libc::ELOOP) | Some(libc::EXDEV) | Some(libc::EAGAIN) | Some(libc::EBUSY) => {
            FilesystemError::ResolutionFailed
        }
        Some(libc::ENOSYS) | Some(libc::EPERM) | Some(libc::EINVAL) | Some(libc::EOPNOTSUPP) => {
            FilesystemError::UnsupportedPlatform
        }
        Some(libc::ENOENT) => FilesystemError::NotFound,
        _ => FilesystemError::IoFailure,
    }
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LinuxWriteStage {
    TempCreated,
    Written,
    Synced,
    BeforeRename,
}

#[cfg(target_os = "linux")]
fn linux_write_nested_relative_with_hook<F>(
    root: &File,
    relative: &Path,
    bytes: &[u8],
    mut hook: F,
) -> Result<(), FilesystemError>
where
    F: FnMut(LinuxWriteStage) -> Result<(), FilesystemError>,
{
    use std::ffi::CString;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    const AT_EMPTY_PATH: u32 = 0x1000;
    const AT_RECURSIVE: u32 = 0x8000;
    const OPEN_TREE_CLONE: u32 = 1;
    const MOUNT_ATTR_NOSYMFOLLOW: u64 = 0x0020_0000;
    const RESOLVE: u64 = 0x01 | 0x04 | 0x08;
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    if relative.is_absolute()
        || relative.components().any(|component| {
            !matches!(component, std::path::Component::Normal(_))
        })
    {
        return Err(FilesystemError::InvalidTarget);
    }
    let relative_name =
        CString::new(relative.as_os_str().as_bytes()).map_err(|_| FilesystemError::InvalidTarget)?;

    #[repr(C)]
    struct MountAttr {
        attr_set: u64,
        attr_clr: u64,
        propagation: u64,
        userns_fd: u64,
    }
    let empty = b"\0";
    let mount_fd = unsafe {
        libc::syscall(
            libc::SYS_open_tree,
            root.as_raw_fd(),
            empty.as_ptr() as *const libc::c_char,
            (AT_EMPTY_PATH | AT_RECURSIVE | OPEN_TREE_CLONE | libc::O_CLOEXEC as u32) as libc::c_uint,
        ) as libc::c_int
    };
    if mount_fd < 0 {
        return Err(linux_filesystem_error(std::io::Error::last_os_error()));
    }
    let mount = unsafe { File::from_raw_fd(mount_fd) };
    let attr = MountAttr {
        attr_set: MOUNT_ATTR_NOSYMFOLLOW,
        attr_clr: 0,
        propagation: 0,
        userns_fd: 0,
    };
    let attr_result = unsafe {
        libc::syscall(
            libc::SYS_mount_setattr,
            mount.as_raw_fd(),
            empty.as_ptr() as *const libc::c_char,
            (AT_EMPTY_PATH | AT_RECURSIVE) as libc::c_uint,
            &attr as *const MountAttr,
            std::mem::size_of::<MountAttr>(),
        )
    };
    if attr_result != 0 {
        return Err(linux_filesystem_error(std::io::Error::last_os_error()));
    }

    let root_metadata = mount.metadata().map_err(|_| FilesystemError::IoFailure)?;
    let parent = relative.parent().ok_or(FilesystemError::InvalidTarget)?;
    let parent_name = if parent.as_os_str().is_empty() {
        CString::new(".").expect("static path")
    } else {
        CString::new(parent.as_os_str().as_bytes())
            .map_err(|_| FilesystemError::InvalidTarget)?
    };
    let parent_handle = linux_openat2(
        mount.as_raw_fd(),
        &parent_name,
        libc::O_PATH | libc::O_DIRECTORY | libc::O_CLOEXEC,
        0,
        RESOLVE,
    )
    .map_err(linux_filesystem_error)?;
    if parent_handle.metadata().map_err(|_| FilesystemError::IoFailure)?.dev()
        != root_metadata.dev()
    {
        return Err(FilesystemError::ResolutionFailed);
    }
    let parent_sync_handle = linux_openat2(
        mount.as_raw_fd(),
        &parent_name,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
        0,
        RESOLVE,
    )
    .map_err(linux_filesystem_error)?;

    match linux_openat2(
        mount.as_raw_fd(),
        &relative_name,
        libc::O_PATH | libc::O_CLOEXEC,
        0,
        RESOLVE,
    ) {
        Ok(existing) => {
            let metadata = existing.metadata().map_err(|_| FilesystemError::IoFailure)?;
            if !metadata.is_file() {
                return Err(FilesystemError::NotRegularFile);
            }
            if metadata.nlink() != 1 {
                return Err(FilesystemError::HardLinkedFile);
            }
            if metadata.dev() != root_metadata.dev() {
                return Err(FilesystemError::ResolutionFailed);
            }
        }
        Err(error) if error.raw_os_error() == Some(libc::ENOENT) => {}
        Err(error) => return Err(linux_filesystem_error(error)),
    }

    let mut created = None;
    for _ in 0..16 {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!(".one-write-{}-{nonce}-{counter}.tmp", std::process::id());
        let temp_name = CString::new(name).map_err(|_| FilesystemError::InvalidTarget)?;
        match linux_openat2(
            mount.as_raw_fd(),
            &temp_name,
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0o600,
            RESOLVE,
        ) {
            Ok(file) => {
                created = Some((file, temp_name));
                break;
            }
            Err(error) if error.raw_os_error() == Some(libc::EEXIST) => continue,
            Err(error) => return Err(linux_filesystem_error(error)),
        }
    }
    let (mut temporary_file, temporary_name) =
        created.ok_or(FilesystemError::IoFailure)?;
    let mut guard = LinuxTempFileGuard {
        parent_fd: mount.as_raw_fd(),
        name: temporary_name,
        committed: false,
    };

    let result = (|| {
        hook(LinuxWriteStage::TempCreated)?;
        let metadata = temporary_file.metadata().map_err(|_| FilesystemError::IoFailure)?;
        if !metadata.is_file() || metadata.nlink() != 1 || metadata.dev() != root_metadata.dev() {
            return Err(FilesystemError::ResolutionFailed);
        }
        temporary_file.write_all(bytes).map_err(|_| FilesystemError::IoFailure)?;
        hook(LinuxWriteStage::Written)?;
        temporary_file.sync_all().map_err(|_| FilesystemError::IoFailure)?;
        hook(LinuxWriteStage::Synced)?;
        hook(LinuxWriteStage::BeforeRename)?;

        let rename_result = unsafe {
            libc::syscall(
                libc::SYS_renameat2,
                mount.as_raw_fd(),
                guard.name.as_ptr(),
                mount.as_raw_fd(),
                relative_name.as_ptr(),
                0_u32,
            )
        };
        if rename_result != 0 {
            return Err(linux_filesystem_error(std::io::Error::last_os_error()));
        }
        // Persist the entry in the actual containing directory.
        parent_sync_handle.sync_all().map_err(|_| FilesystemError::IoFailure)?;
        guard.committed = true;
        Ok(())
    })();
    if result.is_err() {
        guard.cleanup()?;
    }
    result
}

fn linux_write_relative(
    root: &File,
    relative: &Path,
    bytes: &[u8],
) -> Result<(), FilesystemError> {
    linux_write_relative_with_hook(root, relative, bytes, |_| Ok(()))
}

fn linux_write_relative_with_hook<F>(
    root: &File,
    relative: &Path,
    bytes: &[u8],
    mut hook: F,
) -> Result<(), FilesystemError>
where
    F: FnMut(LinuxWriteStage) -> Result<(), FilesystemError>,
{
    use std::ffi::CString;
    use std::os::fd::AsRawFd;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    const RESOLVE_NO_XDEV: u64 = 0x01;
    const RESOLVE_NO_SYMLINKS: u64 = 0x04;
    const RESOLVE_BENEATH: u64 = 0x08;
    const RESOLVE: u64 = RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV;
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    let parent = relative
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let leaf = relative.file_name().ok_or(FilesystemError::InvalidTarget)?;
    let leaf_name = CString::new(leaf.as_bytes()).map_err(|_| FilesystemError::InvalidTarget)?;

    // Direct-child writes use the retained root descriptor. Nested writes
    // use a detached recursive mount clone with NOSYMFOLLOW and root-relative
    // openat2/renameat2 resolution so a moved parent cannot redirect the
    // replacement outside the retained tree. If the kernel cannot provide
    // those primitives or a resolution check fails, the nested write fails
    // closed rather than falling back to an ordinary parent-dirfd rename.
    if parent != Path::new(".") {
        return linux_write_nested_relative_with_hook(root, relative, bytes, |_| Ok(()));
    }
    let parent_fd = root.as_raw_fd();
    let root_metadata = root.metadata().map_err(|_| FilesystemError::IoFailure)?;

    // Reject an existing symlink, mount point, non-regular file, or hard link.
    // The later rename is directory-handle-relative and never follows the
    // destination leaf; a raced leaf replacement cannot redirect the write.
    match linux_openat2(
        parent_fd,
        &leaf_name,
        libc::O_PATH | libc::O_CLOEXEC,
        0,
        RESOLVE,
    ) {
        Ok(existing) => {
            let metadata = existing.metadata().map_err(|_| FilesystemError::IoFailure)?;
            if !metadata.is_file() {
                return Err(FilesystemError::NotRegularFile);
            }
            if metadata.nlink() != 1 {
                return Err(FilesystemError::HardLinkedFile);
            }
            if metadata.dev() != root_metadata.dev() {
                return Err(FilesystemError::ResolutionFailed);
            }
        }
        Err(error) if error.raw_os_error() == Some(libc::ENOENT) => {}
        Err(error) => return Err(linux_filesystem_error(error)),
    }

    let mut created = None;
    for _ in 0..16 {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!(".one-write-{}-{nonce}-{counter}.tmp", std::process::id());
        let temp_name = CString::new(name).map_err(|_| FilesystemError::InvalidTarget)?;
        if temp_name.as_bytes() == leaf_name.as_bytes() {
            continue;
        }
        match linux_openat2(
            parent_fd,
            &temp_name,
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0o600,
            RESOLVE,
        ) {
            Ok(file) => {
                created = Some((file, temp_name));
                break;
            }
            Err(error) if error.raw_os_error() == Some(libc::EEXIST) => continue,
            Err(error) => return Err(linux_filesystem_error(error)),
        }
    }
    let (mut temporary_file, temporary_name) = created.ok_or(FilesystemError::IoFailure)?;
    let mut guard = LinuxTempFileGuard {
        parent_fd,
        name: temporary_name,
        committed: false,
    };

    let result = (|| {
        hook(LinuxWriteStage::TempCreated)?;
        let metadata = temporary_file
            .metadata()
            .map_err(|_| FilesystemError::IoFailure)?;
        if !metadata.is_file() || metadata.nlink() != 1 || metadata.dev() != root_metadata.dev() {
            return Err(FilesystemError::ResolutionFailed);
        }
        temporary_file
            .write_all(bytes)
            .map_err(|_| FilesystemError::IoFailure)?;
        hook(LinuxWriteStage::Written)?;
        temporary_file
            .sync_all()
            .map_err(|_| FilesystemError::IoFailure)?;
        hook(LinuxWriteStage::Synced)?;
        hook(LinuxWriteStage::BeforeRename)?;

        let rename_result = unsafe {
            libc::renameat(
                parent_fd,
                guard.name.as_ptr(),
                parent_fd,
                leaf_name.as_ptr(),
            )
        };
        if rename_result != 0 {
            return Err(linux_filesystem_error(std::io::Error::last_os_error()));
        }
        root.sync_all().map_err(|_| FilesystemError::IoFailure)?;
        guard.committed = true;
        Ok(())
    })();

    if result.is_err() {
        guard.cleanup()?;
    }
    result
}

#[cfg(target_os = "android")]
mod android_native {
    use super::FilesystemError;
    use std::ffi::CString;
    use std::fs::File;
    use std::io::Write;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;
    use std::path::{Component, Path};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    const AT_EMPTY_PATH: i32 = 0x1000;
    const STATX_MNT_ID: u32 = 0x1000;
    const O_PATH: i32 = 0x0020_0000;
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    // Linux/Android statx ABI. The trailing reserved bytes keep the structure
    // at the kernel-defined 256-byte size on 32-bit and 64-bit Android ABIs.
    #[repr(C)]
    #[derive(Default)]
    struct StatxTimestamp { seconds: i64, nanoseconds: u32, reserved: i32 }
    #[repr(C)]
    #[derive(Default)]
    struct Statx {
        mask: u32, block_size: u32, attributes: u64,
        links: u32, uid: u32, gid: u32, mode: u16, spare0: u16,
        inode: u64, size: u64, blocks: u64, attributes_mask: u64,
        atime: StatxTimestamp, btime: StatxTimestamp,
        ctime: StatxTimestamp, mtime: StatxTimestamp,
        rdev_major: u32, rdev_minor: u32, dev_major: u32, dev_minor: u32,
        mount_id: u64, dio_mem_align: u32, dio_offset_align: u32,
        subvolume: u64, atomic_min: u32, atomic_max: u32,
        atomic_segments: u32, reserved: [u8; 76],
    }
    const _: [(); 256] = [(); std::mem::size_of::<Statx>()];

    pub(super) fn mount_id(file: &File) -> Result<u64, FilesystemError> {
        let empty = b"\0";
        let mut info = Statx::default();
        let result = unsafe {
            libc::syscall(libc::SYS_statx, file.as_raw_fd(), empty.as_ptr() as *const libc::c_char,
                AT_EMPTY_PATH, STATX_MNT_ID, &mut info as *mut Statx)
        };
        if result != 0 || info.mask & STATX_MNT_ID == 0 {
            return Err(FilesystemError::UnsupportedPlatform);
        }
        Ok(info.mount_id)
    }

    fn map_error(error: std::io::Error) -> FilesystemError {
        match error.raw_os_error() {
            Some(libc::ELOOP) | Some(libc::EXDEV) | Some(libc::EBUSY) | Some(libc::ENOTDIR) => FilesystemError::ResolutionFailed,
            Some(libc::ENOENT) => FilesystemError::NotFound,
            Some(libc::ENOSYS) | Some(libc::EINVAL) => FilesystemError::UnsupportedPlatform,
            _ => FilesystemError::IoFailure,
        }
    }

    fn openat(directory: &File, name: &CString, flags: i32, mode: u32) -> Result<File, FilesystemError> {
        let fd = unsafe { libc::openat(directory.as_raw_fd(), name.as_ptr(), flags, mode) };
        if fd < 0 { return Err(map_error(std::io::Error::last_os_error())); }
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn parent_and_leaf(root: &File, relative: &Path) -> Result<(File, CString), FilesystemError> {
        let mut components = relative.components();
        let Some(Component::Normal(leaf)) = components.next() else {
            return Err(FilesystemError::InvalidTarget);
        };
        // POSIX directory descriptors do not pin a directory inside the
        // retained root: an attacker may rename an opened parent directory
        // outside the root before the leaf operation. Until this backend has
        // a native anti-move primitive, nested paths must fail closed.
        if components.next().is_some() {
            return Err(FilesystemError::UnsupportedPlatform);
        }
        let leaf = CString::new(leaf.as_bytes()).map_err(|_| FilesystemError::InvalidTarget)?;
        let parent = root.try_clone().map_err(|_| FilesystemError::IoFailure)?;
        Ok((parent, leaf))
    }

    fn create_temp(parent: &File, name: &CString) -> Result<File, std::io::Error> {
        let fd = unsafe {
            libc::openat(parent.as_raw_fd(), name.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC | libc::O_NOFOLLOW, 0o600)
        };
        if fd < 0 { return Err(std::io::Error::last_os_error()); }
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    pub(super) fn open_relative(root: &File, relative: &Path) -> Result<File, FilesystemError> {
        let (parent, leaf) = parent_and_leaf(root, relative)?;
        let file = openat(&parent, &leaf,
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK, 0)?;
        let metadata = file.metadata().map_err(|_| FilesystemError::IoFailure)?;
        if !metadata.is_file() { return Err(FilesystemError::NotRegularFile); }
        if metadata.nlink() != 1 { return Err(FilesystemError::HardLinkedFile); }
        if metadata.dev() != root.metadata().map_err(|_| FilesystemError::IoFailure)?.dev()
            || mount_id(root)? != mount_id(&file)? {
            return Err(FilesystemError::ResolutionFailed);
        }
        Ok(file)
    }

    pub(super) fn write_relative(root: &File, relative: &Path, bytes: &[u8]) -> Result<(), FilesystemError> {
        let (parent, leaf) = parent_and_leaf(root, relative)?;
        let root_mount = mount_id(root)?;
        let root_dev = root.metadata().map_err(|_| FilesystemError::IoFailure)?.dev();
        match openat(&parent, &leaf, O_PATH | libc::O_CLOEXEC | libc::O_NOFOLLOW, 0) {
            Ok(existing) => {
                let metadata = existing.metadata().map_err(|_| FilesystemError::IoFailure)?;
                if metadata.file_type().is_symlink() { return Err(FilesystemError::ResolutionFailed); }
                if !metadata.is_file() { return Err(FilesystemError::NotRegularFile); }
                if metadata.nlink() != 1 { return Err(FilesystemError::HardLinkedFile); }
                if metadata.dev() != root_dev || mount_id(&existing)? != root_mount {
                    return Err(FilesystemError::ResolutionFailed);
                }
            }
            Err(FilesystemError::NotFound) => {},
            Err(error) => return Err(error),
        }
        let mut created = None;
        for _ in 0..16 {
            let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
            let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
            let name = CString::new(format!(".one-write-{}-{nonce}-{counter}.tmp", std::process::id()))
                .map_err(|_| FilesystemError::InvalidTarget)?;
            match create_temp(&parent, &name) {
                Ok(file) => { created = Some((file, name)); break; },
                Err(error) if error.raw_os_error() == Some(libc::EEXIST) => continue,
                Err(error) => return Err(map_error(error)),
            }
        }
        let (mut temp, temp_name) = created.ok_or(FilesystemError::IoFailure)?;
        let result = (|| {
            let metadata = temp.metadata().map_err(|_| FilesystemError::IoFailure)?;
            if !metadata.is_file() || metadata.nlink() != 1 || metadata.dev() != root_dev
                || mount_id(&temp)? != root_mount {
                return Err(FilesystemError::ResolutionFailed);
            }
            temp.write_all(bytes).map_err(|_| FilesystemError::IoFailure)?;
            temp.sync_all().map_err(|_| FilesystemError::IoFailure)?;
            let renamed = unsafe { libc::renameat(parent.as_raw_fd(), temp_name.as_ptr(), parent.as_raw_fd(), leaf.as_ptr()) };
            if renamed != 0 { return Err(map_error(std::io::Error::last_os_error())); }
            parent.sync_all().map_err(|_| FilesystemError::IoFailure)?;
            Ok(())
        })();
        if result.is_err() {
            let removed = unsafe { libc::unlinkat(parent.as_raw_fd(), temp_name.as_ptr(), 0) };
            if removed != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ENOENT) {
                return Err(FilesystemError::IoFailure);
            }
        }
        result
    }
}

#[cfg(target_os = "macos")]
mod macos_native {
    use super::FilesystemError;
    use std::ffi::CString;
    use std::fs::File;
    use std::io::Write;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;
    use std::path::{Component, Path};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn map_error(error: std::io::Error) -> FilesystemError {
        match error.raw_os_error() {
            Some(libc::ELOOP) | Some(libc::EXDEV) | Some(libc::EBUSY) | Some(libc::ENOTDIR) => FilesystemError::ResolutionFailed,
            Some(libc::ENOENT) => FilesystemError::NotFound,
            _ => FilesystemError::IoFailure,
        }
    }

    fn openat(directory: &File, name: &CString, flags: i32, mode: u32) -> Result<File, FilesystemError> {
        let fd = unsafe { libc::openat(directory.as_raw_fd(), name.as_ptr(), flags, mode) };
        if fd < 0 { return Err(map_error(std::io::Error::last_os_error())); }
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn verify_same_mount(root: &File, file: &File) -> Result<(), FilesystemError> {
        let mut root_info: libc::statfs = unsafe { std::mem::zeroed() };
        let mut file_info: libc::statfs = unsafe { std::mem::zeroed() };
        // SAFETY: both descriptors are live and each statfs output points to
        // a writable statfs value with the platform ABI layout.
        if unsafe { libc::fstatfs(root.as_raw_fd(), &mut root_info) } != 0
            || unsafe { libc::fstatfs(file.as_raw_fd(), &mut file_info) } != 0 {
            return Err(FilesystemError::IoFailure);
        }
        if root_info.f_fsid.val != file_info.f_fsid.val
            || root_info.f_mntonname != file_info.f_mntonname {
            return Err(FilesystemError::ResolutionFailed);
        }
        Ok(())
    }

    fn parent_and_leaf(root: &File, relative: &Path) -> Result<(File, CString), FilesystemError> {
        let components: Vec<_> = relative.components().collect();
        if components.is_empty() { return Err(FilesystemError::InvalidTarget); }
        // POSIX directory descriptors do not pin a directory inside the
        // retained root: an attacker may rename an opened parent directory
        // outside the root before the leaf operation. Until this backend has
        // a native anti-move primitive, nested paths must fail closed.
        if components.len() != 1 { return Err(FilesystemError::UnsupportedPlatform); }
        let mut parent = root.try_clone().map_err(|_| FilesystemError::IoFailure)?;
        for component in &components[..components.len() - 1] {
            let Component::Normal(name) = component else { return Err(FilesystemError::InvalidTarget); };
            let name = CString::new(name.as_bytes()).map_err(|_| FilesystemError::InvalidTarget)?;
            let next = openat(&parent, &name,
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK, 0)?;
            let metadata = next.metadata().map_err(|_| FilesystemError::IoFailure)?;
            if !metadata.is_dir() { return Err(FilesystemError::NotRegularFile); }
            verify_same_mount(root, &next)?;
            parent = next;
        }
        let Component::Normal(leaf) = components[components.len() - 1] else {
            return Err(FilesystemError::InvalidTarget);
        };
        let leaf = CString::new(leaf.as_bytes()).map_err(|_| FilesystemError::InvalidTarget)?;
        Ok((parent, leaf))
    }

    fn create_temp(parent: &File, name: &CString) -> Result<File, std::io::Error> {
        let fd = unsafe {
            libc::openat(parent.as_raw_fd(), name.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC | libc::O_NOFOLLOW, 0o600)
        };
        if fd < 0 { return Err(std::io::Error::last_os_error()); }
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    pub(super) fn open_relative(root: &File, relative: &Path) -> Result<File, FilesystemError> {
        let (parent, leaf) = parent_and_leaf(root, relative)?;
        let file = openat(&parent, &leaf,
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK, 0)?;
        let metadata = file.metadata().map_err(|_| FilesystemError::IoFailure)?;
        if !metadata.is_file() { return Err(FilesystemError::NotRegularFile); }
        if metadata.nlink() != 1 { return Err(FilesystemError::HardLinkedFile); }
        verify_same_mount(root, &file)?;
        Ok(file)
    }

    pub(super) fn write_relative(root: &File, relative: &Path, bytes: &[u8]) -> Result<(), FilesystemError> {
        let (parent, leaf) = parent_and_leaf(root, relative)?;
        match openat(&parent, &leaf,
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK, 0) {
            Ok(existing) => {
                let metadata = existing.metadata().map_err(|_| FilesystemError::IoFailure)?;
                if !metadata.is_file() { return Err(FilesystemError::NotRegularFile); }
                if metadata.nlink() != 1 { return Err(FilesystemError::HardLinkedFile); }
                verify_same_mount(root, &existing)?;
            }
            Err(FilesystemError::NotFound) => {},
            Err(error) => return Err(error),
        }
        let mut created = None;
        for _ in 0..16 {
            let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
            let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
            let name = CString::new(format!(".one-write-{}-{nonce}-{counter}.tmp", std::process::id()))
                .map_err(|_| FilesystemError::InvalidTarget)?;
            match create_temp(&parent, &name) {
                Ok(file) => { created = Some((file, name)); break; },
                Err(error) if error.raw_os_error() == Some(libc::EEXIST) => continue,
                Err(error) => return Err(map_error(error)),
            }
        }
        let (mut temp, temp_name) = created.ok_or(FilesystemError::IoFailure)?;
        let result = (|| {
            let metadata = temp.metadata().map_err(|_| FilesystemError::IoFailure)?;
            if !metadata.is_file() || metadata.nlink() != 1 { return Err(FilesystemError::ResolutionFailed); }
            verify_same_mount(root, &temp)?;
            temp.write_all(bytes).map_err(|_| FilesystemError::IoFailure)?;
            temp.sync_all().map_err(|_| FilesystemError::IoFailure)?;
            let renamed = unsafe {
                libc::renameat(parent.as_raw_fd(), temp_name.as_ptr(), parent.as_raw_fd(), leaf.as_ptr())
            };
            if renamed != 0 { return Err(map_error(std::io::Error::last_os_error())); }
            parent.sync_all().map_err(|_| FilesystemError::IoFailure)?;
            Ok(())
        })();
        if result.is_err() {
            let removed = unsafe { libc::unlinkat(parent.as_raw_fd(), temp_name.as_ptr(), 0) };
            if removed != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ENOENT) {
                return Err(FilesystemError::IoFailure);
            }
        }
        result
    }
}
#[cfg(windows)]
mod windows_native {
    use super::FilesystemError;
    use std::ffi::c_void;
    use std::fs::File;
    use std::io::Write;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    use std::path::{Component, Path};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileInformationByHandle, SetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
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
    const FILE_CREATE: u32 = 2;
    const FILE_OPEN_IF: u32 = 3;
    const DELETE_ACCESS: u32 = 0x0001_0000;
    const FILE_DIRECTORY_FILE: u32 = 0x0000_0001;
    const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;
    const FILE_NON_DIRECTORY_FILE: u32 = 0x0000_0040;
    const FILE_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x0000_0080;

    const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x0000_0010;
    const FILE_ATTRIBUTE_DEVICE: u32 = 0x0000_0040;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

    const STATUS_REPARSE_POINT_ENCOUNTERED: u32 = 0xC000_050B;
    const STATUS_OBJECT_NAME_NOT_FOUND: u32 = 0xC000_0034;
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
        fn NtSetInformationFile(
            file_handle: HANDLE,
            io_status_block: *mut IoStatusBlock,
            file_information: *const c_void,
            length: u32,
            file_information_class: i32,
        ) -> i32;
    }

    fn status_error(status: i32) -> FilesystemError {
        #[cfg(test)]
        eprintln!("NtCreateFile returned NTSTATUS 0x{:08X}", status as u32);
        match status as u32 {
            STATUS_REPARSE_POINT_ENCOUNTERED => FilesystemError::ResolutionFailed,
            STATUS_OBJECT_NAME_NOT_FOUND => FilesystemError::NotFound,
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

    pub fn canonical_root_path(path: &Path) -> Result<std::path::PathBuf, FilesystemError> {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|_| FilesystemError::InvalidRoot)?
                .join(path)
        };
        let Some(leaf) = absolute.file_name() else {
            // Volume roots have no final directory component to follow.
            return std::fs::canonicalize(&absolute).map_err(|_| FilesystemError::InvalidRoot);
        };
        let parent = absolute.parent().ok_or(FilesystemError::InvalidRoot)?;
        let canonical_parent =
            std::fs::canonicalize(parent).map_err(|_| FilesystemError::InvalidRoot)?;
        // Preserve the final component verbatim so open_root can reject a
        // reparse point instead of canonicalizing through it.
        Ok(canonical_parent.join(leaf))
    }

    pub fn root_matches_path(root: &File, path: &Path) -> Result<bool, FilesystemError> {
        use std::os::windows::fs::MetadataExt;

        let handle_info = file_info(root)?;
        let path_info = std::fs::metadata(path).map_err(|_| FilesystemError::InvalidRoot)?;
        let handle_index = ((handle_info.nFileIndexHigh as u64) << 32)
            | handle_info.nFileIndexLow as u64;
        Ok(path_info.volume_serial_number() == Some(handle_info.dwVolumeSerialNumber)
            && path_info.file_index() == Some(handle_index))
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
                #[cfg(test)]
                eprintln!("opened component is a reparse point: attrs=0x{:08X}", info.dwFileAttributes);
                return Err(FilesystemError::ResolutionFailed);
            }
            if info.dwVolumeSerialNumber != root_info.dwVolumeSerialNumber {
                #[cfg(test)]
                eprintln!("opened component volume mismatch: root=0x{:08X} child=0x{:08X}", root_info.dwVolumeSerialNumber, info.dwVolumeSerialNumber);
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

    #[repr(C)]
    struct FileRenameInfo {
        replace_if_exists: u8,
        root_directory: HANDLE,
        file_name_length: u32,
        file_name: [u16; 1],
    }

    #[repr(C)]
    struct FileDispositionInfo {
        delete_file: u8,
    }

    fn mark_for_delete(file: &File) -> Result<(), FilesystemError> {
        let mut info = FileDispositionInfo { delete_file: 1 };
        let ok = unsafe {
            SetFileInformationByHandle(
                file.as_raw_handle() as HANDLE,
                4 as _,
                (&mut info as *mut FileDispositionInfo).cast::<c_void>(),
                std::mem::size_of::<FileDispositionInfo>() as u32,
            )
        };
        if ok == 0 { return Err(FilesystemError::IoFailure); }
        Ok(())
    }

    struct TempFileGuard {
        file: File,
        committed: bool,
    }

    impl TempFileGuard {
        fn cleanup(&mut self) -> Result<(), FilesystemError> {
            if self.committed { return Ok(()); }
            mark_for_delete(&self.file)?;
            self.committed = true;
            Ok(())
        }
    }

    impl Drop for TempFileGuard {
        fn drop(&mut self) {
            if !self.committed {
                let _ = mark_for_delete(&self.file);
            }
        }
    }

    fn rename_relative(file: &File, leaf: &[u16]) -> Result<(), FilesystemError> {
        if leaf.is_empty() { return Err(FilesystemError::InvalidTarget); }
        let name_bytes = leaf.len().checked_mul(2).ok_or(FilesystemError::InvalidTarget)?;
        let name_offset = std::mem::offset_of!(FileRenameInfo, file_name);
        let buffer_length = name_offset
            .checked_add(name_bytes)
            .and_then(|length| length.checked_add(2))
            .ok_or(FilesystemError::InvalidTarget)?;
        let buffer_length_u32 = u32::try_from(buffer_length).map_err(|_| FilesystemError::InvalidTarget)?;
        let word_count = buffer_length.checked_add(7).ok_or(FilesystemError::InvalidTarget)? / 8;
        let mut storage = vec![0u64; word_count];
        let base = storage.as_mut_ptr().cast::<u8>();
        let root_offset = std::mem::offset_of!(FileRenameInfo, root_directory);
        let length_offset = std::mem::offset_of!(FileRenameInfo, file_name_length);

        unsafe {
            base.write(1); // ReplaceIfExists = TRUE.
            std::ptr::write_unaligned(base.add(root_offset).cast::<HANDLE>(), std::ptr::null_mut());
            std::ptr::write_unaligned(base.add(length_offset).cast::<u32>(), name_bytes as u32);
            let destination = base.add(name_offset).cast::<u16>();
            std::ptr::copy_nonoverlapping(leaf.as_ptr(), destination, leaf.len());
            destination.add(leaf.len()).write(0);
        }

        let mut io_status = IoStatusBlock {
            value: IoStatusValue { status: 0 },
            information: 0,
        };
        let status = unsafe {
            NtSetInformationFile(
                file.as_raw_handle() as HANDLE,
                &mut io_status,
                base.cast::<c_void>(),
                buffer_length_u32,
                10, // FileRenameInformation
            )
        };
        if status < 0 {
            #[cfg(test)]
            eprintln!(
                "NtSetInformationFile(FileRenameInformation) failed: NTSTATUS=0x{:08X}, leaf={:?}",
                status as u32,
                String::from_utf16_lossy(leaf)
            );
            return Err(status_error(status));
        }
        Ok(())
    }

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    pub fn write_relative(root: &File, relative: &Path, bytes: &[u8]) -> Result<(), FilesystemError> {
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
        let mut leaf_wide = None;

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
            if index + 1 == components.len() {
                leaf_wide = Some(wide);
                break;
            }

            let directory = open_named(
                &wide,
                parent_handle,
                OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
                FILE_LIST_DIRECTORY | FILE_TRAVERSE | FILE_READ_ATTRIBUTES | SYNCHRONIZE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                FILE_OPEN,
                FILE_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_OPEN_REPARSE_POINT,
            )?;
            let info = file_info(&directory)?;
            if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
                || info.dwVolumeSerialNumber != root_info.dwVolumeSerialNumber
                || info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0
            {
                return Err(FilesystemError::ResolutionFailed);
            }
            parents.push(directory);
            parent_handle = parents.last().expect("parent handle was just retained").as_raw_handle() as HANDLE;
        }

        let leaf_wide = leaf_wide.ok_or(FilesystemError::InvalidTarget)?;

        // Inspect an existing leaf without following it. The handle is closed
        // before replacement; the final rename uses a simple same-directory
        // name relative to the source handle, with RootDirectory = NULL.
        match open_named(
            &leaf_wide,
            parent_handle,
            OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
            FILE_READ_ATTRIBUTES | SYNCHRONIZE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            FILE_OPEN,
            FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_OPEN_REPARSE_POINT,
        ) {
            Ok(existing) => {
                let info = file_info(&existing)?;
                if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                    return Err(FilesystemError::ResolutionFailed);
                }
                if info.dwVolumeSerialNumber != root_info.dwVolumeSerialNumber {
                    return Err(FilesystemError::ResolutionFailed);
                }
                if info.dwFileAttributes & (FILE_ATTRIBUTE_DIRECTORY | FILE_ATTRIBUTE_DEVICE) != 0 {
                    return Err(FilesystemError::NotRegularFile);
                }
                if info.nNumberOfLinks != 1 {
                    return Err(FilesystemError::HardLinkedFile);
                }
                drop(existing);
            }
            Err(FilesystemError::NotFound) => {}
            Err(error) => return Err(error),
        }

        let mut created = None;
        for _ in 0..8 {
            let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
            let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
            let name = format!(".one-write-{}-{nonce}-{counter}.tmp", std::process::id());
            let wide: Vec<u16> = name.encode_utf16().collect();
            match open_named(
                &wide,
                parent_handle,
                OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
                FILE_WRITE_DATA | FILE_READ_ATTRIBUTES | SYNCHRONIZE | DELETE_ACCESS,
                FILE_SHARE_READ,
                FILE_CREATE,
                FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_OPEN_REPARSE_POINT,
            ) {
                Ok(file) => { created = Some(file); break; }
                Err(FilesystemError::IoFailure) => continue,
                Err(error) => return Err(error),
            }
        }
        let file = created.ok_or(FilesystemError::IoFailure)?;
        let mut temporary = TempFileGuard { file, committed: false };

        let result = (|| {
            let info = file_info(&temporary.file)?;
            if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
                || info.dwVolumeSerialNumber != root_info.dwVolumeSerialNumber
                || info.dwFileAttributes & (FILE_ATTRIBUTE_DIRECTORY | FILE_ATTRIBUTE_DEVICE) != 0
                || info.nNumberOfLinks != 1
            {
                return Err(FilesystemError::ResolutionFailed);
            }
            temporary.file.write_all(bytes).map_err(|_| FilesystemError::IoFailure)?;
            temporary.file.flush().map_err(|_| FilesystemError::IoFailure)?;
            rename_relative(&temporary.file, &leaf_wide)?;
            temporary.committed = true;
            Ok(())
        })();

        if result.is_err() && !temporary.committed {
            temporary.cleanup()?;
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::os::windows::io::AsRawHandle;

        #[test]
        fn retained_parent_handle_blocks_directory_rename() {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock must be after Unix epoch")
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "one-win-parent-rename-{}-{nonce}",
                std::process::id()
            ));
            let nested = root.join("nested");
            let moved = root.with_extension("moved");
            std::fs::create_dir_all(&nested).unwrap();
            std::fs::write(nested.join("sentinel.txt"), b"UNCHANGED").unwrap();

            let root_handle = open_root(&root).unwrap();
            let component: Vec<u16> = "nested".encode_utf16().collect();
            // These are the same parent-handle share flags used by write_relative:
            // FILE_SHARE_DELETE is deliberately absent so a concurrent rename
            // cannot move the retained parent outside the trusted root.
            let retained_parent = open_named(
                &component,
                root_handle.as_raw_handle() as HANDLE,
                OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
                FILE_LIST_DIRECTORY | FILE_TRAVERSE | FILE_READ_ATTRIBUTES | SYNCHRONIZE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                FILE_OPEN,
                FILE_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_OPEN_REPARSE_POINT,
            )
            .unwrap();
            drop(root_handle);

            let rename_while_retained = std::fs::rename(&nested, &moved);
            assert!(
                rename_while_retained.is_err(),
                "Windows allowed the retained parent directory to move while its no-delete-share handle was open"
            );
            assert!(nested.is_dir());
            assert!(!moved.exists());
            assert_eq!(std::fs::read(nested.join("sentinel.txt")).unwrap(), b"UNCHANGED");

            drop(retained_parent);
            std::fs::rename(&nested, &moved)
                .expect("directory rename should succeed after the retained handle closes");
            assert!(moved.is_dir());
            std::fs::remove_dir_all(&moved).unwrap();
            std::fs::remove_dir_all(&root).unwrap();
        }
    }
}

#[cfg(all(test, windows))]
mod windows_root_tests {
    use super::*;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn authority_rejects_reparse_root_without_following_final_component() {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let base = std::env::temp_dir().join(format!(
            "one-root-reparse-{}-{nonce}",
            std::process::id()
        ));
        let target = base.join("target");
        let alias = base.join("alias");
        std::fs::create_dir_all(&target).unwrap();

        let output = Command::new("cmd.exe")
            .args([
                "/C",
                "mklink",
                "/J",
                alias.to_str().unwrap(),
                target.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "could not create junction fixture: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        assert_eq!(
            FilesystemAuthority::open(&alias, 1024, 1024).err(),
            Some(FilesystemError::InvalidRoot),
            "the trusted root must reject a final-component reparse point"
        );

        std::fs::remove_dir(alias).unwrap();
        std::fs::remove_dir_all(base).unwrap();
    }
}

#[cfg(all(test, target_os = "linux"))]
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
    fn authority_rejects_symlink_root_without_following_final_component() {
        use std::os::unix::fs::symlink;

        let base = std::env::temp_dir().join(format!(
            "one-fs-root-symlink-{}",
            std::process::id()
        ));
        let root = base.join("root");
        let alias = base.join("root-alias");
        std::fs::create_dir_all(&root).unwrap();
        symlink(&root, &alias).unwrap();

        assert_eq!(
            FilesystemAuthority::open(&alias, 1024, 1024).err(),
            Some(FilesystemError::InvalidRoot),
            "the trusted root must not follow a final symlink"
        );

        std::fs::remove_file(alias).unwrap();
        std::fs::remove_dir_all(base).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn authority_atomic_write_failures_cleanup_temp_and_preserve_target() {
        let root = std::env::temp_dir().join(format!(
            "one-fs-write-fault-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let target = root.join("target.txt");
        std::fs::write(&target, b"KEEP_ORIGINAL").unwrap();

        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        for fail_at in [
            LinuxWriteStage::TempCreated,
            LinuxWriteStage::Written,
            LinuxWriteStage::Synced,
            LinuxWriteStage::BeforeRename,
        ] {
            let result = linux_write_relative_with_hook(
                &authority.root,
                std::path::Path::new("target.txt"),
                b"REPLACEMENT",
                |stage| {
                    if stage == fail_at {
                        Err(FilesystemError::IoFailure)
                    } else {
                        Ok(())
                    }
                },
            );

            assert_eq!(result, Err(FilesystemError::IoFailure));
            assert_eq!(std::fs::read(&target).unwrap(), b"KEEP_ORIGINAL");
            let entries = std::fs::read_dir(&root)
                .unwrap()
                .map(|entry| entry.unwrap().file_name())
                .collect::<Vec<_>>();
            assert_eq!(
                entries,
                vec![std::ffi::OsString::from("target.txt")],
                "temporary sibling leaked after injected failure at {fail_at:?}"
            );
        }

        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn nested_atomic_write_uses_detached_mount_and_rejects_parent_move() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let base = std::env::temp_dir().join(format!(
            "one-fs-nested-mount-{}-{nonce}",
            std::process::id()
        ));
        let root = base.join("root");
        let nested = root.join("nested");
        let outside = base.join("outside");
        let target = nested.join("target.txt");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(&target, b"INITIAL").unwrap();

        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();

        // Environments without CAP_SYS_ADMIN or open_tree/mount_setattr keep
        // nested writes fail-closed. The privileged Linux path is exercised
        // below whenever the kernel permits detached mount cloning.
        let success = linux_write_nested_relative_with_hook(
            &authority.root,
            Path::new("nested/target.txt"),
            b"UPDATED",
            |_| Ok(()),
        );
        if success == Err(FilesystemError::UnsupportedPlatform) {
            drop(authority);
            std::fs::remove_dir_all(base).unwrap();
            return;
        }
        success.expect("detached-mount nested write should succeed when supported");
        assert_eq!(std::fs::read(&target).unwrap(), b"UPDATED");

        std::fs::write(&target, b"KEEP_ON_PARENT_MOVE").unwrap();
        let moved_to = outside.join("moved");
        let mut moved = false;
        let result = linux_write_nested_relative_with_hook(
            &authority.root,
            Path::new("nested/target.txt"),
            b"MUST_NOT_ESCAPE",
            |stage| {
                if stage == LinuxWriteStage::BeforeRename {
                    std::fs::rename(&nested, &moved_to).unwrap();
                    moved = true;
                }
                Ok(())
            },
        );
        assert!(moved, "the test must move the retained parent before commit");
        assert!(
            matches!(
                result,
                Err(FilesystemError::NotFound | FilesystemError::ResolutionFailed)
            ),
            "a parent moved outside the root must make the final root-relative rename fail: {result:?}"
        );
        assert_eq!(
            std::fs::read(moved_to.join("target.txt")).unwrap(),
            b"KEEP_ON_PARENT_MOVE"
        );
        std::fs::rename(&moved_to, &nested).unwrap();
        let root_entries = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>();
        assert_eq!(root_entries, vec![std::ffi::OsString::from("nested")]);
        drop(authority);
        std::fs::remove_dir_all(base).unwrap();
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
    fn authority_rejects_fifo_without_blocking() {
        use std::os::unix::ffi::OsStrExt;

        let root = std::env::temp_dir().join(format!("one-fs-authority-fifo-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let fifo = root.join("pipe");
        let fifo_name = std::ffi::CString::new(fifo.as_os_str().as_bytes()).unwrap();
        // SAFETY: mkfifo receives a valid NUL-terminated path and a restrictive mode.
        assert_eq!(unsafe { libc::mkfifo(fifo_name.as_ptr(), 0o600) }, 0);

        let authority = FilesystemAuthority::open(&root, 32, 32).unwrap();
        assert_eq!(authority.read("pipe"), Err(FilesystemError::NotRegularFile));

        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn authority_enforces_read_and_write_limits_without_partial_replacement() {
        let root = std::env::temp_dir().join(format!("one-fs-authority-limits-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let target = root.join("inside.txt");
        std::fs::write(&target, b"four").unwrap();

        let authority = FilesystemAuthority::open(&root, 3, 3).unwrap();
        assert_eq!(authority.read("inside.txt"), Err(FilesystemError::ReadLimitExceeded));
        assert_eq!(authority.write("inside.txt", b"four"), Err(FilesystemError::WriteLimitExceeded));
        assert_eq!(std::fs::read(&target).unwrap(), b"four");

        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
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


#[cfg(all(test, any(target_os = "linux", windows)))]
mod hardlink_tests {
    use super::*;

    #[test]
    fn authority_rejects_hard_linked_targets_without_side_effects() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after Unix epoch")
            .as_nanos();
        let base = std::env::temp_dir().join(format!(
            "one-fs-hardlink-{}-{nonce}",
            std::process::id()
        ));
        let root = base.join("root");
        let target = root.join("inside.txt");
        let outside = base.join("outside.txt");
        std::fs::create_dir_all(&root).unwrap();
        let original = b"HARD_LINK_SENTINEL";
        std::fs::write(&target, original).unwrap();
        std::fs::hard_link(&target, &outside)
            .expect("test filesystem must support hard links");

        let authority = FilesystemAuthority::open(&root, 1024, 1024).unwrap();
        assert_eq!(
            authority.read("inside.txt"),
            Err(FilesystemError::HardLinkedFile)
        );
        assert_eq!(
            authority.write("inside.txt", b"MUST_NOT_REPLACE"),
            Err(FilesystemError::HardLinkedFile)
        );
        assert_eq!(std::fs::read(&target).unwrap(), original);
        assert_eq!(std::fs::read(&outside).unwrap(), original);

        drop(authority);
        std::fs::remove_dir_all(base).unwrap();
    }
}

#[cfg(all(test, target_os = "android"))]
mod android_filesystem_tests {
    use super::{FilesystemAuthority, FilesystemError};
    use std::os::unix::fs::symlink;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("one-android-{label}-{}-{}", std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn android_direct_child_read_and_atomic_write_work_with_root_handle() {
        let root = temp_dir("rw");
        let target = root.join("data.txt");
        std::fs::write(&target, b"before").unwrap();
        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        assert_eq!(authority.read("data.txt").unwrap(), b"before");
        authority.write("data.txt", b"after").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"after");
        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn android_leaf_symlinks_and_nested_paths_fail_closed() {
        let root = temp_dir("deny");
        let outside = root.with_extension("outside");
        std::fs::create_dir_all(root.join("nested")).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("sentinel.txt"), b"outside-sentinel").unwrap();
        symlink(&outside, root.join("escape-dir")).unwrap();
        symlink(outside.join("sentinel.txt"), root.join("escape.txt")).unwrap();
        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        assert_eq!(authority.read("escape.txt"), Err(FilesystemError::ResolutionFailed));
        assert_eq!(authority.write("escape.txt", b"changed"), Err(FilesystemError::ResolutionFailed));
        assert_eq!(authority.read("nested/file.txt"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(authority.write("nested/file.txt", b"nested"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(authority.read("escape-dir/sentinel.txt"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(authority.write("escape-dir/sentinel.txt", b"changed"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(std::fs::read(outside.join("sentinel.txt")).unwrap(), b"outside-sentinel");
        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_dir_all(outside).unwrap();
    }

    #[test]
    fn android_root_final_symlink_is_rejected() {
        let root = temp_dir("root");
        let alias = root.with_extension("alias");
        symlink(&root, &alias).unwrap();
        assert!(matches!(FilesystemAuthority::open(&alias, 4096, 4096), Err(FilesystemError::InvalidRoot)));
        std::fs::remove_file(alias).unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn android_hard_link_targets_are_rejected_without_modifying_either_name() {
        let root = temp_dir("hardlink");
        let target = root.join("inside.txt");
        let outside = root.with_extension("outside.txt");
        std::fs::write(&target, b"KEEP_BOTH").unwrap();
        std::fs::hard_link(&target, &outside).expect("Android temp filesystem must support hard links");
        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        assert_eq!(authority.read("inside.txt"), Err(FilesystemError::HardLinkedFile));
        assert_eq!(authority.write("inside.txt", b"REPLACE"), Err(FilesystemError::HardLinkedFile));
        assert_eq!(std::fs::read(&target).unwrap(), b"KEEP_BOTH");
        assert_eq!(std::fs::read(&outside).unwrap(), b"KEEP_BOTH");
        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_file(outside).unwrap();
    }
}

#[cfg(all(test, target_os = "macos"))]
mod macos_filesystem_tests {
    use super::{FilesystemAuthority, FilesystemError};
    use std::os::unix::fs::symlink;
    use std::time::{SystemTime, UNIX_EPOCH};
    fn temp_dir(label: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("one-macos-{label}-{}-{}", std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&path).unwrap(); path
    }
    #[test]
    fn macos_direct_child_read_and_atomic_write_work_with_root_handle() {
        let root = temp_dir("rw"); let target = root.join("data.txt");
        std::fs::write(&target, b"before").unwrap();
        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        assert_eq!(authority.read("data.txt").unwrap(), b"before");
        authority.write("data.txt", b"after").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"after");
        drop(authority); std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn macos_leaf_symlink_and_nested_paths_fail_closed() {
        let root = temp_dir("deny"); let outside = root.with_extension("outside");
        std::fs::create_dir_all(root.join("nested")).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("sentinel.txt"), b"outside-sentinel").unwrap();
        symlink(&outside, root.join("escape-dir")).unwrap();
        symlink(outside.join("sentinel.txt"), root.join("escape.txt")).unwrap();
        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        assert_eq!(authority.read("escape.txt"), Err(FilesystemError::ResolutionFailed));
        assert_eq!(authority.write("escape.txt", b"changed"), Err(FilesystemError::ResolutionFailed));
        assert_eq!(authority.read("nested/file.txt"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(authority.write("nested/file.txt", b"nested"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(authority.read("escape-dir/sentinel.txt"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(authority.write("escape-dir/sentinel.txt", b"changed"), Err(FilesystemError::UnsupportedPlatform));
        assert_eq!(std::fs::read(outside.join("sentinel.txt")).unwrap(), b"outside-sentinel");
        drop(authority); std::fs::remove_dir_all(root).unwrap(); std::fs::remove_dir_all(outside).unwrap();
    }
    #[test]
    fn macos_root_final_symlink_is_rejected() {
        let root = temp_dir("root"); let alias = root.with_extension("alias");
        symlink(&root, &alias).unwrap();
        assert!(matches!(FilesystemAuthority::open(&alias, 4096, 4096), Err(FilesystemError::InvalidRoot)));
        std::fs::remove_file(alias).unwrap(); std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn macos_hard_link_targets_are_rejected_without_modifying_either_name() {
        let root = temp_dir("hardlink");
        let target = root.join("inside.txt");
        let outside = root.with_extension("outside.txt");
        std::fs::write(&target, b"KEEP_BOTH").unwrap();
        std::fs::hard_link(&target, &outside).expect("macOS temp filesystem must support hard links");
        let authority = FilesystemAuthority::open(&root, 4096, 4096).unwrap();
        assert_eq!(authority.read("inside.txt"), Err(FilesystemError::HardLinkedFile));
        assert_eq!(authority.write("inside.txt", b"REPLACE"), Err(FilesystemError::HardLinkedFile));
        assert_eq!(std::fs::read(&target).unwrap(), b"KEEP_BOTH");
        assert_eq!(std::fs::read(&outside).unwrap(), b"KEEP_BOTH");
        drop(authority);
        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_file(outside).unwrap();
    }
}
