use one_core::ir::{IrExpr, IrLoopKind, IrProgram, IrStatement};
use one_core::native_backend::{NativeLinuxAarch64Backend, NativeLinuxX86_64Backend};

fn backend() -> NativeLinuxAarch64Backend {
    NativeLinuxAarch64Backend::new()
}

#[test]
fn data_reaches_native_codegen() {
    let program = IrProgram {
        statements: vec![IrStatement::Declaration {
            name: "nilai".into(),
            value: IrExpr::Number(42),
        }],
    };
    let source = backend().codegen(&program).expect("DATA must reach native codegen");
    assert!(source.contains("long long nilai = 42;"));
}

#[test]
fn action_reaches_native_codegen() {
    let program = IrProgram {
        statements: vec![IrStatement::Action {
            name: "tampilkan".into(),
            args: vec![IrExpr::Text("halo".into())],
        }],
    };
    let source = backend().codegen(&program).expect("ACTION must reach native codegen");
    assert!(source.contains("puts(\"halo\");"));
}

#[test]
fn condition_reaches_native_codegen() {
    let program = IrProgram {
        statements: vec![IrStatement::Conditional {
            condition: IrExpr::Boolean(true),
            then_block: vec![IrStatement::Action {
                name: "tampilkan".into(),
                args: vec![IrExpr::Text("ya".into())],
            }],
            else_block: vec![IrStatement::Action {
                name: "tampilkan".into(),
                args: vec![IrExpr::Text("tidak".into())],
            }],
        }],
    };
    let source = backend().codegen(&program).expect("CONDITION must reach native codegen");
    assert!(source.contains("if (true)"));
    assert!(source.contains("else"));
}

#[test]
fn loop_reaches_native_codegen() {
    let program = IrProgram {
        statements: vec![IrStatement::Loop {
            kind: IrLoopKind::While {
                condition: IrExpr::Boolean(false),
            },
            body: vec![IrStatement::Action {
                name: "tampilkan".into(),
                args: vec![IrExpr::Text("loop".into())],
            }],
        }],
    };
    let source = backend().codegen(&program).expect("LOOP must reach native codegen");
    assert!(source.contains("while (false)"));
}

#[test]
fn function_reaches_native_codegen() {
    let program = IrProgram {
        statements: vec![IrStatement::Function {
            name: "jumlah".into(),
            params: vec!["nilai".into()],
            result: IrExpr::Identifier("nilai".into()),
            body: vec![],
        }],
    };
    let source = backend().codegen(&program).expect("FUNCTION must reach native codegen");
    assert!(source.contains("long long jumlah(long long nilai)"));
}

#[test]
fn event_reaches_native_codegen() {
    let program = IrProgram {
        statements: vec![IrStatement::Event {
            trigger: vec!["mulai".into()],
            body: vec![IrStatement::Action {
                name: "tampilkan".into(),
                args: vec![IrExpr::Text("event".into())],
            }],
        }],
    };
    let source = backend().codegen(&program).expect("EVENT must reach native codegen");
    assert!(source.contains("void one_event_mulai(void)"));
    assert!(source.contains("one_dispatch_event_mulai();"));
}

#[test]
fn all_primary_families_codegen_is_deterministic() {
    let program = IrProgram {
        statements: vec![
            IrStatement::Declaration {
                name: "nilai".into(),
                value: IrExpr::Number(1),
            },
            IrStatement::Action {
                name: "tampilkan".into(),
                args: vec![IrExpr::Identifier("nilai".into())],
            },
            IrStatement::Conditional {
                condition: IrExpr::Boolean(true),
                then_block: vec![],
                else_block: vec![],
            },
            IrStatement::Loop {
                kind: IrLoopKind::While {
                    condition: IrExpr::Boolean(false),
                },
                body: vec![],
            },
            IrStatement::Function {
                name: "identitas".into(),
                params: vec!["x".into()],
                result: IrExpr::Identifier("x".into()),
                body: vec![],
            },
            IrStatement::Event {
                trigger: vec!["mulai".into()],
                body: vec![],
            },
        ],
    };
    let first = backend().codegen(&program).expect("codegen must succeed");
    let second = backend().codegen(&program).expect("codegen must be deterministic");
    assert_eq!(first, second);
}

#[test]
fn unsupported_construct_has_structured_diagnostic() {
    let program = IrProgram {
        statements: vec![IrStatement::Network {
            operation: one_core::ir::IrNetworkOperation::semantic(
                one_core::network::NetworkOperation::Receive {
                    connection: one_core::network::NetworkConnectionId(1),
                },
            ),
        }],
    };
    let error = backend().codegen(&program).expect_err("unsupported construct must fail");
    assert_eq!(error.code(), "NATIVE-CODEGEN-UNSUPPORTED");
}

#[test]
fn invalid_type_has_structured_diagnostic() {
    let program = IrProgram {
        statements: vec![
            IrStatement::Declaration {
                name: "nilai".into(),
                value: IrExpr::Number(1),
            },
            IrStatement::Assignment {
                name: "nilai".into(),
                value: IrExpr::Text("salah".into()),
            },
        ],
    };
    let error = backend().codegen(&program).expect_err("invalid type must fail");
    assert_eq!(error.code(), "NATIVE-CODEGEN-TYPE");
}

#[test]
fn invalid_lowering_has_structured_diagnostic() {
    let program = IrProgram {
        statements: vec![IrStatement::Assignment {
            name: "tidak_ada".into(),
            value: IrExpr::Number(1),
        }],
    };
    let error = backend().codegen(&program).expect_err("invalid lowering must fail");
    assert_eq!(error.code(), "NATIVE-CODEGEN-UNDEFINED");
}

#[test]
fn invalid_target_backend_is_rejected_before_build() {
    let backend = NativeLinuxX86_64Backend::new();
    let target = backend.target();
    assert!(target.validate_backend(&backend).is_ok());
}
