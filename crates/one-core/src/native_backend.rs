use std::{collections::BTreeMap, env, ffi::OsStr, fmt, path::PathBuf, process::Command};

use crate::{
    ir::{IrExpr, IrProgram, IrStatement},
    target::{
        ArtifactFormat, ExecutionProfile, LinkerStrategy, RuntimeClass, TargetArchitecture,
        TargetBackend, TargetEnvironment, TargetSpec, ToolchainCapability, ToolchainOperation,
        ToolchainSpec, ToolchainTarget,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeArtifact {
    pub path: PathBuf,
    pub format: ArtifactFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeBackendError {
    code: &'static str,
    message: String,
}
impl NativeBackendError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    pub fn code(&self) -> &'static str {
        self.code
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl fmt::Display for NativeBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for NativeBackendError {}

pub struct NativeAndroidAarch64Backend;
impl NativeAndroidAarch64Backend {
    pub const fn new() -> Self {
        Self
    }
    pub fn target(&self) -> TargetSpec {
        TargetSpec::hosted(TargetArchitecture::Aarch64, TargetEnvironment::Linux)
            .with_backend(self.id())
    }
    pub fn toolchain(&self) -> ToolchainSpec {
        ToolchainSpec::new(
            "clang-android-aarch64",
            self.id(),
            ToolchainTarget::new(
                "aarch64-linux-android24",
                TargetArchitecture::Aarch64,
                TargetEnvironment::Linux,
            ),
            LinkerStrategy::Named("clang".into()),
            ArtifactFormat::Executable,
            ToolchainCapability::BuildAndExecute,
        )
    }
    pub fn codegen(&self, program: &IrProgram) -> Result<String, NativeBackendError> {
        let mut generator = CGenerator {
            variables: BTreeMap::new(),
            output: String::new(),
        };
        generator
            .output
            .push_str("#include <stdio.h>\n#include <stdbool.h>\n\nint main(void) {\n");
        for statement in &program.statements {
            generator.statement(statement)?;
        }
        generator.output.push_str("  return 0;\n}\n");
        Ok(generator.output)
    }
    pub fn build(
        &self,
        program: &IrProgram,
        output: impl Into<PathBuf>,
    ) -> Result<NativeArtifact, NativeBackendError> {
        let output = output.into();
        let target = self.target();
        let toolchain = self.toolchain();
        toolchain
            .validate_for(&target, self, ToolchainOperation::Build)
            .map_err(|error| {
                NativeBackendError::new("NATIVE-TOOLCHAIN-INVALID", error.to_string())
            })?;
        let source = self.codegen(program)?;
        let source_path = output.with_extension("c");
        std::fs::write(&source_path, source).map_err(|error| {
            NativeBackendError::new(
                "NATIVE-CODEGEN-WRITE",
                format!(
                    "tidak dapat menulis source C '{}': {error}",
                    source_path.display()
                ),
            )
        })?;
        let compiler = std::env::var_os("ONE_ANDROID_CLANG")
            .or_else(|| std::env::var_os("ONE_CLANG"))
            .unwrap_or_else(|| "clang".into());
        let status = Command::new(&compiler)
            .arg("--target=aarch64-linux-android24")
            .arg(&source_path)
            .arg("-o")
            .arg(&output)
            .status()
            .map_err(|error| {
                NativeBackendError::new(
                    "NATIVE-TOOLCHAIN-INVOKE",
                    format!("tidak dapat menjalankan clang: {error}"),
                )
            })?;
        if !status.success() {
            return Err(NativeBackendError::new(
                "NATIVE-LINK-FAILED",
                format!("clang gagal dengan status {status}"),
            ));
        }
        if !output.is_file() {
            return Err(NativeBackendError::new(
                "NATIVE-ARTIFACT-MISSING",
                format!("artifact native tidak terbentuk: {}", output.display()),
            ));
        }
        let _ = std::fs::remove_file(&source_path);
        Ok(NativeArtifact {
            path: output,
            format: ArtifactFormat::Executable,
        })
    }
}
impl Default for NativeAndroidAarch64Backend {
    fn default() -> Self {
        Self::new()
    }
}
impl TargetBackend for NativeAndroidAarch64Backend {
    fn id(&self) -> &'static str {
        "native-android-aarch64"
    }
    fn required_runtime(&self) -> RuntimeClass {
        RuntimeClass::Hosted
    }
    fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool {
        matches!(architecture, TargetArchitecture::Aarch64)
    }
    fn supports_profile(&self, profile: ExecutionProfile) -> bool {
        matches!(profile, ExecutionProfile::Hosted)
    }
    fn supports_environment(&self, environment: TargetEnvironment) -> bool {
        matches!(environment, TargetEnvironment::Linux)
    }
}

struct CGenerator {
    variables: BTreeMap<String, CType>,
    output: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CType {
    Text,
    Number,
    Boolean,
}
impl CGenerator {
    fn identifier(name: &str) -> Result<(), NativeBackendError> {
        let mut chars = name.chars();
        let Some(first) = chars.next() else {
            return Err(NativeBackendError::new(
                "NATIVE-CODEGEN-IDENTIFIER",
                "identifier native tidak boleh kosong",
            ));
        };
        if !(first == '_' || first.is_ascii_alphabetic())
            || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
        {
            return Err(NativeBackendError::new(
                "NATIVE-CODEGEN-IDENTIFIER",
                format!("identifier '{}' tidak dapat dipetakan aman ke C", name),
            ));
        }
        Ok(())
    }
    fn statement(&mut self, statement: &IrStatement) -> Result<(), NativeBackendError> {
        match statement {
            IrStatement::Declaration { name, value } => {
                Self::identifier(name)?;
                let ty = self.expr_type(value)?;
                self.variables.insert(name.clone(), ty);
                self.output.push_str("  ");
                self.output.push_str(match ty {
                    CType::Text => "const char* ",
                    CType::Number => "long long ",
                    CType::Boolean => "bool ",
                });
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.expr(value)?;
                self.output.push_str(";\n");
            }
            IrStatement::Assignment { name, value } => {
                Self::identifier(name)?;
                let Some(expected) = self.variables.get(name).copied() else {
                    return Err(NativeBackendError::new(
                        "NATIVE-CODEGEN-UNDEFINED",
                        format!("assignment ke variabel '{}' tidak didukung", name),
                    ));
                };
                if expected != self.expr_type(value)? {
                    return Err(NativeBackendError::new(
                        "NATIVE-CODEGEN-TYPE",
                        format!("tipe assignment '{}' tidak cocok", name),
                    ));
                }
                self.output.push_str("  ");
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.expr(value)?;
                self.output.push_str(";\n");
            }
            IrStatement::Action { name, args } if name == "tampilkan" && args.len() == 1 => {
                match self.expr_type(&args[0])? {
                    CType::Text => {
                        self.output.push_str("  puts(");
                        self.expr(&args[0])?;
                        self.output.push_str(");\n");
                    }
                    CType::Number => {
                        self.output.push_str("  printf(\"%lld\\n\", ");
                        self.expr(&args[0])?;
                        self.output.push_str(");\n");
                    }
                    CType::Boolean => {
                        self.output.push_str("  puts(");
                        self.expr(&args[0])?;
                        self.output.push_str(" ? \"benar\" : \"salah\");\n");
                    }
                }
            }
            IrStatement::Action { name, .. } if name == "tampilkan" => {
                return Err(NativeBackendError::new(
                    "NATIVE-CODEGEN-ARITY",
                    "'tampilkan' membutuhkan tepat satu nilai",
                ));
            }
            IrStatement::Action { name, .. } => {
                return Err(NativeBackendError::new(
                    "NATIVE-CODEGEN-UNSUPPORTED",
                    format!("aksi '{}' belum didukung backend native", name),
                ));
            }
            _ => {
                return Err(NativeBackendError::new(
                    "NATIVE-CODEGEN-UNSUPPORTED",
                    "konstruksi IR ini belum didukung backend native Android AArch64",
                ));
            }
        }
        Ok(())
    }
    fn expr_type(&self, expr: &IrExpr) -> Result<CType, NativeBackendError> {
        match expr {
            IrExpr::Text(_) => Ok(CType::Text),
            IrExpr::Number(_) => Ok(CType::Number),
            IrExpr::Boolean(_) => Ok(CType::Boolean),
            IrExpr::Identifier(name) => {
                Self::identifier(name)?;
                self.variables.get(name).copied().ok_or_else(|| {
                    NativeBackendError::new(
                        "NATIVE-CODEGEN-UNDEFINED",
                        format!("identifier '{}' tidak ditemukan", name),
                    )
                })
            }
            IrExpr::Binary { .. } | IrExpr::Nothing | IrExpr::Call { .. } => {
                Err(NativeBackendError::new(
                    "NATIVE-CODEGEN-UNSUPPORTED",
                    "ekspresi non-literal belum didukung backend native",
                ))
            }
        }
    }
    fn expr(&mut self, expr: &IrExpr) -> Result<(), NativeBackendError> {
        match expr {
            IrExpr::Text(value) => {
                self.output.push('"');
                for ch in value.chars() {
                    match ch {
                        '\\' => self.output.push_str("\\\\"),
                        '"' => self.output.push_str("\\\""),
                        '\n' => self.output.push_str("\\n"),
                        '\r' => self.output.push_str("\\r"),
                        '\t' => self.output.push_str("\\t"),
                        other => self.output.push(other),
                    }
                }
                self.output.push('"');
            }
            IrExpr::Number(value) => self.output.push_str(&value.to_string()),
            IrExpr::Boolean(value) => self.output.push_str(if *value { "true" } else { "false" }),
            IrExpr::Identifier(name) => {
                Self::identifier(name)?;
                self.output.push_str(name);
            }
            IrExpr::Binary { .. } | IrExpr::Nothing | IrExpr::Call { .. } => {
                return Err(NativeBackendError::new(
                    "NATIVE-CODEGEN-UNSUPPORTED",
                    "ekspresi non-literal belum didukung backend native",
                ));
            }
        }
        Ok(())
    }
}

pub struct NativeLinuxAarch64Backend;
pub struct NativeLinuxX86_64Backend;
pub struct NativeWindowsX86_64Backend;

macro_rules! native_backend {
    ($name:ident, $id:literal, $triple:literal, $arch:expr, $env:expr, $compiler:literal, $target_arg:literal, $linker_resolver:expr) => {
        impl $name {
            pub const fn new() -> Self {
                Self
            }

            pub fn target(&self) -> TargetSpec {
                TargetSpec::hosted($arch, $env).with_backend(self.id())
            }

            pub fn toolchain(&self) -> ToolchainSpec {
                ToolchainSpec::new(
                    $compiler,
                    self.id(),
                    ToolchainTarget::new($triple, $arch, $env),
                    LinkerStrategy::Named("clang".into()),
                    ArtifactFormat::Executable,
                    ToolchainCapability::BuildAndExecute,
                )
            }

            pub fn codegen(&self, program: &IrProgram) -> Result<String, NativeBackendError> {
                CGenerator::generate(program)
            }

            pub fn build(
                &self,
                program: &IrProgram,
                output: impl Into<PathBuf>,
            ) -> Result<NativeArtifact, NativeBackendError> {
                let output = output.into();
                let target = self.target();
                let toolchain = self.toolchain();

                toolchain
                    .validate_for(&target, self, ToolchainOperation::Build)
                    .map_err(|error| {
                        NativeBackendError::new("NATIVE-TOOLCHAIN-INVALID", error.to_string())
                    })?;

                let source = self.codegen(program)?;
                let source_path = output.with_extension("c");

                std::fs::write(&source_path, source).map_err(|error| {
                    NativeBackendError::new(
                        "NATIVE-CODEGEN-WRITE",
                        format!(
                            "tidak dapat menulis source C '{}': {error}",
                            source_path.display()
                        ),
                    )
                })?;

                let compiler_override = if self.id() == "native-windows-x86_64" {
                    "ONE_WINDOWS_CLANG"
                } else {
                    "ONE_CLANG"
                };
                let compiler = std::env::var_os(compiler_override)
                    .or_else(|| std::env::var_os("ONE_CLANG"))
                    .unwrap_or_else(|| $compiler.into());

                let mut command = Command::new(&compiler);
                command.arg($target_arg);

                let linker_arg: Option<PathBuf> = $linker_resolver()?;
                if let Some(linker_arg) = linker_arg {
                    command.arg(format!("--ld-path={}", linker_arg.display()));
                }
                let status = command
                    .arg(&source_path)
                    .arg("-o")
                    .arg(&output)
                    .status()
                    .map_err(|error| {
                        NativeBackendError::new(
                            "NATIVE-TOOLCHAIN-INVOKE",
                            format!("tidak dapat menjalankan compiler: {error}"),
                        )
                    })?;

                if !status.success() {
                    return Err(NativeBackendError::new(
                        "NATIVE-LINK-FAILED",
                        format!("compiler gagal dengan status {status}"),
                    ));
                }

                if !output.is_file() {
                    return Err(NativeBackendError::new(
                        "NATIVE-ARTIFACT-MISSING",
                        format!("artifact native tidak terbentuk: {}", output.display()),
                    ));
                }

                validate_native_artifact(&output, $arch, $env)?;

                let _ = std::fs::remove_file(&source_path);

                Ok(NativeArtifact {
                    path: output,
                    format: ArtifactFormat::Executable,
                })
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl TargetBackend for $name {
            fn id(&self) -> &'static str {
                $id
            }

            fn required_runtime(&self) -> RuntimeClass {
                RuntimeClass::Hosted
            }

            fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool {
                architecture == &$arch
            }

            fn supports_profile(&self, profile: ExecutionProfile) -> bool {
                matches!(profile, ExecutionProfile::Hosted)
            }

            fn supports_environment(&self, environment: TargetEnvironment) -> bool {
                environment == $env
            }
        }
    };
}


fn resolve_linux_x86_64_linker(
    override_path: Option<&PathBuf>,
    path_env: Option<&OsStr>,
) -> Result<PathBuf, NativeBackendError> {
    if let Some(path) = override_path {
        return match Command::new(path).arg("--version").status() {
            Ok(status) if status.success() => Ok(path.clone()),
            _ => Err(NativeBackendError::new(
                "NATIVE-LINKER-INCOMPATIBLE",
                "linker GNU x86_64 ditemukan tetapi tidak kompatibel atau tidak dapat dijalankan",
            )),
        };
    }

    let path_env = path_env.unwrap_or_else(|| OsStr::new(""));
    let mut found_incompatible = false;

    for candidate in env::split_paths(path_env)
        .map(|directory| directory.join("x86_64-linux-gnu-ld"))
    {
        if !candidate.is_file() {
            continue;
        }

        match Command::new(&candidate).arg("--version").status() {
            Ok(status) if status.success() => return Ok(candidate),
            _ => found_incompatible = true,
        }
    }

    if found_incompatible {
        Err(NativeBackendError::new(
            "NATIVE-LINKER-INCOMPATIBLE",
            "linker GNU x86_64 ditemukan tetapi tidak kompatibel atau tidak dapat dijalankan",
        ))
    } else {
        Err(NativeBackendError::new(
            "NATIVE-LINKER-NOT-FOUND",
            "target x86_64-unknown-linux-gnu membutuhkan linker GNU x86_64 yang kompatibel",
        ))
    }
}

#[cfg(all(test, unix))]
mod linker_discovery_tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn linux_x86_64_linker_override_diprioritaskan() {
        let root = std::env::temp_dir().join(format!(
            "one-native-linker-override-{}",
            std::process::id()
        ));
        let expected = root.join("x86_64-linux-gnu-ld");
        let path_candidate = root.join("path-bin");
        let path_linker = path_candidate.join("x86_64-linux-gnu-ld");

        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&path_candidate).unwrap();

        std::fs::write(&expected, b"#!/bin/sh\nexit 0\n").unwrap();
        std::fs::write(&path_linker, b"#!/bin/sh\nexit 1\n").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for path in [&expected, &path_linker] {
                let mut permissions = std::fs::metadata(path).unwrap().permissions();
                permissions.set_mode(0o755);
                std::fs::set_permissions(path, permissions).unwrap();
            }
        }

        assert_eq!(
            resolve_linux_x86_64_linker(Some(&expected), Some(path_candidate.as_os_str()))
                .unwrap(),
            expected
        );

        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn linux_x86_64_linker_override_tidak_kompatibel_ditolak() {
        let root = std::env::temp_dir().join(format!(
            "one-native-linker-override-invalid-{}",
            std::process::id()
        ));
        let candidate = root.join("x86_64-linux-gnu-ld");

        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(&candidate, b"#!/bin/sh\nexit 1\n").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&candidate).unwrap().permissions();
            permissions.set_mode(0o755);
            std::fs::set_permissions(&candidate, permissions).unwrap();
        }

        let result = resolve_linux_x86_64_linker(Some(&candidate), None);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().code(),
            "NATIVE-LINKER-INCOMPATIBLE"
        );

        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn linux_x86_64_linker_host_salah_ditolak() {
        let root = std::env::temp_dir().join(format!(
            "one-native-linker-invalid-{}",
            std::process::id()
        ));
        let bin = root.join("bin");
        let candidate = bin.join("x86_64-linux-gnu-ld");

        std::fs::create_dir_all(&bin).unwrap();
        std::fs::write(&candidate, b"#!/bin/sh\nexit 1\n").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&candidate).unwrap().permissions();
            permissions.set_mode(0o755);
            std::fs::set_permissions(&candidate, permissions).unwrap();
        }

        let result = resolve_linux_x86_64_linker(None, Some(bin.as_os_str()));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().code(),
            "NATIVE-LINKER-INCOMPATIBLE"
        );

        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn linux_x86_64_linker_ditemukan_dari_path() {
        let root = std::env::temp_dir().join(format!(
            "one-native-linker-test-{}",
            std::process::id()
        ));
        let bin = root.join("bin");
        let expected = bin.join("x86_64-linux-gnu-ld");

        std::fs::create_dir_all(&bin).unwrap();
        // This test verifies PATH discovery and successful invocation, not GNU ld
        // compatibility parsing. Use a known executable instead of a shell script
        // so the test is independent of the host shell and script execution policy.
        std::os::unix::fs::symlink("/bin/true", &expected).unwrap();

        let path_env = bin.as_os_str();
        assert_eq!(
            resolve_linux_x86_64_linker(None, Some(path_env)).unwrap(),
            expected
        );

        let _ = std::fs::remove_dir_all(root);
    }
}


