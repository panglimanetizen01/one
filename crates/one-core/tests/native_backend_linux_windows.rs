use one_core::{
    analyze, lower, parse, ArtifactFormat, IrProgram, NativeLinuxAarch64Backend,
    NativeLinuxX86_64Backend, NativeWindowsX86_64Backend, TargetArchitecture, TargetBackend,
    TargetEnvironment, ToolchainCapability,
};

fn compile(source: &str) -> IrProgram {
    let program = parse(source).expect("source harus dapat diparse");
    let typed = analyze(&program).expect("source harus dapat dianalisis");
    lower(&typed).expect("source harus dapat diturunkan ke IR")
}

#[test]
fn linux_aarch64_memiliki_target_dan_toolchain_yang_tepat() {
    let backend = NativeLinuxAarch64Backend::new();
    let target = backend.target();

    backend.validate(&target).expect("target harus valid");
    assert_eq!(target.architecture(), &TargetArchitecture::Aarch64);
    assert_eq!(target.environment(), TargetEnvironment::Linux);

    let toolchain = backend.toolchain();
    assert_eq!(toolchain.target().triple(), "aarch64-unknown-linux-gnu");
    assert_eq!(toolchain.artifact(), &ArtifactFormat::Executable);
    assert_eq!(toolchain.capability(), ToolchainCapability::BuildAndExecute);
}

#[test]
fn linux_x86_64_memiliki_target_dan_toolchain_yang_tepat() {
    let backend = NativeLinuxX86_64Backend::new();
    let target = backend.target();

    backend.validate(&target).expect("target harus valid");
    assert_eq!(target.architecture(), &TargetArchitecture::X86_64);
    assert_eq!(target.environment(), TargetEnvironment::Linux);

    let toolchain = backend.toolchain();
    assert_eq!(toolchain.target().triple(), "x86_64-unknown-linux-gnu");
    assert_eq!(toolchain.artifact(), &ArtifactFormat::Executable);
    assert_eq!(toolchain.capability(), ToolchainCapability::BuildAndExecute);
}

#[test]
fn windows_x86_64_memiliki_target_dan_toolchain_yang_tepat() {
    let backend = NativeWindowsX86_64Backend::new();
    let target = backend.target();

    backend.validate(&target).expect("target harus valid");
    assert_eq!(target.architecture(), &TargetArchitecture::X86_64);
    assert_eq!(target.environment(), TargetEnvironment::Windows);

    let toolchain = backend.toolchain();
    assert_eq!(toolchain.target().triple(), "x86_64-pc-windows-gnu");
    assert_eq!(toolchain.artifact(), &ArtifactFormat::Executable);
    assert_eq!(toolchain.capability(), ToolchainCapability::BuildAndExecute);
}

#[test]
fn ketiga_backend_menghasilkan_codegen_deterministik() {
    let ir = compile("tampilkan \"ONE_NATIVE_BACKEND_OK\"\n");

    let linux_arm = NativeLinuxAarch64Backend::new();
    assert_eq!(
        linux_arm.codegen(&ir).unwrap(),
        linux_arm.codegen(&ir).unwrap()
    );

    let linux_x64 = NativeLinuxX86_64Backend::new();
    assert_eq!(
        linux_x64.codegen(&ir).unwrap(),
        linux_x64.codegen(&ir).unwrap()
    );

    let windows = NativeWindowsX86_64Backend::new();
    assert_eq!(windows.codegen(&ir).unwrap(), windows.codegen(&ir).unwrap());
}

#[test]
fn ketiga_backend_menolak_ir_yang_belum_didukung() {
    let ir = compile("sapa \"X\"\n");

    let error = NativeLinuxAarch64Backend::new()
        .codegen(&ir)
        .expect_err("IR unsupported harus ditolak");
    assert_eq!(error.code(), "NATIVE-CODEGEN-UNSUPPORTED");

    let error = NativeLinuxX86_64Backend::new()
        .codegen(&ir)
        .expect_err("IR unsupported harus ditolak");
    assert_eq!(error.code(), "NATIVE-CODEGEN-UNSUPPORTED");

    let error = NativeWindowsX86_64Backend::new()
        .codegen(&ir)
        .expect_err("IR unsupported harus ditolak");
    assert_eq!(error.code(), "NATIVE-CODEGEN-UNSUPPORTED");
}

