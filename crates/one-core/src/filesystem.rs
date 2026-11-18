//! Host-owned filesystem authority for Issue #104.
//!
//! Linux reads use one openat2 resolution rooted at a retained directory
//! descriptor. Other platforms remain unavailable until their native
//! no-follow implementation is complete. The root path is trusted host
//! configuration, never a request supplied by the language program.

use std::fs::File;
use std::io::Read;
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

impl FilesystemAuthority {
    /// Open and retain the host-selected project root. The root is trusted
    /// host configuration; never pass a program-controlled target here.
    pub fn open(root: impl AsRef<Path>, max_read_bytes: u64, max_write_bytes: u64) -> Result<Self, FilesystemError> {
        if max_read_bytes == 0 || max_write_bytes == 0 { return Err(FilesystemError::InvalidRoot); }
        let canonical_root = std::fs::canonicalize(root).map_err(|_| FilesystemError::InvalidRoot)?;
        let root = File::open(&canonical_root).map_err(|_| FilesystemError::InvalidRoot)?;
        if !root.metadata().map_err(|_| FilesystemError::InvalidRoot)?.is_dir() {
            return Err(FilesystemError::InvalidRoot);
        }
        Ok(Self { root, canonical_root, max_read_bytes, max_write_bytes })
    }

    pub fn root(&self) -> &Path { &self.canonical_root }
    pub const fn max_read_bytes(&self) -> u64 { self.max_read_bytes }
    pub const fn max_write_bytes(&self) -> u64 { self.max_write_bytes }

    /// Read a regular, single-link file through the retained root handle.
    /// Absolute targets are accepted only when beneath the trusted root.
    pub fn read(&self, target: impl AsRef<Path>) -> Result<Vec<u8>, FilesystemError> {
        let relative = self.relative_target(target.as_ref())?;
        let mut file = self.open_relative(&relative)?;
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

    /// Writes remain fail-closed until a race-resistant native write strategy
    /// is implemented and adversarially verified. Never use ambient path I/O.
    pub fn write(&self, _target: impl AsRef<Path>, _bytes: &[u8]) -> Result<(), FilesystemError> {
        Err(FilesystemError::UnsupportedPlatform)
    }

    fn relative_target(&self, target: &Path) -> Result<PathBuf, FilesystemError> {
        let relative = if target.is_absolute() {
            target.strip_prefix(&self.canonical_root).map_err(|_| FilesystemError::OutsideRoot)?
        } else { target };
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
        if fd < 0 { return Err(FilesystemError::ResolutionFailed); }
        // SAFETY: successful openat2 returns a new owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    #[cfg(not(target_os = "linux"))]
    fn open_relative(&self, _relative: &Path) -> Result<File, FilesystemError> {
        Err(FilesystemError::UnsupportedPlatform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_rejects_zero_limits_and_non_directory_roots() {
        assert_eq!(FilesystemAuthority::open(std::env::temp_dir(), 0, 1).err(), Some(FilesystemError::InvalidRoot));
        let file = std::env::temp_dir().join("one-filesystem-authority-root-file");
        std::fs::write(&file, b"x").unwrap();
        assert_eq!(FilesystemAuthority::open(&file, 10, 10).err(), Some(FilesystemError::InvalidRoot));
        let _ = std::fs::remove_file(file);
    }

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