impl CGenerator {
    fn generate(program: &IrProgram) -> Result<String, NativeBackendError> {
        let mut generator = CGenerator {
            variables: BTreeMap::new(),
            output: String::new(),
        };

        generator
            .output
            .push_str("#include <stdio.h>\n#include <stdbool.h>\n\nint main(void) {\n");

        for statement in &program.statements {
            generator.statement(statement)?;
        }

        generator.output.push_str("  return 0;\n}\n");
        Ok(generator.output)
    }
}

native_backend!(
    NativeLinuxAarch64Backend,
    "native-linux-aarch64",
    "aarch64-unknown-linux-gnu",
    TargetArchitecture::Aarch64,
    TargetEnvironment::Linux,
    "clang",
    "--target=aarch64-unknown-linux-gnu",
    || Ok::<Option<PathBuf>, NativeBackendError>(None)
);

native_backend!(
    NativeLinuxX86_64Backend,
    "native-linux-x86_64",
    "x86_64-unknown-linux-gnu",
    TargetArchitecture::X86_64,
    TargetEnvironment::Linux,
    "clang",
    "--target=x86_64-unknown-linux-gnu",
    || {
        resolve_linux_x86_64_linker(
            std::env::var_os("ONE_LINUX_X86_64_LD")
                .map(PathBuf::from)
                .as_ref(),
            std::env::var_os("PATH").as_deref(),
        )
        .map(Some)
    }
);

