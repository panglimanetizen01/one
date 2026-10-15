pub mod ast;
pub mod capability;
pub mod diagnostic;
pub mod interop;
pub mod interpreter;
pub mod ir;
pub mod lexer;
pub mod network;
pub mod parser;
pub mod platform;
pub mod runtime;
pub mod semantic;
pub mod stdlib;
pub mod token;
pub mod types;

pub use ast::{BinaryOp, Expr, LoopKind, Program, Statement};
pub use capability::{Capability, CapabilitySet};
pub use diagnostic::{Diagnostic, DiagnosticBundle, Severity};
pub use interop::{run_process, ProcessResult, ProcessSpec};
pub use interpreter::{ExecutionResult, Interpreter, NetworkExecutionResult, RuntimeError};
pub use ir::{lower, lower_network, IrExpr, IrLoopKind, IrNetworkOperation, IrProgram, IrStatement};
pub use lexer::lex;
pub use network::{NetworkAddressKind, NetworkConnectionId, NetworkConstraints, NetworkDiagnosticStage, NetworkFailure, NetworkOperation, NetworkProtocolFamily, NetworkSecurity, NetworkTarget, NetworkTransport, ProtocolCapability, ProtocolRegistry, ProtocolSupportStatus};
pub use parser::parse;
pub use semantic::{analyze, TypedProgram};
pub use token::{Span, Token, TokenKind};
pub use types::Type;

#[cfg(test)]
mod tests {
    use super::*;

    fn compile(source: &str) -> IrProgram {
        let program = parse(source).expect("source should parse");
        let typed = analyze(&program).expect("source should analyze");
        lower(&typed).expect("source should lower")
    }

    #[test]
    fn core_pipeline_smoke_contract() {
        let ir = compile("nama adalah \"Budi\"\ntampilkan nama\n");
        let mut interpreter = Interpreter::new(CapabilitySet::none());
        let result = interpreter.execute(&ir).expect("source should execute");
        assert_eq!(result.output, vec!["Budi"]);
    }
}
