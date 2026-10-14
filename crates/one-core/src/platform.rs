use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    Other,
}

pub fn current_platform() -> Platform {
    if cfg!(target_os = "windows") { Platform::Windows } else if cfg!(target_os = "linux") { Platform::Linux } else { Platform::Other }
}

pub fn normalize_path(path: impl AsRef<Path>) -> PathBuf {
    path.as_ref().to_path_buf()
}
