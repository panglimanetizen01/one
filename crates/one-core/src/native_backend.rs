use std::{collections::BTreeMap, fmt, path::PathBuf, process::Command};

use crate::{
    ir::{IrExpr, IrProgram, IrStatement},
    target::{ArtifactFormat, ExecutionProfile, LinkerStrategy, RuntimeClass, TargetArchitecture, TargetBackend, TargetEnvironment, TargetSpec, ToolchainCapability, ToolchainOperation, ToolchainSpec, ToolchainTarget},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeArtifact { pub path: PathBuf, pub format: ArtifactFormat }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeBackendError { code: &'static str, message: String }
impl NativeBackendError {
    fn new(code: &'static str, message: impl Into<String>) -> Self { Self { code, message: message.into() } }
    pub fn code(&self) -> &'static str { self.code }
    pub fn message(&self) -> &str { &self.message }
}
impl fmt::Display for NativeBackendError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}: {}", self.code, self.message) } }
impl std::error::Error for NativeBackendError {}

pub struct NativeAndroidAarch64Backend;
impl NativeAndroidAarch64Backend {
    pub const fn new() -> Self { Self }
    pub fn target(&self) -> TargetSpec { TargetSpec::hosted(TargetArchitecture::Aarch64, TargetEnvironment::Linux).with_backend(self.id()) }
    pub fn toolchain(&self) -> ToolchainSpec { ToolchainSpec::new("clang-android-aarch64", self.id(), ToolchainTarget::new("aarch64-linux-android24", TargetArchitecture::Aarch64, TargetEnvironment::Linux), LinkerStrategy::Named("clang".into()), ArtifactFormat::Executable, ToolchainCapability::BuildAndExecute) }
    pub fn codegen(&self, program: &IrProgram) -> Result<String, NativeBackendError> {
        let mut generator = CGenerator { variables: BTreeMap::new(), output: String::new() };
        generator.output.push_str("#include <stdio.h>\n#include <stdbool.h>\n\nint main(void) {\n");
        for statement in &program.statements { generator.statement(statement)?; }
        generator.output.push_str("  return 0;\n}\n");
        Ok(generator.output)
    }
    pub fn build(&self, program: &IrProgram, output: impl Into<PathBuf>) -> Result<NativeArtifact, NativeBackendError> {
        let output = output.into();
        let target = self.target();
        let toolchain = self.toolchain();
        toolchain.validate_for(&target, self, ToolchainOperation::Build).map_err(|error| NativeBackendError::new("NATIVE-TOOLCHAIN-INVALID", error.to_string()))?;
        let source = self.codegen(program)?;
        let source_path = output.with_extension("c");
        std::fs::write(&source_path, source).map_err(|error| NativeBackendError::new("NATIVE-CODEGEN-WRITE", format!("tidak dapat menulis source C '{}': {error}", source_path.display())))?;
        let compiler = std::env::var_os("ONE_CLANG").unwrap_or_else(|| "clang".into());
        let status = Command::new(&compiler).arg("--target=aarch64-linux-android24").arg(&source_path).arg("-o").arg(&output).status().map_err(|error| NativeBackendError::new("NATIVE-TOOLCHAIN-INVOKE", format!("tidak dapat menjalankan clang: {error}")))?;
        if !status.success() { return Err(NativeBackendError::new("NATIVE-LINK-FAILED", format!("clang gagal dengan status {status}"))); }
        if !output.is_file() { return Err(NativeBackendError::new("NATIVE-ARTIFACT-MISSING", format!("artifact native tidak terbentuk: {}", output.display()))); }
        let _ = std::fs::remove_file(&source_path);
        Ok(NativeArtifact { path: output, format: ArtifactFormat::Executable })
    }
}
impl Default for NativeAndroidAarch64Backend { fn default() -> Self { Self::new() } }
impl TargetBackend for NativeAndroidAarch64Backend {
    fn id(&self) -> &'static str { "native-android-aarch64" }
    fn required_runtime(&self) -> RuntimeClass { RuntimeClass::Hosted }
    fn supports_architecture(&self, architecture: &TargetArchitecture) -> bool { matches!(architecture, TargetArchitecture::Aarch64) }
    fn supports_profile(&self, profile: ExecutionProfile) -> bool { matches!(profile, ExecutionProfile::Hosted) }
    fn supports_environment(&self, environment: TargetEnvironment) -> bool { matches!(environment, TargetEnvironment::Linux) }
}

struct CGenerator { variables: BTreeMap<String, CType>, output: String }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] enum CType { Text, Number, Boolean }
impl CGenerator {
    fn identifier(name: &str) -> Result<(), NativeBackendError> {
        let mut chars = name.chars();
        let Some(first) = chars.next() else { return Err(NativeBackendError::new("NATIVE-CODEGEN-IDENTIFIER", "identifier native tidak boleh kosong")); };
        if !(first == '_' || first.is_ascii_alphabetic()) || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric()) {
            return Err(NativeBackendError::new("NATIVE-CODEGEN-IDENTIFIER", format!("identifier '{}' tidak dapat dipetakan aman ke C", name)));
        }
        Ok(())
    }
    fn statement(&mut self, statement: &IrStatement) -> Result<(), NativeBackendError> {
        match statement {
            IrStatement::Declaration { name, value } => { Self::identifier(name)?; let ty = self.expr_type(value)?; self.variables.insert(name.clone(), ty); self.output.push_str("  "); self.output.push_str(match ty { CType::Text => "const char* ", CType::Number => "long long ", CType::Boolean => "bool " }); self.output.push_str(name); self.output.push_str(" = "); self.expr(value)?; self.output.push_str(";\n"); }
            IrStatement::Assignment { name, value } => { Self::identifier(name)?; let Some(expected) = self.variables.get(name).copied() else { return Err(NativeBackendError::new("NATIVE-CODEGEN-UNDEFINED", format!("assignment ke variabel '{}' tidak didukung", name))); }; if expected != self.expr_type(value)? { return Err(NativeBackendError::new("NATIVE-CODEGEN-TYPE", format!("tipe assignment '{}' tidak cocok", name))); } self.output.push_str("  "); self.output.push_str(name); self.output.push_str(" = "); self.expr(value)?; self.output.push_str(";\n"); }
            IrStatement::Action { name, args } if name == "tampilkan" && args.len() == 1 => match self.expr_type(&args[0])? { CType::Text => { self.output.push_str("  puts("); self.expr(&args[0])?; self.output.push_str(");\n"); }, CType::Number => { self.output.push_str("  printf(\"%lld\\n\", "); self.expr(&args[0])?; self.output.push_str(");\n"); }, CType::Boolean => { self.output.push_str("  puts("); self.expr(&args[0])?; self.output.push_str(" ? \"benar\" : \"salah\");\n"); } },
            IrStatement::Action { name, .. } if name == "tampilkan" => return Err(NativeBackendError::new("NATIVE-CODEGEN-ARITY", "'tampilkan' membutuhkan tepat satu nilai")),
            IrStatement::Action { name, .. } => return Err(NativeBackendError::new("NATIVE-CODEGEN-UNSUPPORTED", format!("aksi '{}' belum didukung backend native", name))),
            _ => return Err(NativeBackendError::new("NATIVE-CODEGEN-UNSUPPORTED", "konstruksi IR ini belum didukung backend native Android AArch64")),
        }
        Ok(())
    }
    fn expr_type(&self, expr: &IrExpr) -> Result<CType, NativeBackendError> {
        match expr {
            IrExpr::Text(_) => Ok(CType::Text), IrExpr::Number(_) => Ok(CType::Number), IrExpr::Boolean(_) => Ok(CType::Boolean),
            IrExpr::Identifier(name) => { Self::identifier(name)?; self.variables.get(name).copied().ok_or_else(|| NativeBackendError::new("NATIVE-CODEGEN-UNDEFINED", format!("identifier '{}' tidak ditemukan", name))) }
            IrExpr::Binary { .. } | IrExpr::Nothing | IrExpr::Call { .. } => Err(NativeBackendError::new("NATIVE-CODEGEN-UNSUPPORTED", "ekspresi non-literal belum didukung backend native")),
        }
    }
    fn expr(&mut self, expr: &IrExpr) -> Result<(), NativeBackendError> {
        match expr {
            IrExpr::Text(value) => { self.output.push('"'); for ch in value.chars() { match ch { '\\' => self.output.push_str("\\\\"), '"' => self.output.push_str("\\\""), '\n' => self.output.push_str("\\n"), '\r' => self.output.push_str("\\r"), '\t' => self.output.push_str("\\t"), other => self.output.push(other) } } self.output.push('"'); },
            IrExpr::Number(value) => self.output.push_str(&value.to_string()), IrExpr::Boolean(value) => self.output.push_str(if *value { "true" } else { "false" }),
            IrExpr::Identifier(name) => { Self::identifier(name)?; self.output.push_str(name); },
            IrExpr::Binary { .. } | IrExpr::Nothing | IrExpr::Call { .. } => return Err(NativeBackendError::new("NATIVE-CODEGEN-UNSUPPORTED", "ekspresi non-literal belum didukung backend native")),
        }
        Ok(())
    }
}
