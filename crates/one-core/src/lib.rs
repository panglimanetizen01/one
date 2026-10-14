pub mod ast;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod token;
pub mod types;

pub use ast::{BinaryOp, Expr, LoopKind, Program, Statement};
pub use diagnostic::{Diagnostic, DiagnosticBundle, Severity};
pub use lexer::lex;
pub use parser::parse;
pub use semantic::{analyze, TypedProgram};
pub use token::{Span, Token, TokenKind};
pub use types::Type;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_smoke_contract() {
        let program = parse("nama adalah \"Budi\"\ntampilkan nama\n").expect("source should parse");
        analyze(&program).expect("source should be semantically valid");
    }
}
