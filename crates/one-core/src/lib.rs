pub mod diagnostic;
pub mod lexer;
pub mod token;

pub use diagnostic::{Diagnostic, DiagnosticBundle, Severity};
pub use lexer::lex;
pub use token::{Span, Token, TokenKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_smoke_contract() {
        let tokens = lex("nama adalah \"Budi\"\n").expect("source should lex");
        assert_eq!(tokens.last().map(|t| &t.kind), Some(&TokenKind::Eof));
    }
}
