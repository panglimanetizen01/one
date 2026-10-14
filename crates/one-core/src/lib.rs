pub mod ast;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod token;

pub use ast::{BinaryOp, Expr, LoopKind, Program, Statement};
pub use diagnostic::{Diagnostic, DiagnosticBundle, Severity};
pub use lexer::lex;
pub use parser::parse;
pub use token::{Span, Token, TokenKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_smoke_contract() {
        let program = parse("nama adalah \"Budi\"\n").expect("source should parse");
        assert!(matches!(program.statements.first(), Some(Statement::Declaration { .. })));
    }
}
