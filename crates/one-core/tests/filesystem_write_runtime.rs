use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use one_core::capability::Capability;
use one_core::governance::{ExecutionPolicy, ScopeRule};
use one_core::interpreter::Interpreter;
use one_core::ir::lower;
use one_core::parser::parse;
use one_core::semantic::analyze;

fn unique_temp_path() -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("one-filesystem-write-{nanos}.txt")).to_string_lossy().replace('\\', "/")
}

#[test]
fn filesystem_write_requires_capability_but_fails_closed_without_verified_write_backend() {
    let path = unique_temp_path();
    let source = format!("tulis ke berkas \"{path}\" dengan \"halo dari One\"\n");
    let program = parse(&source).expect("filesystem write grammar should parse");
    let typed = analyze(&program).expect("filesystem write should pass semantic analysis");
    let ir = lower(&typed).expect("filesystem write should lower to IR");

    let mut denied = Interpreter::with_orchestration_policy(ExecutionPolicy::default());
    let error = denied.execute(&ir).expect_err("filesystem write must be denied without explicit capability");
    assert_eq!(error.code, "N3209");

    let policy = ExecutionPolicy::default()
        .grant(Capability::Filesystem)
        .allow_scope(Capability::Filesystem, ScopeRule::Target(path.clone()));
    let mut allowed = Interpreter::with_orchestration_policy(policy);
    let error = allowed.execute(&ir).expect_err("filesystem write must remain unavailable until a race-resistant backend is verified");
    assert_eq!(error.code, "N3212");
    assert!(error.message.contains("UnsupportedPlatform"));
    assert!(!std::path::Path::new(&path).exists(), "fail-closed write must not create an outside file");
    let _ = fs::remove_file(path);
}
