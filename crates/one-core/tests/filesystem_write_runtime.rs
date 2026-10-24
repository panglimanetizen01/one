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
    std::env::current_dir().unwrap().join(format!("one-filesystem-write-{nanos}.txt")).to_string_lossy().replace('\\', "/")
}

#[test]
fn filesystem_write_requires_explicit_capability_and_writes_target() {
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
    allowed.execute(&ir).expect("filesystem write should execute with explicit filesystem capability and target scope");
    assert_eq!(fs::read_to_string(&path).unwrap(), "halo dari One");
    let _ = fs::remove_file(path);
}
