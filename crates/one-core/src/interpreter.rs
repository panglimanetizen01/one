#![allow(clippy::possible_missing_else)]
#![allow(unused_imports)]
use std::collections::{BTreeMap, HashMap};
use crate::ast::BinaryOp;
use crate::capability::{Capability, CapabilitySet};
use crate::adapter::{AdapterArgument, FileReadAdapter, FileWriteAdapter, HttpGetAdapter, PlatformKind, ProcessAdapter};
use crate::database::SqliteAdapter;
use crate::governance::ExecutionPolicy;
use crate::ir::{IrExpr, IrLoopKind, IrNetworkOperation, IrProgram, IrStatement};
use crate::network_dns_adapter::DnsResolveAdapter;
use crate::orchestration::{AdapterId, CapabilityRequest};
use crate::orchestration_audit::AuditRecord;
use crate::orchestration_result::{normalize_result, NormalizationExpectation, NormalizedValue};
use crate::orchestrator::{OrchestrationError, Orchestrator};
use crate::runtime::{Environment, Value};
use crate::stdlib;
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ExecutionResult { pub output: Vec<String> }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct NetworkExecutionResult { pub description: String, pub performed_external_io: bool }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct RuntimeError { pub code: &'static str, pub message: String }
const MAX_RUNTIME_OUTPUT_BYTES: usize = 16 * 1024 * 1024;
pub struct Interpreter { pub capabilities: CapabilitySet, pub max_steps: usize, pub max_call_depth: usize, output_bytes: usize, functions: HashMap<String, (Vec<String>, IrExpr, Vec<IrStatement>)>, events: HashMap<String, Vec<IrStatement>>, environment: Option<Environment>, steps: usize, call_depth: usize, orchestrator: Option<Orchestrator> }
impl Interpreter {
 pub fn new(capabilities: CapabilitySet) -> Self { Self { capabilities, max_steps: 100_000, max_call_depth: 128, output_bytes: 0, functions: HashMap::new(), events: HashMap::new(), environment: None, steps: 0, call_depth: 0, orchestrator: None } }
 pub fn with_orchestration_policy(policy: ExecutionPolicy) -> Self { let mut orchestrator = Orchestrator::new(policy); orchestrator.register(Box::new(DnsResolveAdapter::new())); orchestrator.register(Box::new(FileReadAdapter::new())); orchestrator.register(Box::new(FileWriteAdapter::new())); orchestrator.register(Box::new(ProcessAdapter::new())); orchestrator.register(Box::new(SqliteAdapter::new())); orchestrator.register(Box::new(HttpGetAdapter::new())); Self { capabilities: CapabilitySet::none(), max_steps: 100_000, max_call_depth: 128, output_bytes: 0, functions: HashMap::new(), events: HashMap::new(), environment: None, steps: 0, call_depth: 0, orchestrator: Some(orchestrator) } }
 pub fn execute_network(&mut self, operation: &IrNetworkOperation) -> Result<NetworkExecutionResult, RuntimeError> { let semantic = operation.operation(); if let Err(bundle) = semantic.validate() { let diagnostic = &bundle.0[0]; return Err(RuntimeError { code: diagnostic.code, message: diagnostic.message.clone() }); } for capability in semantic.required_capabilities() { if !self.capabilities.permits(capability) { return Err(RuntimeError { code: "N3101", message: format!("Kemampuan jaringan {:?} belum diberikan kepada program.", capability) }); } } Ok(NetworkExecutionResult { description: "network operation delegated to reference backend".into(), performed_external_io: false }) }
 pub fn execute(&mut self, program: &IrProgram) -> Result<ExecutionResult, RuntimeError> { self.functions.clear(); self.events.clear(); self.environment = None; self.steps = 0; self.output_bytes = 0; self.register_definitions(&program.statements); let mut env = Environment::new(); let mut output = Vec::new(); self.exec_block(&program.statements, &mut env, &mut output)?; self.environment = Some(env); Ok(ExecutionResult { output }) }
 pub fn dispatch_event(&mut self, trigger: &str) -> Result<ExecutionResult, RuntimeError> { let body = self.events.get(trigger).cloned().ok_or_else(|| RuntimeError { code: "R3001", message: format!("Event '{}' belum terdaftar.", trigger) })?; let mut env = self.environment.take().ok_or_else(|| RuntimeError { code: "R3021", message: "Program belum dijalankan; jalankan program sebelum memicu event.".into() })?; let mut output = Vec::new(); self.steps = 0; self.output_bytes = 0; let result = self.exec_block(&body, &mut env, &mut output); self.environment = Some(env); result.map(|()| ExecutionResult { output }) }
 fn register_definitions(&mut self, statements: &[IrStatement]) { for statement in statements { match statement { IrStatement::Function { name, params, result, body } => { self.functions.insert(name.clone(), (params.clone(), result.clone(), body.clone())); }, IrStatement::Event { trigger, body } => { self.events.insert(trigger.join(" "), body.clone()); }, _ => {} } } }
 fn tick(&mut self) -> Result<(), RuntimeError> { self.steps += 1; if self.steps > self.max_steps { Err(RuntimeError { code: "R3002", message: "Program melebihi batas langkah eksekusi yang aman.".into() }) } else { Ok(()) } }
 fn exec_block(&mut self, statements: &[IrStatement], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { for statement in statements { self.tick()?; self.exec_statement(statement, env, output)?; } Ok(()) }
 fn exec_statement(&mut self, statement: &IrStatement, env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { match statement {
  IrStatement::Declaration { name, value } => { let value = self.eval(value, env)?; env.define(name.clone(), value); },
  IrStatement::Assignment { name, value } => { let value = self.eval(value, env)?; if !env.assign(name, value) { return Err(RuntimeError { code: "R3003", message: format!("Saya belum menemukan '{}'.", name) }); } },
  IrStatement::Action { name, args } => self.exec_action(name, args, env, output)?,
  IrStatement::Conditional { condition, then_block, else_block } => match self.eval(condition, env)? { Value::Boolean(true) => { env.push(); self.exec_block(then_block, env, output)?; env.pop(); }, Value::Boolean(false) => { env.push(); self.exec_block(else_block, env, output)?; env.pop(); }, _ => return Err(RuntimeError { code: "R3004", message: "Kondisi harus menghasilkan boolean.".into() }) },
  IrStatement::Loop { kind, body } => match kind { IrLoopKind::ForEach { variable, iterable } => { let list = self.eval(iterable, env)?; if let Value::List(values) = list { for value in values { env.push(); env.define(variable.clone(), value); self.exec_block(body, env, output)?; env.pop(); } } else { return Err(RuntimeError { code: "R3005", message: "Loop 'untuk setiap' membutuhkan daftar.".into() }); } }, IrLoopKind::While { condition } => loop { if !matches!(self.eval(condition, env)?, Value::Boolean(true)) { break; } env.push(); self.exec_block(body, env, output)?; env.pop(); self.tick()?; } },
  IrStatement::Function { .. } | IrStatement::Event { .. } => {},
  IrStatement::Network { operation } => { let _ = self.execute_network(operation)?; },
  IrStatement::Orchestration { semantic, capability, effect, target, adapter, expectation } => {
   let (scope, arguments) = match target { IrExpr::Text(value) => (crate::orchestration::ExecutionScope::target(value.clone()), Vec::new()), IrExpr::Call { name, args } if name == "filesystem.write" && args.len() == 2 => { let path = match self.eval(&args[0], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3200", message: format!("Target orchestration harus berupa teks, bukan {}.", other.display()) }) }; let content = match self.eval(&args[1], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3201", message: format!("Isi filesystem write harus berupa teks, bukan {}.", other.display()) }) }; (crate::orchestration::ExecutionScope::target(path), vec![AdapterArgument::Text(content)]) }, IrExpr::Call { name, args } if name == "database.query" && args.len() >= 2 => { let database = match self.eval(&args[0], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3200", message: format!("Database orchestration harus berupa teks, bukan {}.", other.display()) }) }; let sql = match self.eval(&args[1], env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3201", message: format!("SQL orchestration harus berupa teks, bukan {}.", other.display()) }) }; let mut arguments = vec![AdapterArgument::Text(sql)]; for argument in &args[2..] { let value = match self.eval(argument, env)? { Value::Text(value) => value, other => return Err(RuntimeError { code: "N3202", message: format!("Parameter database harus berupa teks, bukan {}.", other.display()) }) }; arguments.push(AdapterArgument::Text(value)); } (crate::orchestration::ExecutionScope::resource(database), arguments) }, _ => { let value = self.eval(target, env)?; match value { Value::Text(value) => (crate::orchestration::ExecutionScope::target(value), Vec::new()), other => return Err(RuntimeError { code: "N3200", message: format!("Target orchestration harus berupa teks, bukan {}.", other.display()) }) } } };
   let orchestrator = self.orchestrator.as_ref().ok_or_else(|| RuntimeError { code: "N3205", message: "Orchestration belum diizinkan untuk interpreter ini.".into() })?;
   let request = CapabilityRequest::new(*semantic, *capability, scope, *effect); let mut audit = Vec::<AuditRecord>::new();
   let mut adapter_request = crate::adapter::AdapterRequest::new(request);
   for argument in arguments { adapter_request = adapter_request.with_argument(argument); }
   let (adapter_id, result) = orchestrator.execute_request_with_audit_for_adapter(&adapter_request, PlatformKind::Any, Some(&AdapterId::new(adapter)), &mut audit).map_err(Self::orchestration_error)?;
   let normalized = normalize_result(adapter_id, result, *expectation).map_err(|error| RuntimeError { code: "N3206", message: format!("Hasil orchestration tidak dapat dinormalisasi: {:?}", error) })?;
   match normalized.value() { NormalizedValue::Text(value) => { for line in value.lines() { self.push_output(output, line.to_owned())?; } }, NormalizedValue::Bytes(value) => match std::str::from_utf8(value) { Ok(text) => { for line in text.lines() { self.push_output(output, line.to_owned())?; } }, Err(_) => return Err(RuntimeError { code: "N3207", message: "Hasil orchestration berupa bytes yang bukan UTF-8.".into() }) } }
  }
 } Ok(()) }
 fn orchestration_error(error: OrchestrationError) -> RuntimeError { match error { OrchestrationError::InvalidRequest(error) => RuntimeError { code: "N3208", message: format!("Permintaan orchestration tidak valid: {:?}.", error) }, OrchestrationError::PolicyDenied(decision) => RuntimeError { code: "N3209", message: format!("Permintaan orchestration ditolak policy: {:?}.", decision) }, OrchestrationError::ApprovalRequired => RuntimeError { code: "N3210", message: "Permintaan orchestration membutuhkan persetujuan.".into() }, OrchestrationError::NoCompatibleAdapter => RuntimeError { code: "N3211", message: "Tidak ada adapter orchestration yang kompatibel.".into() }, OrchestrationError::Adapter(error) => RuntimeError { code: "N3212", message: format!("Adapter orchestration gagal: {:?}.", error) } } }
 fn exec_action(&mut self, name: &str, args: &[IrExpr], env: &mut Environment, output: &mut Vec<String>) -> Result<(), RuntimeError> { match name { "tampilkan" => { if args.len() != 1 { return Err(RuntimeError { code: "R3006", message: "'tampilkan' membutuhkan tepat satu nilai.".into() }); } let value = self.eval(&args[0], env)?.display(); self.push_output(output, value)?; }, "sapa" => { if args.len() != 1 { return Err(RuntimeError { code: "R3007", message: "'sapa' membutuhkan satu nama.".into() }); } let value = format!("Halo, {}!", self.eval(&args[0], env)?.display()); self.push_output(output, value)?; }, "buat daftar" => { if args.len() != 1 { return Err(RuntimeError { code: "R3008", message: "'buat daftar' membutuhkan nama daftar.".into() }); } if let IrExpr::Identifier(name) = &args[0] { env.define(name.clone(), Value::List(Vec::new())); } else { return Err(RuntimeError { code: "R3009", message: "Nama daftar harus berupa identifier.".into() }); } }, "buat pemetaan" => {
    if args.len() != 1 { return Err(RuntimeError { code: "R3030", message: "'buat pemetaan' membutuhkan nama pemetaan.".into() }); }
    if let IrExpr::Identifier(map_name) = &args[0] {
        env.define(map_name.clone(), Value::Map(BTreeMap::new()));
    } else {
        return Err(RuntimeError { code: "R3031", message: "Nama pemetaan harus berupa identifier.".into() });
    }
}, "masukkan ke pemetaan" => {
    if args.len() != 3 { return Err(RuntimeError { code: "R3032", message: "'masukkan' membutuhkan kunci, nilai, dan nama pemetaan.".into() }); }
    let key = match self.eval(&args[0], env)? {
        Value::Text(key) => key,
        _ => return Err(RuntimeError { code: "R3033", message: "Kunci pemetaan harus berupa teks.".into() }),
    };
    let value = self.eval(&args[1], env)?;
    let map_name = match &args[2] {
        IrExpr::Identifier(name) => name,
        _ => return Err(RuntimeError { code: "R3034", message: "Nama pemetaan harus berupa identifier.".into() }),
    };
    let mut map = env.get(map_name).ok_or_else(|| RuntimeError { code: "R3035", message: format!("Pemetaan '{}' belum dibuat.", map_name) })?;
    match &mut map {
        Value::Map(values) => { values.insert(key, value); }
        _ => return Err(RuntimeError { code: "R3036", message: format!("'{}' bukan pemetaan.", map_name) }),
    }
    let _ = env.assign(map_name, map);
}, "ambil kunci dari pemetaan" => {
    if args.len() != 2 { return Err(RuntimeError { code: "R3037", message: "'ambil kunci' membutuhkan kunci dan nama pemetaan.".into() }); }
    let key = match self.eval(&args[0], env)? {
        Value::Text(key) => key,
        _ => return Err(RuntimeError { code: "R3038", message: "Kunci pemetaan harus berupa teks.".into() }),
    };
    let map_name = match &args[1] {
        IrExpr::Identifier(name) => name,
        _ => return Err(RuntimeError { code: "R3039", message: "Nama pemetaan harus berupa identifier.".into() }),
    };
    let map = env.get(map_name).ok_or_else(|| RuntimeError { code: "R3040", message: format!("Pemetaan '{}' belum dibuat.", map_name) })?;
    match map {
        Value::Map(values) => { let value = values.get(&key).ok_or_else(|| RuntimeError { code: "R3041", message: format!("Kunci '{}' tidak ditemukan dalam pemetaan '{}'.", key, map_name) })?.display(); self.push_output(output, value)?; },
        _ => return Err(RuntimeError { code: "R3042", message: format!("'{}' bukan pemetaan.", map_name) }),
    }
}, "hapus kunci dari pemetaan" => {
    if args.len() != 2 { return Err(RuntimeError { code: "R3043", message: "'hapus kunci' membutuhkan kunci dan nama pemetaan.".into() }); }
    let key = match self.eval(&args[0], env)? {
        Value::Text(key) => key,
        _ => return Err(RuntimeError { code: "R3044", message: "Kunci pemetaan harus berupa teks.".into() }),
    };
    let map_name = match &args[1] {
        IrExpr::Identifier(name) => name,
        _ => return Err(RuntimeError { code: "R3045", message: "Nama pemetaan harus berupa identifier.".into() }),
    };
    let mut map = env.get(map_name).ok_or_else(|| RuntimeError { code: "R3046", message: format!("Pemetaan '{}' belum dibuat.", map_name) })?;
    match &mut map {
        Value::Map(values) => {
            if values.remove(&key).is_none() { return Err(RuntimeError { code: "R3047", message: format!("Kunci '{}' tidak ditemukan dalam pemetaan '{}'.", key, map_name) }); }
        }
        _ => return Err(RuntimeError { code: "R3048", message: format!("'{}' bukan pemetaan.", map_name) }),
    }
    let _ = env.assign(map_name, map);
}, "tambahkan" | "hapus" => { if args.len() != 2 { return Err(RuntimeError { code: "R3010", message: format!("'{}' membutuhkan nilai dan nama daftar.", name) }); } let item = match &args[0] { IrExpr::Identifier(word) => env.get(word).unwrap_or_else(|| Value::Text(word.clone())), _ => self.eval(&args[0], env)? }; let list_name = match &args[1] { IrExpr::Identifier(value) => value, _ => return Err(RuntimeError { code: "R3011", message: "Nama daftar harus berupa identifier.".into() }) }; let mut list = env.get(list_name).ok_or_else(|| RuntimeError { code: "R3012", message: format!("Saya belum menemukan daftar '{}'.", list_name) })?; match &mut list { Value::List(values) if name == "tambahkan" => values.push(item), Value::List(values) => { if let Some(index) = values.iter().position(|v| v == &item) { values.remove(index); } }, _ => return Err(RuntimeError { code: "R3013", message: format!("'{}' bukan daftar.", list_name) }) } let _ = env.assign(list_name, list); }, "gunakan proses" => { if !self.capabilities.permits(Capability::Process) { return Err(RuntimeError { code: "R3014", message: "Kemampuan proses belum diberikan kepada program.".into() }); } }, _ => return Err(RuntimeError { code: "R3015", message: format!("Aksi '{}' belum memiliki implementasi standard library.", name) }) } Ok(()) }
 fn push_output(&mut self, output: &mut Vec<String>, value: String) -> Result<(), RuntimeError> {
    let next = self.output_bytes.checked_add(value.len()).ok_or_else(|| RuntimeError {
        code: "R3050",
        message: "Program melewati batas output runtime sebesar 16 MiB.".into(),
    })?;
    if next > MAX_RUNTIME_OUTPUT_BYTES {
        return Err(RuntimeError {
            code: "R3050",
            message: "Program melewati batas output runtime sebesar 16 MiB.".into(),
        });
    }
    self.output_bytes = next;
    output.push(value);
    Ok(())
 }
 fn eval(&mut self, expr: &IrExpr, env: &Environment) -> Result<Value, RuntimeError> { match expr { IrExpr::Text(v) => Ok(Value::Text(v.clone())), IrExpr::Number(v) => Ok(Value::Number(*v)), IrExpr::Boolean(v) => Ok(Value::Boolean(*v)), IrExpr::Nothing => Ok(Value::Nothing), IrExpr::Identifier(name) => env.get(name).ok_or_else(|| RuntimeError { code: "R3016", message: format!("Saya belum menemukan '{}'.", name) }), IrExpr::Binary { left, op, right } => { let left_value = self.eval(left, env)?; let right_value = self.eval(right, env)?; self.eval_binary(*op, left_value, right_value) }, IrExpr::Call { name, args } => self.call_function(name, args, env) } }
 fn eval_binary(&self, op: BinaryOp, left: Value, right: Value) -> Result<Value, RuntimeError> { match (op, left, right) { (BinaryOp::Add, Value::Number(a), Value::Number(b)) => a.checked_add(b).map(Value::Number).ok_or_else(|| RuntimeError { code: "R3023", message: "Hasil operasi angka berada di luar rentang integer.".into() }), (BinaryOp::Subtract, Value::Number(a), Value::Number(b)) => a.checked_sub(b).map(Value::Number).ok_or_else(|| RuntimeError { code: "R3023", message: "Hasil operasi angka berada di luar rentang integer.".into() }), (BinaryOp::Multiply, Value::Number(a), Value::Number(b)) => a.checked_mul(b).map(Value::Number).ok_or_else(|| RuntimeError { code: "R3023", message: "Hasil operasi angka berada di luar rentang integer.".into() }), (BinaryOp::Divide, Value::Number(a), Value::Number(b)) if b != 0 => a.checked_div(b).map(Value::Number).ok_or_else(|| RuntimeError { code: "R3023", message: "Hasil operasi angka berada di luar rentang integer.".into() }), (BinaryOp::Divide, _, Value::Number(0)) => Err(RuntimeError { code: "R3017", message: "Pembagian dengan nol tidak diperbolehkan.".into() }), (BinaryOp::Equal, a, b) => Ok(Value::Boolean(a == b)), (BinaryOp::Greater, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)), (BinaryOp::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)), (BinaryOp::AtMost, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)), (BinaryOp::AtLeast, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)), (BinaryOp::Add, Value::Text(a), Value::Text(b)) => Ok(Value::Text(format!("{}{}", a, b))), _ => Err(RuntimeError { code: "R3018", message: "Operasi tidak cocok dengan tipe nilai.".into() }) } }
 fn call_function(&mut self, name: &str, args: &[IrExpr], caller_env: &Environment) -> Result<Value, RuntimeError> {
    let builtin = matches!(name,
        "panjang teks" | "teks memuat" | "pangkas teks" | "ganti teks" |
        "pisah teks" | "ubah teks ke angka" | "panjang daftar" | "tambah elemen daftar" | "hapus elemen daftar" | "ambil indeks" |
        "nilai mutlak" | "bagi bulat" | "angka sisa bagi" | "angka minimum" | "angka maksimum" |
        "nilai pemetaan" | "atur pemetaan" | "ubah nilai ke json" |
        "teks diawali" | "teks diakhiri" | "huruf kecil" | "huruf besar" | "gabung teks" | "jumlah daftar angka" | "nilai dari json" | "waktu utc dari unix" | "unix dari waktu utc"
    );
    if builtin {
        let expected_arity = match name {
            "panjang teks" | "pangkas teks" | "ubah teks ke angka" | "panjang daftar" | "nilai mutlak" | "ubah nilai ke json" | "huruf kecil" | "huruf besar" | "jumlah daftar angka" | "nilai dari json" | "waktu utc dari unix" | "unix dari waktu utc" => 1,
            "teks memuat" | "pisah teks" | "tambah elemen daftar" | "hapus elemen daftar" | "ambil indeks" | "bagi bulat" | "angka sisa bagi" | "angka minimum" | "angka maksimum" | "nilai pemetaan" | "teks diawali" | "teks diakhiri" | "gabung teks" => 2,
            "atur pemetaan" => 3,
            "ganti teks" => 3,
            _ => 0,
        };
        if args.len() != expected_arity {
            return Err(RuntimeError {
                code: "R3020",
                message: format!("Fungsi bawaan '{}' membutuhkan {} argumen.", name, expected_arity),
            });
        }
        let values = args.iter().map(|arg| self.eval(arg, caller_env)).collect::<Result<Vec<_>, _>>()?;
        let result = match name {
            "panjang teks" => stdlib::text_length(&values[0]),
            "teks memuat" => stdlib::text_contains(&values[0], &values[1]),
            "pangkas teks" => stdlib::text_trim(&values[0]),
            "ganti teks" => stdlib::text_replace(&values[0], &values[1], &values[2]),
            "pisah teks" => stdlib::text_split(&values[0], &values[1]),
            "ubah teks ke angka" => stdlib::text_to_integer(&values[0]),
            "panjang daftar" => stdlib::list_length(&values[0]),
            "tambah elemen daftar" => stdlib::list_append(values[0].clone(), values[1].clone()),
            "hapus elemen daftar" => stdlib::list_remove(values[0].clone(), &values[1]),
            "ambil indeks" => stdlib::list_get(&values[0], &values[1]),
            "nilai mutlak" => stdlib::number_abs(&values[0]),
            "bagi bulat" => stdlib::number_divide(&values[0], &values[1]),
            "angka sisa bagi" => stdlib::number_modulo(&values[0], &values[1]),
            "angka minimum" => stdlib::number_min(&values[0], &values[1]),
            "angka maksimum" => stdlib::number_max(&values[0], &values[1]),
            "nilai pemetaan" => stdlib::map_get(&values[0], &values[1]),
            "atur pemetaan" => stdlib::map_set(values[0].clone(), &values[1], values[2].clone()),
            "hapus kunci pemetaan" => stdlib::map_remove(values[0].clone(), &values[1]),
            "ubah nilai ke json" => stdlib::value_to_json(&values[0]),
            "nilai dari json" => stdlib::json_to_value(&values[0]),
            "waktu utc dari unix" => stdlib::unix_seconds_to_utc_text(&values[0]),
            "unix dari waktu utc" => stdlib::utc_text_to_unix_seconds(&values[0]),
            "teks diawali" => stdlib::text_starts_with(&values[0], &values[1]),
            "teks diakhiri" => stdlib::text_ends_with(&values[0], &values[1]),
            "huruf kecil" => stdlib::text_lowercase(&values[0]),
            "huruf besar" => stdlib::text_uppercase(&values[0]),
            "gabung teks" => stdlib::list_join_text(&values[0], &values[1]),
            "jumlah daftar angka" => stdlib::list_sum_numbers(&values[0]),
            _ => None,
        };
        return result.ok_or_else(|| RuntimeError {
            code: "R3022",
            message: format!("Argumen tidak valid atau operasi '{}' gagal.", name),
        });
    }
    if self.call_depth >= self.max_call_depth {
        return Err(RuntimeError {
            code: "R3024",
            message: format!("Batas kedalaman pemanggilan fungsi ({}) terlampaui.", self.max_call_depth),
        });
    }
    self.call_depth += 1;
    let result = self.call_user_function(name, args, caller_env);
    self.call_depth -= 1;
    result
}