native_backend!(
    NativeWindowsX86_64Backend,
    "native-windows-x86_64",
    "x86_64-pc-windows-gnu",
    TargetArchitecture::X86_64,
    TargetEnvironment::Windows,
    "x86_64-w64-mingw32-clang",
    "--target=x86_64-w64-windows-gnu",
    || Ok::<Option<PathBuf>, NativeBackendError>(None)
);

fn validate_native_artifact(
    path: &std::path::Path,
    architecture: TargetArchitecture,
    environment: TargetEnvironment,
) -> Result<(), NativeBackendError> {
    let bytes = std::fs::read(path).map_err(|error| {
        NativeBackendError::new(
            "NATIVE-ARTIFACT-INVALID",
            format!("gagal membaca artifact '{}': {}", path.display(), error),
        )
    })?;

    let invalid = || {
        NativeBackendError::new(
            "NATIVE-ARTIFACT-INVALID",
            format!(
                "artifact '{}' tidak sesuai format/arsitektur target",
                path.display()
            ),
        )
    };

    match environment {
        TargetEnvironment::Linux => {
            if bytes.len() < 20 || &bytes[0..4] != b"\x7fELF" {
                return Err(invalid());
            }

            if bytes[4] != 2 || bytes[5] != 1 {
                return Err(invalid());
            }

            let elf_type = u16::from_le_bytes([bytes[16], bytes[17]]);
            if !matches!(elf_type, 2 | 3) {
                return Err(invalid());
            }

            let machine = u16::from_le_bytes([bytes[18], bytes[19]]);
            let expected = match architecture {
                TargetArchitecture::Aarch64 => 183,
                TargetArchitecture::X86_64 => 62,
                _ => return Err(invalid()),
            };

            if machine != expected {
                return Err(invalid());
            }
        }
        TargetEnvironment::Windows => {
            if bytes.len() < 70 || &bytes[0..2] != b"MZ" {
                return Err(invalid());
            }

            let pe_offset =
                u32::from_le_bytes([bytes[0x3c], bytes[0x3d], bytes[0x3e], bytes[0x3f]]) as usize;

            if pe_offset.checked_add(6).is_none()
                || bytes.len() < pe_offset + 6
                || &bytes[pe_offset..pe_offset + 4] != b"PE\0\0"
            {
                return Err(invalid());
            }

            let machine = u16::from_le_bytes([bytes[pe_offset + 4], bytes[pe_offset + 5]]);

            if !matches!(architecture, TargetArchitecture::X86_64) || machine != 0x8664 {
                return Err(invalid());
            }
        }
        _ => return Err(invalid()),
    }

    Ok(())
}

