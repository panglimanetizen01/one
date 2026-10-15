#![allow(clippy::possible_missing_else)]
#![allow(clippy::collapsible_if)]

use std::collections::HashMap;
use crate::ast::{BinaryOp, Expr, LoopKind, Program, Statement};
use crate::diagnostic::{Diagnostic, DiagnosticBundle};
use crate::network::NetworkOperation;
use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct TypedProgram { pub program: Program }

pub fn analyze(program: &Program) -> Result<TypedProgram, DiagnosticBundle> {
    let mut analyzer = Analyzer { scopes: vec![HashMap::new()], functions: HashMap::new() };
    analyzer.register_functions(&program.statements)?;
    analyzer.analyze_block(&program.statements)?;
    Ok(TypedProgram { program: program.clone() })
}

pub fn analyze_network(operation: &NetworkOperation) -> Result<(), DiagnosticBundle> { operation.validate() }

struct Analyzer { scopes: Vec<HashMap<String, Type>>, functions: HashMap<String, usize> }

impl Analyzer {
    fn register_functions(&mut self, statements: &[Statement]) -> Result<(), DiagnosticBundle> { for statement in statements { if let Statement::Function { name, params, .. } = statement { if self.functions.insert(name.clone(), params.len()).is_some() { return Err(DiagnosticBundle::single(Diagnostic::error("E2013", format!("Pekerjaan '{}' sudah dibuat.", name), None))); } } } Ok(()) }
    fn current_scope(&mut self) -> &mut HashMap<String, Type> { self.scopes.last_mut().unwrap() }
    fn lookup(&self, name: &str) -> Option<Type> { self.scopes.iter().rev().find_map(|scope| scope.get(name).copied()) }
    fn analyze_block(&mut self, statements: &[Statement]) -> Result<(), DiagnosticBundle> { for statement in statements { self.analyze_statement(statement)?; } Ok(()) }
    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), DiagnosticBundle> {
        match statement {
            Statement::Declaration { name, value, span } => { if self.current_scope().contains_key(name) { return Err(DiagnosticBundle::single(Diagnostic::error("E2001", format!("Nama '{}' sudah dibuat di ruang lingkup ini.", name), Some(span.clone())))); } let ty = self.infer(value)?; self.current_scope().insert(name.clone(), ty); }
            Statement::Assignment { name, value, span } => { let expected = self.lookup(name).ok_or_else(|| DiagnosticBundle::single(Diagnostic::error("E2002", format!("Saya belum menemukan '{}'. Anda mungkin ingin membuatnya terlebih dahulu.", name), Some(span.clone()))))?; let actual = self.infer(value)?; if !compatible(expected, actual) { return Err(DiagnosticBundle::single(Diagnostic::error("E2003", format!("Nilai '{}' bertipe {}, tetapi membutuhkan {}.", name, actual.name(), expected.name()), Some(span.clone())))); } }
            Statement::Action { name, args, span } => match name.as_str() {
                "tampilkan" | "sapa" => { if args.len() != 1 { return Err(DiagnosticBundle::single(Diagnostic::error("E2004", format!("'{}' membutuhkan tepat satu nilai.", name), Some(span.clone())))); } self.infer(&args[0])?; }
                "buat daftar" => { if args.len() != 1 || !matches!(&args[0], Expr::Identifier(_)) { return Err(DiagnosticBundle::single(Diagnostic::error("E2005", "'buat daftar' membutuhkan nama daftar.", Some(span.clone())))); } if let Expr::Identifier(name) = &args[0] { self.current_scope().insert(name.clone(), Type::List); } }
                "tambahkan" | "hapus" => { if args.len() != 2 { return Err(DiagnosticBundle::single(Diagnostic::error("E2006", format!("'{}' membutuhkan nilai dan nama daftar.", name), Some(span.clone())))); } if let Expr::Identifier(list_name) = &args[1] { match self.lookup(list_name) { Some(Type::List | Type::Any) => {}, Some(_) => return Err(DiagnosticBundle::single(Diagnostic::error("E2007", format!("'{}' bukan daftar.", list_name), Some(span.clone())))), None => return Err(DiagnosticBundle::single(Diagnostic::error("E2008", format!("Saya belum menemukan daftar '{}'.", list_name), Some(span.clone())))), } } else { return Err(DiagnosticBundle::single(Diagnostic::error("E2009", "Nama daftar harus berupa identifier.", Some(span.clone())))); } }
                _ => { for arg in args { self.infer(arg)?; } }
            },
            Statement::Conditional { condition, then_block, else_block, .. } => { let ty = self.infer(condition)?; if !matches!(ty, Type::Boolean | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2010", "Kondisi harus menghasilkan boolean.", None))); } self.scopes.push(HashMap::new()); self.analyze_block(then_block)?; self.scopes.pop(); if !else_block.is_empty() { self.scopes.push(HashMap::new()); self.analyze_block(else_block)?; self.scopes.pop(); } }
            Statement::Loop { kind, body, .. } => match kind {
                LoopKind::ForEach { variable, iterable } => { let ty = self.infer(iterable)?; if !matches!(ty, Type::List | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2011", "Nilai setelah 'di' harus berupa daftar.", None))); } self.scopes.push(HashMap::new()); self.current_scope().insert(variable.clone(), Type::Any); self.analyze_block(body)?; self.scopes.pop(); }
                LoopKind::While { condition } => { let ty = self.infer(condition)?; if !matches!(ty, Type::Boolean | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2012", "Kondisi selama harus menghasilkan boolean.", None))); } self.scopes.push(HashMap::new()); self.analyze_block(body)?; self.scopes.pop(); }
            },
            Statement::Function { params, result, body, .. } => { self.scopes.push(HashMap::new()); for param in params { self.current_scope().insert(param.clone(), Type::Any); } self.infer(result)?; self.analyze_block(body)?; self.scopes.pop(); }
            Statement::Event { body, .. } => { self.scopes.push(HashMap::new()); self.analyze_block(body)?; self.scopes.pop(); }
        }
        Ok(())
    }
    fn infer(&self, expr: &Expr) -> Result<Type, DiagnosticBundle> {
        match expr {
            Expr::Text(_) => Ok(Type::Text), Expr::Number(_) => Ok(Type::Number), Expr::Boolean(_) => Ok(Type::Boolean), Expr::Nothing => Ok(Type::Nothing),
            Expr::Identifier(name) => self.lookup(name).ok_or_else(|| DiagnosticBundle::single(Diagnostic::error("E2014", format!("Saya belum menemukan '{}'.", name), None))),
            Expr::Binary { left, op, right } => { let l = self.infer(left)?; let r = self.infer(right)?; match op { BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => if matches!(l, Type::Number | Type::Any) && matches!(r, Type::Number | Type::Any) { Ok(Type::Number) } else { Err(DiagnosticBundle::single(Diagnostic::error("E2015", "Operasi hitung membutuhkan angka.", None))) }, BinaryOp::Equal | BinaryOp::Greater | BinaryOp::Less | BinaryOp::AtLeast | BinaryOp::AtMost => if compatible(l, r) || l == Type::Any || r == Type::Any { Ok(Type::Boolean) } else { Err(DiagnosticBundle::single(Diagnostic::error("E2016", "Dua nilai dibandingkan dengan tipe yang tidak kompatibel.", None))) } } }
            Expr::Call { name, args } => { let arity = self.functions.get(name).copied().ok_or_else(|| DiagnosticBundle::single(Diagnostic::error("E2017", format!("Pekerjaan '{}' belum dibuat.", name), None)))?; if arity != args.len() { return Err(DiagnosticBundle::single(Diagnostic::error("E2018", format!("Pekerjaan '{}' membutuhkan {} parameter.", name, arity), None))); } for arg in args { self.infer(arg)?; } Ok(Type::Any) }
        }
    }
}
fn compatible(expected: Type, actual: Type) -> bool { expected == actual || expected == Type::Any || actual == Type::Any }
