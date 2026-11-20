use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform { Windows, Linux, Android, Other }

pub fn current_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else {
        Platform::Other
    }
}

pub fn normalize_path(path: impl AsRef<Path>) -> PathBuf { path.as_ref().to_path_buf() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_platform_is_explicit() {
        let platform = current_platform();
        if cfg!(target_os = "windows") {
            assert_eq!(platform, Platform::Windows);
        } else if cfg!(target_os = "android") {
            assert_eq!(platform, Platform::Android);
        } else if cfg!(target_os = "linux") {
            assert_eq!(platform, Platform::Linux);
        } else {
            assert_eq!(platform, Platform::Other);
        }
    }

    #[test]
    fn normalize_path_preserves_declared_path_without_hidden_filesystem_search() {
        let input = PathBuf::from("project/src/main.one");
        assert_eq!(normalize_path(&input), input);
    }
}
