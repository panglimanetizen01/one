use crate::token::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration { name: String, value: Expr, span: Span },
    Assignment { name: String, value: Expr, span: Span },
    Action { name: String, args: Vec<Expr>, span: Span },
    Conditional { condition: Expr, then_block: Vec<Statement>, else_block: Vec<Statement>, span: Span },
    Loop { kind: LoopKind, body: Vec<Statement>, span: Span },
    Function { name: String, params: Vec<String>, result: Expr, body: Vec<Statement>, span: Span },
    Event { trigger: Vec<String>, body: Vec<Statement>, span: Span },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoopKind {
    ForEach { variable: String, iterable: Expr },
    While { condition: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Text(String),
    Number(i64),
    Boolean(bool),
    Nothing,
    Identifier(String),
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
    AtLeast,
    AtMost,
}
