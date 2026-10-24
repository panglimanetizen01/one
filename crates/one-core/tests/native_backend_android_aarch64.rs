use std::{fs, process::Command};

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
fn backend_menolak_target_arsitektur_yang_salah() {
    let backend = NativeAndroidAarch64Backend::new();
    let target = TargetSpec::hosted(TargetArchitecture::X86_64, TargetEnvironment::Linux)
        .with_backend(backend.id());
    assert!(backend.validate(&target).is_err());
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
    let ir = compile(
        "umur adalah 18\nkalau umur sudah 17 tahun\n    tampilkan \"Boleh masuk\"\n",
    );
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

#[test]
#[ignore = "membutuhkan Clang Android AArch64 dan Android emulator; set ONE_ANDROID_CLANG dan ONE_ANDROID_ADB"]
fn build_native_menghasilkan_dan_menjalankan_elf_android_aarch64() {
    let root = std::env::temp_dir().join(format!("one-native-test-{}", std::process::id()));
    fs::create_dir_all(&root).expect("temp dir harus dapat dibuat");
    let output = root.join("hello-android-aarch64");
    let ir = compile("tampilkan \"ONE_NATIVE_BACKEND_OK\"\n");
    let artifact = NativeAndroidAarch64Backend::new()
        .build(&ir, &output)
        .expect("backend harus menghasilkan artifact native");
    assert!(artifact.path.is_file(), "artifact Android tidak dibuat");

    // Assert the actual artifact format/architecture before handing it to Android.
    // A successful cross-build alone must not be confused with runtime acceptance.
    let elf = fs::read(&artifact.path).expect("artifact ELF harus dapat dibaca");
    assert!(elf.len() >= 20, "artifact terlalu pendek untuk header ELF");
    assert_eq!(&elf[0..4], b"\x7fELF", "artifact harus ELF");
    assert_eq!(elf[4], 2, "artifact harus ELFCLASS64");
    assert_eq!(elf[5], 1, "artifact harus little-endian");
    assert_eq!(u16::from_le_bytes([elf[18], elf[19]]), 183, "e_machine harus EM_AARCH64");

    let adb = std::env::var("ONE_ANDROID_ADB")
        .expect("ONE_ANDROID_ADB harus menunjuk ke adb yang terhubung ke emulator Android");
    let remote = format!("/data/local/tmp/one-native-test-{}", std::process::id());
    let artifact_path = artifact.path.to_str().expect("path artifact harus UTF-8");
    let push = Command::new(&adb)
        .args(["push", artifact_path, &remote])
        .output()
        .expect("adb push harus dapat dijalankan");
    assert!(
        push.status.success(),
        "gagal mengirim ELF ke emulator: {}",
        String::from_utf8_lossy(&push.stderr)
    );

    let chmod = Command::new(&adb)
        .args(["shell", "chmod", "700", &remote])
        .output()
        .expect("adb chmod harus dapat dijalankan");
    assert!(
        chmod.status.success(),
        "gagal mengatur izin ELF di emulator: {}",
        String::from_utf8_lossy(&chmod.stderr)
    );

    let run = Command::new(&adb)
        .args(["shell", &remote])
        .output()
        .expect("ELF Android harus dapat dijalankan melalui adb shell");
    assert!(
        run.status.success(),
        "ELF Android gagal dieksekusi: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&run.stdout).trim(),
        "ONE_NATIVE_BACKEND_OK"
    );

    let _ = Command::new(&adb).args(["shell", "rm", "-f", &remote]).status();
    let _ = fs::remove_dir_all(root);
}
