use std::collections::HashMap;
use crate::ast::BinaryOp;
use crate::capability::{Capability, CapabilitySet};
use crate::ir::{IrExpr, IrLoopKind, IrProgram, IrStatement};
use crate::runtime::{Environment, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult { pub output: Vec<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError { pub code: &'static str, pub message: String }

pub struct Interpreter {
    pub capabilities: CapabilitySet,
    pub max_steps: usize,
    functions: HashMap<String, (Vec<String>, IrExpr, Vec<IrStatement>)>,
    events: HashMap<String, Vec<IrStatement>>,
    steps: usize,
}

impl Interpreter {
    pub fn new(capabilities: CapabilitySet) -> Self { Self { capabilities, max_steps: 100_000, functions: HashMap::new(), events: HashMap::new(), steps: 0 } }
    pub fn execute(&mut self, program: &IrProgram) -> Result<ExecutionResult, RuntimeError> {
        self.functions.clear(); self.events.clear(); self.steps = 0;
        self.register_definitions(&program.statements);
        let mut env = Environment::new(); let mut output = Vec::new();
        self.exec_block(&program.statements, &mut env, &mut output)?;
        Ok(ExecutionResult { output })
    }
    pub fn dispatch_event(&mut self, trigger: &str) -> Result<ExecutionResult, RuntimeError> {
        let body = self.events.get(trigger).cloned().ok_or_else(|| RuntimeError { code: "R3001", message: format!("Event '{}' belum terdaftar.", trigger) })?;
        let mut env = Environment::new(); let mut output = Vec::new(); self.steps = 0; self.exec_block(&body, &mut env, &mut output)?; Ok(ExecutionResult { output })
    }
    fn register_definitions(&mut self, statements: &[IrStatement]) { for statement in statements { match statement { IrStatement::Function { name, params, result, body } => { self.functions.insert(name.clone(), (params.clone(), result.clone(), body.clone())); }, IrStatement::Event { trigger, body } => { self.events.insert(trigger.join(" "), body.clone()); }, _ => {} } } }
    fn tick(&mut self) -> Result<(), RuntimeError> { self.steps += 1; if self.steps > self.max_steps { Err(RuntimeError { code: "R3002", message: "Program melebihi batas langkah eksekusi yang aman.".into() }) } else { Ok(()) } }
    fn exec_block(&mut self, statements: &[IrStatement], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { for statement in statements { self.tick()?; self.exec_statement(statement, env, output)?; } Ok(()) }

    fn exec_statement(&mut self, statement: &IrStatement, env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> {
        match statement {
            IrStatement::Declaration { name, value } => { let value = self.eval(value, env)?; env.define(name.clone(), value); }
            IrStatement::Assignment { name, value } => { let value = self.eval(value, env)?; if !env.assign(name, value) { return Err(RuntimeError { code: "R3003", message: format!("Saya belum menemukan '{}'.", name) }); } }
            IrStatement::Action { name, args } => self.exec_action(name, args, env, output)?,
            IrStatement::Conditional { condition, then_block, else_block } => match self.eval(condition, env)? { Value::Boolean(true) => { env.push(); self.exec_block(then_block, env, output)?; env.pop(); }, Value::Boolean(false) => { env.push(); self.exec_block(else_block, env, output)?; env.pop(); }, _ => return Err(RuntimeError { code: "R3004", message: "Kondisi harus menghasilkan boolean.".into() }) },
            IrStatement::Loop { kind, body } => match kind {
                IrLoopKind::ForEach { variable, iterable } => { let list = self.eval(iterable, env)?; if let Value::List(values) = list { for value in values { env.push(); env.define(variable.clone(), value); self.exec_block(body, env, output)?; env.pop(); } } else { return Err(RuntimeError { code: "R3005", message: "Loop 'untuk setiap' membutuhkan daftar.".into() }); } }
                IrLoopKind::While { condition } => loop { if !matches!(self.eval(condition, env)?, Value::Boolean(true)) { break; } env.push(); self.exec_block(body, env, output)?; env.pop(); self.tick()?; }
            },
            IrStatement::Function { .. } | IrStatement::Event { .. } => {}
        }
        Ok(())
    }

    fn exec_action(&mut self, name: &str, args: &[IrExpr], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> {
        match name {
            "tampilkan" => { if args.len() != 1 { return Err(RuntimeError { code: "R3006", message: "'tampilkan' membutuhkan tepat satu nilai.".into() }); } output.push(self.eval(&args[0], env)?.display()); }
            "sapa" => { if args.len() != 1 { return Err(RuntimeError { code: "R3007", message: "'sapa' membutuhkan satu nama.".into() }); } output.push(format!("Halo, {}!", self.eval(&args[0], env)?.display())); }
            "buat daftar" => { if args.len() != 1 { return Err(RuntimeError { code: "R3008", message: "'buat daftar' membutuhkan nama daftar.".into() }); } if let IrExpr::Identifier(name) = &args[0] { env.define(name.clone(), Value::List(Vec::new())); } else { return Err(RuntimeError { code: "R3009", message: "Nama daftar harus berupa identifier.".into() }); } }
            "tambahkan" | "hapus" => {
                if args.len() != 2 { return Err(RuntimeError { code: "R3010", message: format!("'{}' membutuhkan nilai dan nama daftar.", name) }); }
                let item = match &args[0] { IrExpr::Identifier(word) => env.get(word).unwrap_or_else(|| Value::Text(word.clone())), _ => self.eval(&args[0], env)? };
                let list_name = match &args[1] { IrExpr::Identifier(value) => value, _ => return Err(RuntimeError { code: "R3011", message: "Nama daftar harus berupa identifier.".into() }) };
                let mut list = env.get(list_name).ok_or_else(|| RuntimeError { code: "R3012", message: format!("Saya belum menemukan daftar '{}'.", list_name) })?;
                match &mut list {
                    Value::List(values) if name == "tambahkan" => values.push(item),
                    Value::List(values) => { if let Some(index) = values.iter().position(|v| v == &item) { values.remove(index); } }
                    _ => return Err(RuntimeError { code: "R3013", message: format!("'{}' bukan daftar.", list_name) }),
                }
                let _ = env.assign(list_name, list);
            }
            "gunakan proses" => { if !self.capabilities.permits(Capability::Process) { return Err(RuntimeError { code: "R3014", message: "Kemampuan proses belum diberikan kepada program.".into() }); } }
            _ => return Err(RuntimeError { code: "R3015", message: format!("Aksi '{}' belum memiliki implementasi standard library.", name) }),
        }
        Ok(())
    }

    fn eval(&mut self, expr: &IrExpr, env: &Environment) -> Result<Value, RuntimeError> {
        match expr {
            IrExpr::Text(v) => Ok(Value::Text(v.clone())), IrExpr::Number(v) => Ok(Value::Number(*v)), IrExpr::Boolean(v) => Ok(Value::Boolean(*v)), IrExpr::Nothing => Ok(Value::Nothing),
            IrExpr::Identifier(name) => env.get(name).ok_or_else(|| RuntimeError { code: "R3016", message: format!("Saya belum menemukan '{}'.", name) }),
            IrExpr::Binary { left, op, right } => { let left_value = self.eval(left, env)?; let right_value = self.eval(right, env)?; self.eval_binary(*op, left_value, right_value) }
            IrExpr::Call { name, args } => self.call_function(name, args, env),
        }
    }

    fn eval_binary(&self, op: BinaryOp, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (op, left, right) {
            (BinaryOp::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)), (BinaryOp::Subtract, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (BinaryOp::Multiply, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)), (BinaryOp::Divide, Value::Number(a), Value::Number(b)) if b != 0 => Ok(Value::Number(a / b)),
            (BinaryOp::Divide, _, Value::Number(0)) => Err(RuntimeError { code: "R3017", message: "Pembagian dengan nol tidak diperbolehkan.".into() }),
            (BinaryOp::Equal, a, b) => Ok(Value::Boolean(a == b)), (BinaryOp::Greater, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (BinaryOp::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)), (BinaryOp::AtMost, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            (BinaryOp::AtLeast, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)), (BinaryOp::Add, Value::Text(a), Value::Text(b)) => Ok(Value::Text(format!("{}{}", a, b))),
            _ => Err(RuntimeError { code: "R3018", message: "Operasi tidak cocok dengan tipe nilai.".into() }),
        }
    }

    fn call_function(&mut self, name: &str, args: &[IrExpr], caller_env: &Environment) -> Result<Value, RuntimeError> {
        let (params, result, body) = self.functions.get(name).cloned().ok_or_else(|| RuntimeError { code: "R3019", message: format!("Pekerjaan '{}' belum dibuat.", name) })?;
        if params.len() != args.len() { return Err(RuntimeError { code: "R3020", message: format!("Pekerjaan '{}' membutuhkan {} parameter.", name, params.len()) }); }
        let mut env = caller_env.clone(); env.push();
        for (param, arg) in params.iter().zip(args) { let value = self.eval(arg, caller_env)?; env.define(param.clone(), value); }
        let mut ignored_output = Vec::new(); self.exec_block(&body, &mut env, &mut ignored_output)?; self.eval(&result, &env)
    }
}
