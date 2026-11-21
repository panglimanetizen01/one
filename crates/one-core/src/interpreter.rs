#![allow(clippy::possible_missing_else)]
#![allow(unused_imports)]
use std::collections::HashMap;
use std::sync::Arc;
use crate::filesystem::FilesystemAuthority;
use crate::ast::BinaryOp;
use crate::capability::{Capability, CapabilitySet};
use crate::adapter::{AdapterArgument, FileDeleteAdapter, FileReadAdapter, FileWriteAdapter, HttpGetAdapter, PlatformKind, ProcessAdapter};
use crate::database::SqliteAdapter;
use crate::governance::ExecutionPolicy;
use crate::ir::{IrExpr, IrLoopKind, IrNetworkOperation, IrProgram, IrStatement};
use crate::network_dns_adapter::DnsResolveAdapter;
use crate::orchestration::{AdapterId, CapabilityRequest};
use crate::orchestration_audit::AuditRecord;
use crate::orchestration_result::{normalize_result, NormalizationExpectation, NormalizedValue};
use crate::orchestrator::{OrchestrationError, Orchestrator};
use crate::runtime::{Environment, Value};
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ExecutionResult { pub output: Vec<String> }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct NetworkExecutionResult { pub description: String, pub performed_external_io: bool }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct RuntimeError { pub code: &'static str, pub message: String }
pub struct Interpreter { pub capabilities: CapabilitySet, pub max_steps: usize, functions: HashMap<String, (Vec<String>, IrExpr, Vec<IrStatement>)>, events: HashMap<String, Vec<IrStatement>>, environment: Option<Environment>, steps: usize, orchestrator: Option<Orchestrator> }
impl Interpreter {
 pub fn new(capabilities: CapabilitySet) -> Self { Self { capabilities, max_steps: 100_000, functions: HashMap::new(), events: HashMap::new(), environment: None, steps: 0, orchestrator: None } }
 pub fn with_orchestration_policy(policy: ExecutionPolicy) -> Self {
  Self::with_optional_filesystem_authority(policy, None)
 }
 pub fn with_orchestration_policy_and_filesystem_authority(policy: ExecutionPolicy, authority: Arc<FilesystemAuthority>) -> Self {
  Self::with_optional_filesystem_authority(policy, Some(authority))
 }
 fn with_optional_filesystem_authority(policy: ExecutionPolicy, authority: Option<Arc<FilesystemAuthority>>) -> Self {
  let mut orchestrator = Orchestrator::new(policy);
  orchestrator.register(Box::new(DnsResolveAdapter::new()));
  if let Some(authority) = authority {
   orchestrator.register(Box::new(FileReadAdapter::with_authority(Arc::clone(&authority))));
   orchestrator.register(Box::new(FileWriteAdapter::with_authority(Arc::clone(&authority))));
    orchestrator.register(Box::new(FileDeleteAdapter::with_authority(authority)));
  } else {
   orchestrator.register(Box::new(FileReadAdapter::new()));
   orchestrator.register(Box::new(FileWriteAdapter::new()));
    orchestrator.register(Box::new(FileDeleteAdapter::new()));
  }
  orchestrator.register(Box::new(ProcessAdapter::new()));
  orchestrator.register(Box::new(SqliteAdapter::new()));
  orchestrator.register(Box::new(HttpGetAdapter::new()));
  Self { capabilities: CapabilitySet::none(), max_steps: 100_000, functions: HashMap::new(), events: HashMap::new(), environment: None, steps: 0, orchestrator: Some(orchestrator) }
 }
 pub fn execute_network(&mut self, operation: &IrNetworkOperation) -> Result<NetworkExecutionResult, RuntimeError> { let semantic = operation.operation(); if let Err(bundle) = semantic.validate() { let diagnostic = &bundle.0[0]; return Err(RuntimeError { code: diagnostic.code, message: diagnostic.message.clone() }); } for capability in semantic.required_capabilities() { if !self.capabilities.permits(capability) { return Err(RuntimeError { code: "N3101", message: format!("Kemampuan jaringan {:?} belum diberikan kepada program.", capability) }); } } Ok(NetworkExecutionResult { description: "network operation delegated to reference backend".into(), performed_external_io: false }) }
 pub fn execute(&mut self, program: &IrProgram) -> Result<ExecutionResult, RuntimeError> { self.functions.clear(); self.events.clear(); self.environment = None; self.steps = 0; self.register_definitions(&program.statements); let mut env = Environment::new(); let mut output = Vec::new(); self.exec_block(&program.statements, &mut env, &mut output)?; self.environment = Some(env); Ok(ExecutionResult { output }) }
 pub fn dispatch_event(&mut self, trigger: &str) -> Result<ExecutionResult, RuntimeError> { let body = self.events.get(trigger).cloned().ok_or_else(|| RuntimeError { code: "R3001", message: format!("Event '{}' belum terdaftar.", trigger) })?; let mut env = self.environment.take().ok_or_else(|| RuntimeError { code: "R3021", message: "Program belum dijalankan; jalankan program sebelum memicu event.".into() })?; let mut output = Vec::new(); self.steps = 0; let result = self.exec_block(&body, &mut env, &mut output); self.environment = Some(env); result.map(|()| ExecutionResult { output }) }
 fn register_definitions(&mut self, statements: &[IrStatement]) { for statement in statements { match statement { IrStatement::Function { name, params, result, body } => { self.functions.insert(name.clone(), (params.clone(), result.clone(), body.clone())); }, IrStatement::Event { trigger, body } => { self.events.insert(trigger.join(" "), body.clone()); }, _ => {} } } }
 fn tick(&mut self) -> Result<(), RuntimeError> { self.steps += 1; if self.steps > self.max_steps { Err(RuntimeError { code: "R3002", message: "Program melebihi batas langkah eksekusi yang aman.".into() }) } else { Ok(()) } }
 fn exec_block(&mut self, statements: &[IrStatement], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { for statement in statements { self.tick()?; self.exec_statement(statement, env, output)?; } Ok(()) }
 fn exec_scoped_block(&mut self, statements: &[IrStatement], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { env.push(); let result = self.exec_block(statements, env, output); env.pop(); result }
 fn exec_statement(&mut self, statement: &IrStatement, env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { match statement {
  IrStatement::Declaration { name, value } => { let value = self.eval(value, env)?; env.define(name.clone(), value); },
  IrStatement::Assignment { name, value } => { let value = self.eval(value, env)?; if !env.assign(name, value) { return Err(RuntimeError { code: "R3003", message: format!("Saya belum menemukan '{}'.", name) }); } },
  IrStatement::Action { name, args } => self.exec_action(name, args, env, output)?,
  IrStatement::Conditional { condition, then_block, else_block } => match self.eval(condition, env)? { Value::Boolean(true) => self.exec_scoped_block(then_block, env, output)?, Value::Boolean(false) => self.exec_scoped_block(else_block, env, output)?, _ => return Err(RuntimeError { code: "R3004", message: "Kondisi harus menghasilkan boolean.".into() }) },
  IrStatement::Loop { kind, body } => match kind { IrLoopKind::ForEach { variable, iterable } => { let list = self.eval(iterable, env)?; if let Value::List(values) = list { for value in values { env.push(); env.define(variable.clone(), value); let result = self.exec_block(body, env, output); env.pop(); result?; } } else { return Err(RuntimeError { code: "R3005", message: "Loop 'untuk setiap' membutuhkan daftar.".into() }); } }, IrLoopKind::While { condition } => loop { if !matches!(self.eval(condition, env)?, Value::Boolean(true)) { break; } env.push(); let result = self.exec_block(body, env, output); env.pop(); result?; self.tick()?; } },
  IrStatement::Function { .. } | IrStatement::Event { .. } => {},
  IrStatement::Network { operation } => { let _ = self.execute_network(operation)?; },
  IrStatement::Orchestration { semantic, capability, effect, target, adapter, expectation } => {
   let (scope, arguments) = match target { IrExpr::Text(value) => (crate::orchestration::ExecutionScope::target(value.clone()), Vec::new()), IrExpr::Call { name, args } if name == "filesystem.write" && args.len() == 2 => { let path = match self.eval(&args[0], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3200", message: format!("Target orchestration harus berupa teks, bukan {}.", other.display()) }) }; let content = match self.eval(&args[1], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3201", message: format!("Isi filesystem write harus berupa teks, bukan {}.", other.display()) }) }; (crate::orchestration::ExecutionScope::target(path), vec![AdapterArgument::Text(content)]) }, IrExpr::Call { name, args } if name == "database.query" && args.len() >= 2 => { let database = match self.eval(&args[0], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3200", message: format!("Database orchestration harus berupa teks, bukan {}.", other.display()) }) }; let sql = match self.eval(&args[1], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3201", message: format!("SQL orchestration harus berupa teks, bukan {}.", other.display()) }) }; let mut arguments = vec![AdapterArgument::Text(sql)]; for argument in &args[2..] { let value = match self.eval(argument, env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3202", message: format!("Parameter database harus berupa teks, bukan {}.", other.display()) }) }; arguments.push(AdapterArgument::Text(value)); } (crate::orchestration::ExecutionScope::resource(database), arguments) }, _ => { let value = self.eval(target, env)?; match value { Value::Text(value) => (crate::orchestration::ExecutionScope::target(value), Vec::new()), other => return Err(RuntimeError { code: "N3200", message: format!("Target orchestration harus berupa teks, bukan {}.", other.display()) }) } } };
   let orchestrator = self.orchestrator.as_ref().ok_or_else(|| RuntimeError { code: "N3205", message: "Orchestration belum diizinkan untuk interpreter ini.".into() })?;
   let request = CapabilityRequest::new(*semantic, *capability, scope, *effect); let mut audit = Vec::<AuditRecord>::new();
   let mut adapter_request = crate::adapter::AdapterRequest::new(request);
   for argument in arguments { adapter_request = adapter_request.with_argument(argument); }
   let (adapter_id, result) = orchestrator.execute_request_with_audit_for_adapter(&adapter_request, PlatformKind::current_host(), Some(&AdapterId::new(adapter)), &mut audit).map_err(Self::orchestration_error)?;
   let normalized = normalize_result(adapter_id, result, *expectation).map_err(|error| RuntimeError { code: "N3206", message: format!("Hasil orchestration tidak dapat dinormalisasi: {:?}", error) })?;
   match normalized.value() { NormalizedValue::Text(value) => output.extend(value.lines().map(ToOwned::to_owned)), NormalizedValue::Bytes(value) => match std::str::from_utf8(value) { Ok(text) => output.extend(text.lines().map(ToOwned::to_owned)), Err(_) => return Err(RuntimeError { code: "N3207", message: "Hasil orchestration berupa bytes yang bukan UTF-8.".into() }) } }
  }
 } Ok(()) }
 fn orchestration_error(error: OrchestrationError) -> RuntimeError { match error { OrchestrationError::InvalidRequest(error) => RuntimeError { code: "N3208", message: format!("Permintaan orchestration tidak valid: {:?}.", error) }, OrchestrationError::PolicyDenied(decision) => RuntimeError { code: "N3209", message: format!("Permintaan orchestration ditolak policy: {:?}.", decision) }, OrchestrationError::ApprovalRequired => RuntimeError { code: "N3210", message: "Permintaan orchestration membutuhkan persetujuan.".into() }, OrchestrationError::NoCompatibleAdapter => RuntimeError { code: "N3211", message: "Tidak ada adapter orchestration yang kompatibel.".into() }, OrchestrationError::Adapter(error) => RuntimeError { code: "N3212", message: format!("Adapter orchestration gagal: {:?}.", error) } } }
 fn exec_action(&mut self, name: &str, args: &[IrExpr], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { match name { "tampilkan" => { if args.len() != 1 { return Err(RuntimeError { code: "R3006", message: "'tampilkan' membutuhkan tepat satu nilai.".into() }); } output.push(self.eval(&args[0], env)?.display()); }, "sapa" => { if args.len() != 1 { return Err(RuntimeError { code: "R3007", message: "'sapa' membutuhkan satu nama.".into() }); } output.push(format!("Halo, {}!", self.eval(&args[0], env)?.display())); }, "buat daftar" => { if args.len() != 1 { return Err(RuntimeError { code: "R3008", message: "'buat daftar' membutuhkan nama daftar.".into() }); } if let IrExpr::Identifier(name) = &args[0] { env.define(name.clone(), Value::List(Vec::new())); } else { return Err(RuntimeError { code: "R3009", message: "Nama daftar harus berupa identifier.".into() }); } }, "tambahkan" | "hapus" => { if args.len() != 2 { return Err(RuntimeError { code: "R3010", message: format!("'{}' membutuhkan nilai dan nama daftar.", name) }); } let item = match &args[0] { IrExpr::Identifier(word) => env.get(word).unwrap_or_else(|| Value::Text(word.clone())), _ => self.eval(&args[0], env)? }; let list_name = match &args[1] { IrExpr::Identifier(value) => value, _ => return Err(RuntimeError { code: "R3011", message: "Nama daftar harus berupa identifier.".into() }) }; let mut list = env.get(list_name).ok_or_else(|| RuntimeError { code: "R3012", message: format!("Saya belum menemukan daftar '{}'.", list_name) })?; match &mut list { Value::List(values) if name == "tambahkan" => values.push(item), Value::List(values) => { if let Some(index) = values.iter().position(|v| v == &item) { values.remove(index); } }, _ => return Err(RuntimeError { code: "R3013", message: format!("'{}' bukan daftar.", list_name) }) } let _ = env.assign(list_name, list); }, "gunakan proses" => { if !self.capabilities.permits(Capability::Process) { return Err(RuntimeError { code: "R3014", message: "Kemampuan proses belum diberikan kepada program.".into() }); } }, _ => return Err(RuntimeError { code: "R3015", message: format!("Aksi '{}' belum memiliki implementasi standard library.", name) }) } Ok(()) }
 fn eval(&mut self, expr: &IrExpr, env: &Environment) -> Result<Value, RuntimeError> { match expr { IrExpr::Text(v) => Ok(Value::Text(v.clone())), IrExpr::Number(v) => Ok(Value::Number(*v)), IrExpr::Boolean(v) => Ok(Value::Boolean(*v)), IrExpr::Nothing => Ok(Value::Nothing), IrExpr::Identifier(name) => env.get(name).ok_or_else(|| RuntimeError { code: "R3016", message: format!("Saya belum menemukan '{}'.", name) }), IrExpr::Binary { left, op, right } => { let left_value = self.eval(left, env)?; let right_value = self.eval(right, env)?; self.eval_binary(*op, left_value, right_value) }, IrExpr::Call { name, args } => self.call_function(name, args, env) } }
 fn eval_binary(&self, op: BinaryOp, left: Value, right: Value) -> Result<Value, RuntimeError> { match (op, left, right) { (BinaryOp::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)), (BinaryOp::Subtract, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)), (BinaryOp::Multiply, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)), (BinaryOp::Divide, Value::Number(a), Value::Number(b)) if b != 0 => Ok(Value::Number(a / b)), (BinaryOp::Divide, _, Value::Number(0)) => Err(RuntimeError { code: "R3017", message: "Pembagian dengan nol tidak diperbolehkan.".into() }), (BinaryOp::Equal, a, b) => Ok(Value::Boolean(a == b)), (BinaryOp::Greater, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)), (BinaryOp::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)), (BinaryOp::AtMost, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)), (BinaryOp::AtLeast, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)), (BinaryOp::Add, Value::Text(a), Value::Text(b)) => Ok(Value::Text(format!("{}{}", a, b))), _ => Err(RuntimeError { code: "R3018", message: "Operasi tidak cocok dengan tipe nilai.".into() }) } }
 fn call_function(&mut self, name: &str, args: &[IrExpr], caller_env: &Environment) -> Result<Value, RuntimeError> { let (params, result, body) = self.functions.get(name).cloned().ok_or_else(|| RuntimeError { code: "R3019", message: format!("Pekerjaan '{}' belum dibuat.", name) })?; if params.len() != args.len() { return Err(RuntimeError { code: "R3020", message: format!("Pekerjaan '{}' membutuhkan {} parameter.", name, params.len()) }); } let mut env = caller_env.clone(); env.push(); for (param, arg) in params.iter().zip(args) { let value = self.eval(arg, caller_env)?; env.define(param.clone(), value); } let mut ignored_output = Vec::new(); self.exec_block(&body, &mut env, &mut ignored_output)?; self.eval(&result, &env) }
}


