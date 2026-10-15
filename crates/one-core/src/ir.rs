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

impl IrProgram {
    pub fn canonical_text(&self) -> String {
        let mut out = String::new();
        write_statements(&mut out, &self.statements, 0);
        out
    }
}

fn write_statements(out: &mut String, statements: &[IrStatement], indent: usize) {
    for statement in statements {
        write_indent(out, indent);
        match statement {
            IrStatement::Declaration { name, value } => {
                out.push_str("declare "); out.push_str(name); out.push_str(" = "); write_expr(out, value); out.push('\n');
            }
            IrStatement::Assignment { name, value } => {
                out.push_str("assign "); out.push_str(name); out.push_str(" = "); write_expr(out, value); out.push('\n');
            }
            IrStatement::Action { name, args } => {
                out.push_str("action "); out.push_str(name); write_expr_list(out, args); out.push('\n');
            }
            IrStatement::Conditional { condition, then_block, else_block } => {
                out.push_str("if "); write_expr(out, condition); out.push('\n');
                write_statements(out, then_block, indent + 1);
                if !else_block.is_empty() {
                    write_indent(out, indent); out.push_str("else\n");
                    write_statements(out, else_block, indent + 1);
                }
                write_indent(out, indent); out.push_str("end\n");
            }
            IrStatement::Loop { kind, body } => {
                match kind {
                    IrLoopKind::ForEach { variable, iterable } => { out.push_str("foreach "); out.push_str(variable); out.push_str(" in "); write_expr(out, iterable); out.push('\n'); }
                    IrLoopKind::While { condition } => { out.push_str("while "); write_expr(out, condition); out.push('\n'); }
                }
                write_statements(out, body, indent + 1);
                write_indent(out, indent); out.push_str("end\n");
            }
            IrStatement::Function { name, params, result, body } => {
                out.push_str("function "); out.push_str(name); out.push('(');
                for (index, param) in params.iter().enumerate() { if index > 0 { out.push(','); } out.push_str(param); }
                out.push_str(") = "); write_expr(out, result); out.push('\n');
                write_statements(out, body, indent + 1);
                write_indent(out, indent); out.push_str("end\n");
            }
            IrStatement::Event { trigger, body } => {
                out.push_str("event ");
                for (index, word) in trigger.iter().enumerate() { if index > 0 { out.push(' '); } out.push_str(word); }
                out.push('\n');
                write_statements(out, body, indent + 1);
                write_indent(out, indent); out.push_str("end\n");
            }
        }
    }
}

fn write_expr_list(out: &mut String, args: &[IrExpr]) {
    if args.is_empty() { return; }
    out.push(' ');
    for (index, arg) in args.iter().enumerate() { if index > 0 { out.push_str(", "); } write_expr(out, arg); }
}

fn write_expr(out: &mut String, expr: &IrExpr) {
    match expr {
        IrExpr::Text(value) => { out.push('"'); for ch in value.chars() { match ch { '\\' => out.push_str("\\\\"), '"' => out.push_str("\\\""), '\n' => out.push_str("\\n"), '\r' => out.push_str("\\r"), '\t' => out.push_str("\\t"), other => out.push(other) } } out.push('"'); }
        IrExpr::Number(value) => out.push_str(&value.to_string()),
        IrExpr::Boolean(value) => out.push_str(if *value { "true" } else { "false" }),
        IrExpr::Nothing => out.push_str("nothing"),
        IrExpr::Identifier(value) => out.push_str(value),
        IrExpr::Binary { left, op, right } => { out.push('('); write_expr(out, left); out.push(' '); out.push_str(binary_op_name(*op)); out.push(' '); write_expr(out, right); out.push(')'); }
        IrExpr::Call { name, args } => { out.push_str(name); out.push('('); for (index, arg) in args.iter().enumerate() { if index > 0 { out.push_str(", "); } write_expr(out, arg); } out.push(')'); }
    }
}

fn binary_op_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Subtract => "subtract",
        BinaryOp::Multiply => "multiply",
        BinaryOp::Divide => "divide",
        BinaryOp::Equal => "equal",
        BinaryOp::NotEqual => "not_equal",
        BinaryOp::LessThan => "less_than",
        BinaryOp::LessOrEqual => "less_or_equal",
        BinaryOp::GreaterThan => "greater_than",
        BinaryOp::GreaterOrEqual => "greater_or_equal",
    }
}

fn write_indent(out: &mut String, indent: usize) { for _ in 0..indent { out.push_str("  "); } }

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
