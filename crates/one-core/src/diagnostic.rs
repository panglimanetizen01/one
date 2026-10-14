use crate::token::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
    pub suggestions: Vec<String>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>, span: Option<Span>) -> Self {
        Self { severity: Severity::Error, code, message: message.into(), span, notes: Vec::new(), suggestions: Vec::new() }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBundle(pub Vec<Diagnostic>);

impl DiagnosticBundle {
    pub fn single(diagnostic: Diagnostic) -> Self { Self(vec![diagnostic]) }
}
