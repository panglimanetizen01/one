use one_core::capability::Capability;
use one_core::ir::{lower, IrStatement};
use one_core::orchestration::{EffectClass, SemanticCapabilityId};
use one_core::parser::parse;
use one_core::semantic::analyze;

#[test]
fn filesystem_write_lowers_to_specific_write_capability() {
    let program = parse("tulis ke berkas \"target.txt\" dengan \"halo\"\n").expect("source should parse");
    let typed = analyze(&program).expect("source should analyze");
    let ir = lower(&typed).expect("source should lower");

    assert!(matches!(
        ir.statements.first(),
        Some(IrStatement::Orchestration {
            semantic: SemanticCapabilityId::FilesystemWrite,
            capability: Capability::FilesystemWrite,
            effect: EffectClass::Write,
            ..
        })
    ));
}
