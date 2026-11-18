//! Host-owned project filesystem authority (Issue #104).
//!
//! The production implementation is intentionally introduced in staged,
//! compile-verified steps. No filesystem operation may fall back to ambient
//! path-based access while this module is incomplete.

/// Compile-time API probe for the Windows native relative-open primitive.
///
/// NtCreateFile accepts OBJECT_ATTRIBUTES whose RootDirectory is a retained
/// directory handle. This is the native API surface required for the Windows
/// implementation; this probe does not perform I/O and is not itself a
/// security implementation or acceptance test.
#[cfg(windows)]
#[allow(dead_code)]
mod windows_native_api_probe {
    use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
    use windows_sys::Wdk::Storage::FileSystem::NtCreateFile;

    pub(super) fn assert_native_relative_open_bindings_are_available() {
        let _native_open = NtCreateFile;
        let _object_attributes_size = core::mem::size_of::<OBJECT_ATTRIBUTES>();
    }
}
