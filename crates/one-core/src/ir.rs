use crate::ast::{BinaryOp, Expr, LoopKind, Statement};
use crate::diagnostic::DiagnosticBundle;
use crate::semantic::TypedProgram;

#[derive(Debug, Clone, PartialEq)]
pub struct IrProgram {
    pub statements: Vec<IrStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrStatement {
    Declaration { name: String, value: IrExpr },
    Assignment { name: String, value: IrExpr },
    Action { name: String, args: Vec<IrExpr> },
    Conditional { condition: IrExpr, then_block: Vec<IrStatement>, else_block: Vec<IrStatement> },
    Loop { kind: IrLoopKind, body: Vec<IrStatement> },
    Function { name: String, params: Vec<String>, result: IrExpr, body: Vec<IrStatement> },
    Event { trigger: Vec<String>, body: Vec<IrStatement> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrLoopKind {
    ForEach { variable: String, iterable: IrExpr },
    While { condition: IrExpr },
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrExpr {
    Text(String),
    Number(i64),
    Boolean(bool),
    Nothing,
    Identifier(String),
    Binary { left: Box<IrExpr>, op: BinaryOp, right: Box<IrExpr> },
    Call { name: String, args: Vec<IrExpr> },
}

pub fn lower(program: &TypedProgram) -> Result<IrProgram, DiagnosticBundle> {
    Ok(IrProgram { statements: program.program.statements.iter().map(lower_statement).collect() })
}

fn lower_statement(statement: &Statement) -> IrStatement {
    match statement {
        Statement::Declaration { name, value, .. } => IrStatement::Declaration { name: name.clone(), value: lower_expr(value) },
        Statement::Assignment { name, value, .. } => IrStatement::Assignment { name: name.clone(), value: lower_expr(value) },
        Statement::Action { name, args, .. } => IrStatement::Action { name: name.clone(), args: args.iter().map(lower_expr).collect() },
        Statement::Conditional { condition, then_block, else_block, .. } => IrStatement::Conditional { condition: lower_expr(condition), then_block: then_block.iter().map(lower_statement).collect(), else_block: else_block.iter().map(lower_statement).collect() },
        Statement::Loop { kind, body, .. } => IrStatement::Loop { kind: match kind { LoopKind::ForEach { variable, iterable } => IrLoopKind::ForEach { variable: variable.clone(), iterable: lower_expr(iterable) }, LoopKind::While { condition } => IrLoopKind::While { condition: lower_expr(condition) } }, body: body.iter().map(lower_statement).collect() },
        Statement::Function { name, params, result, body, .. } => IrStatement::Function { name: name.clone(), params: params.clone(), result: lower_expr(result), body: body.iter().map(lower_statement).collect() },
        Statement::Event { trigger, body, .. } => IrStatement::Event { trigger: trigger.clone(), body: body.iter().map(lower_statement).collect() },
    }
}

fn lower_expr(expr: &Expr) -> IrExpr {
    match expr {
        Expr::Text(v) => IrExpr::Text(v.clone()),
        Expr::Number(v) => IrExpr::Number(*v),
        Expr::Boolean(v) => IrExpr::Boolean(*v),
        Expr::Nothing => IrExpr::Nothing,
        Expr::Identifier(v) => IrExpr::Identifier(v.clone()),
        Expr::Binary { left, op, right } => IrExpr::Binary { left: Box::new(lower_expr(left)), op: *op, right: Box::new(lower_expr(right)) },
        Expr::Call { name, args } => IrExpr::Call { name: name.clone(), args: args.iter().map(lower_expr).collect() },
    }
}