#[cfg(unix)]
#[test]
fn linux_x86_64_build_menggunakan_cross_linker_x86_64() {
    let backend = NativeLinuxX86_64Backend::new();
    let ir = compile("tampilkan \"ONE_NATIVE_LINUX_X86_64\"\n");

    let output = std::env::temp_dir().join("one-native-linux-x86-64-linker-test");
    let _ = std::fs::remove_file(&output);
    let _ = std::fs::remove_file(output.with_extension("c"));

    let linker = std::env::temp_dir().join(format!(
        "one-test-x86_64-linux-gnu-ld-{}",
        std::process::id()
    ));
    std::fs::write(
        &linker,
        b"#!/bin/sh\nexec x86_64-linux-gnu-ld \"$@\"\n",
    )
    .expect("fixture linker harus dapat dibuat");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&linker).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&linker, permissions).unwrap();
    }

    struct EnvGuard {
        previous: Option<std::ffi::OsString>,
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.previous {
                    Some(value) => std::env::set_var("ONE_LINUX_X86_64_LD", value),
                    None => std::env::remove_var("ONE_LINUX_X86_64_LD"),
                }
            }
        }
    }

    let _env_guard = EnvGuard {
        previous: std::env::var_os("ONE_LINUX_X86_64_LD"),
    };

    unsafe {
        std::env::set_var("ONE_LINUX_X86_64_LD", &linker);
    }

    let artifact = backend
        .build(&ir, &output)
        .expect("backend Linux x86_64 harus dapat membangun artifact");

    assert!(std::path::Path::new(&linker).is_file());

    assert_eq!(artifact.format, ArtifactFormat::Executable);
    assert!(artifact.path.is_file());

    let _ = std::fs::remove_file(&artifact.path);
}

#[test]
fn windows_x86_64_dapat_membangun_artifact_native() {
    let backend = NativeWindowsX86_64Backend::new();
    let ir = compile("tampilkan \"ONE_NATIVE_WINDOWS_BACKEND\"\n");

    let output = std::env::temp_dir().join("one-native-windows-backend-test.exe");
    let _ = std::fs::remove_file(&output);
    let _ = std::fs::remove_file(output.with_extension("c"));

    let artifact = backend
        .build(&ir, &output)
        .expect("backend Windows harus dapat membangun artifact");

    assert_eq!(artifact.format, ArtifactFormat::Executable);
    assert!(artifact.path.is_file());

    let _ = std::fs::remove_file(&artifact.path);
}

#[test]
fn linux_targets_adalah_target_gnu_linux_one() {
    let aarch64 = NativeLinuxAarch64Backend::new();
    let x86_64 = NativeLinuxX86_64Backend::new();

    assert_eq!(
        aarch64.toolchain().target().triple(),
        "aarch64-unknown-linux-gnu"
    );
    assert_eq!(
        x86_64.toolchain().target().triple(),
        "x86_64-unknown-linux-gnu"
    );
    assert_eq!(
        aarch64.toolchain().target().environment(),
        TargetEnvironment::Linux
    );
    assert_eq!(
        x86_64.toolchain().target().environment(),
        TargetEnvironment::Linux
    );
}

#[test]
fn target_linux_tidak_bergantung_pada_ketersediaan_compiler_host() {
    let aarch64 = NativeLinuxAarch64Backend::new().toolchain();
    let x86_64 = NativeLinuxX86_64Backend::new().toolchain();

    assert_eq!(aarch64.target().triple(), "aarch64-unknown-linux-gnu");
    assert_eq!(x86_64.target().triple(), "x86_64-unknown-linux-gnu");
    assert_eq!(aarch64.capability(), ToolchainCapability::BuildAndExecute);
    assert_eq!(x86_64.capability(), ToolchainCapability::BuildAndExecute);
}