#[cfg(test)]
mod artifact_validation_tests {
    use super::*;
    use std::fs;

    fn temp_artifact(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "one-native-artifact-validation-{}-{}",
            std::process::id(),
            name
        ))
    }

    #[test]
    fn menolak_artifact_linux_yang_bukan_elf() {
        let path = temp_artifact("not-elf");
        fs::write(&path, b"not-an-elf").unwrap();

        let result =
            validate_native_artifact(&path, TargetArchitecture::X86_64, TargetEnvironment::Linux);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "NATIVE-ARTIFACT-INVALID");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn menolak_elf_dengan_arsitektur_yang_salah() {
        let path = temp_artifact("wrong-elf");

        let mut elf = vec![0u8; 64];
        elf[0..4].copy_from_slice(b"\x7fELF");
        elf[4] = 2;
        elf[5] = 1;
        elf[16..18].copy_from_slice(&2u16.to_le_bytes());
        elf[18..20].copy_from_slice(&183u16.to_le_bytes());
        fs::write(&path, elf).unwrap();

        let result =
            validate_native_artifact(&path, TargetArchitecture::X86_64, TargetEnvironment::Linux);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "NATIVE-ARTIFACT-INVALID");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn menerima_elf_aarch64_linux() {
        let path = temp_artifact("elf-aarch64");

        let mut elf = vec![0u8; 64];
        elf[0..4].copy_from_slice(b"\x7fELF");
        elf[4] = 2;
        elf[5] = 1;
        elf[16..18].copy_from_slice(&2u16.to_le_bytes());
        elf[18..20].copy_from_slice(&183u16.to_le_bytes());
        fs::write(&path, elf).unwrap();

        let result =
            validate_native_artifact(&path, TargetArchitecture::Aarch64, TargetEnvironment::Linux);

        assert!(result.is_ok());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn menerima_elf_x86_64_linux() {
        let path = temp_artifact("elf-x86-64");

        let mut elf = vec![0u8; 64];
        elf[0..4].copy_from_slice(b"\x7fELF");
        elf[4] = 2;
        elf[5] = 1;
        elf[16..18].copy_from_slice(&2u16.to_le_bytes());
        elf[18..20].copy_from_slice(&62u16.to_le_bytes());
        fs::write(&path, elf).unwrap();

        let result =
            validate_native_artifact(&path, TargetArchitecture::X86_64, TargetEnvironment::Linux);

        assert!(result.is_ok());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn menerima_pe_x86_64_windows() {
        let path = temp_artifact("pe-x86-64");

        let mut pe = vec![0u8; 128];
        pe[0..2].copy_from_slice(b"MZ");
        pe[0x3c..0x40].copy_from_slice(&(64u32).to_le_bytes());
        pe[64..68].copy_from_slice(b"PE\0\0");
        pe[68..70].copy_from_slice(&0x8664u16.to_le_bytes());
        fs::write(&path, pe).unwrap();

        let result = validate_native_artifact(
            &path,
            TargetArchitecture::X86_64,
            TargetEnvironment::Windows,
        );

        assert!(result.is_ok());

        let _ = fs::remove_file(path);
    }
}