#[cfg(test)]
mod scope_cleanup_tests {
    use super::*;

    #[test]
    fn failed_event_does_not_leak_nested_scope_variables() {
        let mut interpreter = Interpreter::new(CapabilitySet::none());
        let program = IrProgram {
            statements: vec![
                IrStatement::Declaration {
                    name: "root".into(),
                    value: IrExpr::Text("retained".into()),
                },
                IrStatement::Event {
                    trigger: vec!["uji".into()],
                    body: vec![IrStatement::Conditional {
                        condition: IrExpr::Boolean(true),
                        then_block: vec![
                            IrStatement::Declaration {
                                name: "temporary".into(),
                                value: IrExpr::Text("must not leak".into()),
                            },
                            IrStatement::Action {
                                name: "unknown action".into(),
                                args: Vec::new(),
                            },
                        ],
                        else_block: Vec::new(),
                    }],
                },
            ],
        };

        interpreter.execute(&program).expect("program setup should succeed");
        let error = interpreter
            .dispatch_event("uji")
            .expect_err("unknown action should fail the event");
        assert_eq!(error.code, "R3015");

        let environment = interpreter.environment.as_ref().expect("event environment should be retained");
        assert_eq!(environment.get("root"), Some(Value::Text("retained".into())));
        assert_eq!(environment.get("temporary"), None);
    }
}

