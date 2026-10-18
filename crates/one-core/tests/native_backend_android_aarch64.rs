use one_core::{
    lower, parse, analyze, ArtifactFormat, ExecutionProfile, IrProgram, NativeAndroidAarch64Backend,
    TargetArchitecture, TargetBackend, TargetEnvironment, TargetSpec, ToolchainCapability,
};

fn compile(source: &str) -> IrProgram {
    let program = parse(source).expect("source harus dapat diparse");
    let typed = analyze(&program).expect("source harus dapat dianalisis");
    lower(&typed).expect("source harus dapat diturunkan ke IR")
}

#[test]
fn backend_menerima_target_android_aarch64_hosted() {
    let backend = NativeAndroidAarch64Backend::new();
    let target = TargetSpec::hosted(TargetArchitecture::Aarch64, TargetEnvironment::Linux)
        .with_backend(backend.id());
    backend.validate(&target).expect("target Android AArch64 harus diterima");
    assert_eq!(backend.required_runtime(), one_core::RuntimeClass::Hosted);
    assert_eq!(target.profile(), ExecutionProfile::Hosted);
}

#[test]
fn backend_menghasilkan_source_c_deterministik_dari_ir_sederhana() {
    let ir = compile("nama adalah \"Halo native\"\ntampilkan nama\n");
    let backend = NativeAndroidAarch64Backend::new();
    let first = backend.codegen(&ir).expect("IR sederhana harus dapat dikodegen");
    let second = backend.codegen(&ir).expect("IR yang sama harus dapat dikodegen");
    assert_eq!(first, second);
    assert!(first.contains("Halo native"));
}

#[test]
fn backend_menolak_ir_yang_belum_didukung_dan_tidak_fallback_ke_interpreter() {
    let ir = compile("nama adalah \"Halo\"\ntambahkan nama ke daftar\n");
    let backend = NativeAndroidAarch64Backend::new();
    let error = backend.codegen(&ir).expect_err("IR yang belum didukung harus gagal eksplisit");
    assert_eq!(error.code(), "NATIVE-CODEGEN-UNSUPPORTED");
}

#[test]
fn spesifikasi_toolchain_android_aarch64_memakai_executable() {
    let toolchain = NativeAndroidAarch64Backend::new().toolchain();
    assert_eq!(toolchain.target().triple(), "aarch64-linux-android24");
    assert_eq!(toolchain.artifact(), &ArtifactFormat::Executable);
    assert_eq!(toolchain.capability(), ToolchainCapability::BuildAndExecute);
}