fn call_user_function(&mut self, name: &str, args: &[IrExpr], caller_env: &Environment) -> Result<Value, RuntimeError> {
    let (params, result, body) = self.functions.get(name).cloned().ok_or_else(|| RuntimeError {
        code: "R3019",
        message: format!("Pekerjaan '{}' belum dibuat.", name),
    })?;
    if params.len() != args.len() {
        return Err(RuntimeError {
            code: "R3020",
            message: format!("Pekerjaan '{}' membutuhkan {} parameter.", name, params.len()),
        });
    }
    let mut env = caller_env.clone();
    env.push();
    for (param, arg) in params.iter().zip(args) {
        let value = self.eval(arg, caller_env)?;
        env.define(param.clone(), value);
    }
    let mut ignored_output = Vec::new();
    self.exec_block(&body, &mut env, &mut ignored_output)?;
    self.eval(&result, &env)
}
}


#[cfg(test)]
mod resource_bound_tests {
    use super::*;

    #[test]
    fn runtime_output_budget_rejects_aggregate_output_over_limit() {
        let mut interpreter = Interpreter::new(CapabilitySet::none());
        let mut output = Vec::new();
        interpreter.push_output(&mut output, "x".repeat(MAX_RUNTIME_OUTPUT_BYTES - 1))
            .expect("output just below the budget should be accepted");
        let error = interpreter.push_output(&mut output, "xx".into())
            .expect_err("aggregate output over the budget must be rejected");
        assert_eq!(error.code, "R3050");
        assert_eq!(output.len(), 1);
        assert_eq!(interpreter.output_bytes, MAX_RUNTIME_OUTPUT_BYTES - 1);
    }

    #[test]
    fn runtime_output_budget_accepts_exact_limit_and_resets_for_next_execution() {
        let mut interpreter = Interpreter::new(CapabilitySet::none());
        let mut output = Vec::new();
        interpreter.push_output(&mut output, "x".repeat(MAX_RUNTIME_OUTPUT_BYTES))
            .expect("exact output budget should be accepted");
        assert_eq!(interpreter.output_bytes, MAX_RUNTIME_OUTPUT_BYTES);
        interpreter.output_bytes = 0;
        let mut next_output = Vec::new();
        interpreter.push_output(&mut next_output, "next".into())
            .expect("new execution should start with a fresh output budget");
    }
}