#[cfg(test)]
mod filesystem_delete_language_tests {
    use super::*;
    use crate::governance::ScopeRule;

    fn delete_program() -> IrProgram {
        let program = crate::parse("hapus berkas \"target.txt\"\n").expect("delete source should parse");
        let typed = crate::analyze(&program).expect("delete source should analyze");
        crate::lower(&typed).expect("delete source should lower")
    }

    #[cfg(any(target_os = "linux", windows, target_os = "android", target_os = "macos"))]
    #[test]
    fn file_delete_is_denied_without_an_explicit_delete_grant_and_has_no_side_effect() {
        let root = std::env::temp_dir().join(format!("one-delete-denied-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let target = root.join("target.txt");
        std::fs::write(&target, b"KEEP").unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 1024, 1024).unwrap());
        let mut interpreter = Interpreter::with_orchestration_policy_and_filesystem_authority(ExecutionPolicy::default(), authority);
        let error = interpreter.execute(&delete_program()).expect_err("delete must be denied by default");
        assert_eq!(error.code, "N3209");
        assert_eq!(std::fs::read(&target).unwrap(), b"KEEP");
        drop(interpreter);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn explicit_delete_grant_reaches_the_windows_native_adapter() {
        let root = std::env::temp_dir().join(format!("one-delete-allowed-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let target = root.join("target.txt");
        std::fs::write(&target, b"DELETE_ME").unwrap();
        let authority = Arc::new(FilesystemAuthority::open(&root, 1024, 1024).unwrap());
        let policy = ExecutionPolicy::default()
            .grant(Capability::FilesystemDelete)
            .allow_scope(Capability::FilesystemDelete, ScopeRule::Target("target.txt".into()));
        let mut interpreter = Interpreter::with_orchestration_policy_and_filesystem_authority(policy, authority);
        interpreter.execute(&delete_program()).expect("explicitly granted delete should execute on Windows");
        assert!(!target.exists());
        drop(interpreter);
        std::fs::remove_dir_all(root).unwrap();
    }
}
