#![allow(clippy::possible_missing_else)]
#![allow(clippy::collapsible_if)]
use std::collections::HashMap;
use crate::ast::{BinaryOp, Expr, LoopKind, Program, Statement};
use crate::diagnostic::{Diagnostic, DiagnosticBundle};
use crate::network::NetworkOperation;
use crate::types::Type;
#[derive(Debug, Clone, PartialEq)] pub struct TypedProgram { pub program: Program }
pub fn analyze(program: &Program) -> Result<TypedProgram, DiagnosticBundle> { let mut analyzer = Analyzer {
        scopes: vec![HashMap::new()],
        functions: HashMap::from([
            ("panjang teks".to_owned(), 1),
            ("teks memuat".to_owned(), 2),
            ("teks diawali".to_owned(), 2),
            ("teks diakhiri".to_owned(), 2),
            ("pangkas teks".to_owned(), 1),
            ("ganti teks".to_owned(), 3),
            ("pisah teks".to_owned(), 2),
            ("ubah teks ke angka".to_owned(), 1),
            ("panjang daftar".to_owned(), 1),
            ("tambah elemen daftar".to_owned(), 2),
            ("hapus elemen daftar".to_owned(), 2),
            ("ambil indeks".to_owned(), 2),
            ("nilai mutlak".to_owned(), 1),
            ("bagi bulat".to_owned(), 2),
            ("angka sisa bagi".to_owned(), 2),
            ("angka minimum".to_owned(), 2),
            ("angka maksimum".to_owned(), 2),
            ("nilai pemetaan".to_owned(), 2),
            ("atur pemetaan".to_owned(), 3),
            ("hapus kunci pemetaan".to_owned(), 2),
            ("ubah nilai ke json".to_owned(), 1),
            ("huruf kecil".to_owned(), 1),
            ("huruf besar".to_owned(), 1),
            ("gabung teks".to_owned(), 2),
            ("jumlah daftar angka".to_owned(), 1),
            ("nilai dari json".to_owned(), 1),
            ("waktu utc dari unix".to_owned(), 1),
            ("unix dari waktu utc".to_owned(), 1),
        ]),
    }; analyzer.register_functions(&program.statements)?; analyzer.analyze_block(&program.statements)?; Ok(TypedProgram { program: program.clone() }) }
pub fn analyze_network(operation: &NetworkOperation) -> Result<(), DiagnosticBundle> { operation.validate() }
struct Analyzer { scopes: Vec<HashMap<String, Type>>, functions: HashMap<String, usize> }
impl Analyzer {
    fn register_functions(&mut self, statements: &[Statement]) -> Result<(), DiagnosticBundle> { for statement in statements { if let Statement::Function { name, params, span, .. } = statement { if self.functions.insert(name.clone(), params.len()).is_some() { return Err(DiagnosticBundle::single(Diagnostic::error("E2013", format!("Pekerjaan '{}' sudah dibuat.", name), Some(span.clone())))); } } } Ok(()) }
    fn current_scope(&mut self) -> &mut HashMap<String, Type> { self.scopes.last_mut().unwrap() }
    fn lookup(&self, name: &str) -> Option<Type> { self.scopes.iter().rev().find_map(|scope| scope.get(name).copied()) }
    fn analyze_block(&mut self, statements: &[Statement]) -> Result<(), DiagnosticBundle> { for statement in statements { self.analyze_statement(statement)?; } Ok(()) }
    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), DiagnosticBundle> {
        match statement {
            Statement::Declaration { name, value, span } => { if self.current_scope().contains_key(name) { return Err(DiagnosticBundle::single(Diagnostic::error("E2001", format!("Nama '{}' sudah dibuat di ruang lingkup ini.", name), Some(span.clone())))); } let ty = self.infer_at(value, span)?; self.current_scope().insert(name.clone(), ty); }
            Statement::Assignment { name, value, span } => { let expected = self.lookup(name).ok_or_else(|| DiagnosticBundle::single(Diagnostic::error("E2002", format!("Saya belum menemukan '{}'. Anda mungkin ingin membuatnya terlebih dahulu.", name), Some(span.clone()))))?; let actual = self.infer_at(value, span)?; if !compatible(expected, actual) { return Err(DiagnosticBundle::single(Diagnostic::error("E2003", format!("Nilai '{}' bertipe {}, tetapi membutuhkan {}.", name, actual.name(), expected.name()), Some(span.clone())))); } }
            Statement::Action { name, args, span } => match name.as_str() {
                "tampilkan" | "sapa" => { if args.len() != 1 { return Err(DiagnosticBundle::single(Diagnostic::error("E2004", format!("'{}' membutuhkan tepat satu nilai.", name), Some(span.clone())))); } self.infer_at(&args[0], span)?; }
                "cari alamat" => { if args.len() != 1 { return Err(DiagnosticBundle::single(Diagnostic::error("E2019", "'cari alamat' membutuhkan tepat satu target.", Some(span.clone())))); } let target = self.infer_at(&args[0], span)?; if !matches!(target, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2020", "Target 'cari alamat' harus berupa teks.", Some(span.clone())))); } }
                "baca berkas" => { if args.len() != 1 { return Err(DiagnosticBundle::single(Diagnostic::error("E2021", "'baca berkas' membutuhkan tepat satu target.", Some(span.clone())))); } let target = self.infer_at(&args[0], span)?; if !matches!(target, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2022", "Target 'baca berkas' harus berupa teks.", Some(span.clone())))); } }
                "ambil dari" => { if args.len() != 1 { return Err(DiagnosticBundle::single(Diagnostic::error("E2030", "'ambil dari' membutuhkan tepat satu URL HTTP.", Some(span.clone())))); } let target = self.infer_at(&args[0], span)?; if !matches!(target, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2031", "Target 'ambil dari' harus berupa teks URL.", Some(span.clone())))); } }
                "tulis ke berkas" => { if args.len() != 2 { return Err(DiagnosticBundle::single(Diagnostic::error("E2025", "'tulis ke berkas' membutuhkan target dan isi.", Some(span.clone())))); } let target = self.infer_at(&args[0], span)?; let content = self.infer_at(&args[1], span)?; if !matches!(target, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2026", "Target 'tulis ke berkas' harus berupa teks.", Some(span.clone())))); } if !matches!(content, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2027", "Isi 'tulis ke berkas' harus berupa teks.", Some(span.clone())))); } }
                "jalankan proses" => { if args.len() != 1 { return Err(DiagnosticBundle::single(Diagnostic::error("E2023", "'jalankan proses' membutuhkan tepat satu target.", Some(span.clone())))); } let target = self.infer_at(&args[0], span)?; if !matches!(target, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2024", "Target 'jalankan proses' harus berupa teks.", Some(span.clone())))); } }
                "kueri database" => { if args.len() < 2 { return Err(DiagnosticBundle::single(Diagnostic::error("E2028", "'kueri database' membutuhkan database dan SQL.", Some(span.clone())))); } for (index, arg) in args.iter().enumerate() { let ty = self.infer_at(arg, span)?; if !matches!(ty, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2029", format!("Argumen {} 'kueri database' harus berupa teks.", index + 1), Some(span.clone())))); } } }
                "buat daftar" => { if args.len() != 1 || !matches!(&args[0], Expr::Identifier(_)) { return Err(DiagnosticBundle::single(Diagnostic::error("E2005", "'buat daftar' membutuhkan nama daftar.", Some(span.clone())))); } if let Expr::Identifier(name) = &args[0] { self.current_scope().insert(name.clone(), Type::List); } }
                "buat pemetaan" => {
                    if args.len() != 1 || !matches!(&args[0], Expr::Identifier(_)) {
                        return Err(DiagnosticBundle::single(Diagnostic::error("E2032", "'buat pemetaan' membutuhkan nama pemetaan.", Some(span.clone()))));
                    }
                    if let Expr::Identifier(name) = &args[0] {
                        if self.lookup(name).is_some() {
                            return Err(DiagnosticBundle::single(Diagnostic::error("E2001", format!("Nama '{}' sudah dibuat di ruang lingkup ini.", name), Some(span.clone()))));
                        }
                        self.current_scope().insert(name.clone(), Type::Map);
                    }
                }
                "masukkan ke pemetaan" => {
                    if args.len() != 3 { return Err(DiagnosticBundle::single(Diagnostic::error("E2033", "'masukkan' membutuhkan kunci, nilai, dan nama pemetaan.", Some(span.clone())))); }
                    let key_type = self.infer_at(&args[0], span)?;
                    if !matches!(key_type, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2034", "Kunci pemetaan harus berupa teks.", Some(span.clone())))); }
                    self.infer_at(&args[1], span)?;
                    let map_name = match &args[2] { Expr::Identifier(name) => name, _ => return Err(DiagnosticBundle::single(Diagnostic::error("E2035", "Nama pemetaan harus berupa identifier.", Some(span.clone())))) };
                    match self.lookup(map_name) {
                        Some(Type::Map | Type::Any) => {},
                        Some(_) => return Err(DiagnosticBundle::single(Diagnostic::error("E2036", format!("'{}' bukan pemetaan.", map_name), Some(span.clone())))),
                        None => return Err(DiagnosticBundle::single(Diagnostic::error("E2037", format!("Pemetaan '{}' belum dibuat.", map_name), Some(span.clone())))),
                    }
                }
                "ambil kunci dari pemetaan" | "hapus kunci dari pemetaan" => {
                    if args.len() != 2 { return Err(DiagnosticBundle::single(Diagnostic::error("E2038", "Operasi kunci pemetaan membutuhkan kunci dan nama pemetaan.", Some(span.clone())))); }
                    let key_type = self.infer_at(&args[0], span)?;
                    if !matches!(key_type, Type::Text | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2039", "Kunci pemetaan harus berupa teks.", Some(span.clone())))); }
                    let map_name = match &args[1] { Expr::Identifier(name) => name, _ => return Err(DiagnosticBundle::single(Diagnostic::error("E2040", "Nama pemetaan harus berupa identifier.", Some(span.clone())))) };
                    match self.lookup(map_name) {
                        Some(Type::Map | Type::Any) => {},
                        Some(_) => return Err(DiagnosticBundle::single(Diagnostic::error("E2041", format!("'{}' bukan pemetaan.", map_name), Some(span.clone())))),
                        None => return Err(DiagnosticBundle::single(Diagnostic::error("E2042", format!("Pemetaan '{}' belum dibuat.", map_name), Some(span.clone())))),
                    }
                }
                "tambahkan" | "hapus" => { if args.len() != 2 { return Err(DiagnosticBundle::single(Diagnostic::error("E2006", format!("'{}' membutuhkan nilai dan nama daftar.", name), Some(span.clone())))); } if let Expr::Identifier(list_name) = &args[1] { match self.lookup(list_name) { Some(Type::List | Type::Any) => {}, Some(_) => return Err(DiagnosticBundle::single(Diagnostic::error("E2007", format!("'{}' bukan daftar.", list_name), Some(span.clone())))), None => return Err(DiagnosticBundle::single(Diagnostic::error("E2008", format!("Saya belum menemukan daftar '{}'.", list_name), Some(span.clone())))), } } else { return Err(DiagnosticBundle::single(Diagnostic::error("E2009", "Nama daftar harus berupa identifier.", Some(span.clone())))); } }
                _ => { for arg in args { self.infer_at(arg, span)?; } }
            },
            Statement::Conditional { condition, then_block, else_block, span } => { let ty = self.infer_at(condition, span)?; if !matches!(ty, Type::Boolean | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2010", "Kondisi harus menghasilkan boolean.", Some(span.clone())))); } self.scopes.push(HashMap::new()); self.analyze_block(then_block)?; self.scopes.pop(); if !else_block.is_empty() { self.scopes.push(HashMap::new()); self.analyze_block(else_block)?; self.scopes.pop(); } }
            Statement::Loop { kind, body, span } => match kind { LoopKind::ForEach { variable, iterable } => { let ty = self.infer_at(iterable, span)?; if !matches!(ty, Type::List | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2011", "Nilai setelah 'di' harus berupa daftar.", Some(span.clone())))); } self.scopes.push(HashMap::new()); self.current_scope().insert(variable.clone(), Type::Any); self.analyze_block(body)?; self.scopes.pop(); }, LoopKind::While { condition } => { let ty = self.infer_at(condition, span)?; if !matches!(ty, Type::Boolean | Type::Any) { return Err(DiagnosticBundle::single(Diagnostic::error("E2012", "Kondisi selama harus menghasilkan boolean.", Some(span.clone())))); } self.scopes.push(HashMap::new()); self.analyze_block(body)?; self.scopes.pop(); } },
            Statement::Function { params, result, body, span, .. } => { self.scopes.push(HashMap::new()); for param in params { self.current_scope().insert(param.clone(), Type::Any); } self.infer_at(result, span)?; self.analyze_block(body)?; self.scopes.pop(); }
            Statement::Event { body, .. } => { self.scopes.push(HashMap::new()); self.analyze_block(body)?; self.scopes.pop(); }
        }
        Ok(())
    }
    fn infer_at(&self, expr: &Expr, span: &crate::token::Span) -> Result<Type, DiagnosticBundle> {
        self.infer(expr).map_err(|mut bundle| {
            for diagnostic in &mut bundle.0 {
                if diagnostic.span.is_none() {
                    diagnostic.span = Some(span.clone());
                }
            }
            bundle
        })
    }
    fn infer(&self, expr: &Expr) -> Result<Type, DiagnosticBundle> { match expr { Expr::Text(_) => Ok(Type::Text), Expr::Number(_) => Ok(Type::Number), Expr::Boolean(_) => Ok(Type::Boolean), Expr::Nothing => Ok(Type::Nothing), Expr::Identifier(name) => self.lookup(name).ok_or_else(|| DiagnosticBundle::single(Diagnostic::error("E2014", format!("Saya belum menemukan '{}'.", name), None))), Expr::Binary { left, op, right } => { let l = self.infer(left)?; let r = self.infer(right)?; match op { BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => if matches!(l, Type::Number | Type::Any) && matches!(r, Type::Number | Type::Any) { Ok(Type::Number) } else { Err(DiagnosticBundle::single(Diagnostic::error("E2015", "Operasi hitung membutuhkan angka.", None))) }, BinaryOp::Equal | BinaryOp::Greater | BinaryOp::Less | BinaryOp::AtLeast | BinaryOp::AtMost => if compatible(l, r) || l == Type::Any || r == Type::Any { Ok(Type::Boolean) } else { Err(DiagnosticBundle::single(Diagnostic::error("E2016", "Dua nilai dibandingkan dengan tipe yang tidak kompatibel.", None))) } } }, Expr::Call { name, args } => { let arity = self.functions.get(name).copied().ok_or_else(|| DiagnosticBundle::single(Diagnostic::error("E2017", format!("Pekerjaan '{}' belum dibuat.", name), None)))?; if arity != args.len() { return Err(DiagnosticBundle::single(Diagnostic::error("E2018", format!("Pekerjaan '{}' membutuhkan {} parameter.", name, arity), None))); } let mut arg_types = Vec::with_capacity(args.len());
            for arg in args { arg_types.push(self.infer(arg)?); }
            let expected_types: Option<Vec<Type>> = match name.as_str() {
                "panjang teks" | "pangkas teks" => Some(vec![Type::Text]),
                "ubah nilai ke json" => Some(vec![Type::Any]),
                "nilai dari json" => Some(vec![Type::Text]),
                "waktu utc dari unix" => Some(vec![Type::Number]),
                "unix dari waktu utc" => Some(vec![Type::Text]),
                "teks diawali" | "teks diakhiri" => Some(vec![Type::Text, Type::Text]),
                "huruf kecil" | "huruf besar" => Some(vec![Type::Text]),
                "gabung teks" => Some(vec![Type::List, Type::Text]),
                "jumlah daftar angka" => Some(vec![Type::List]),
                "teks memuat" | "pisah teks" => Some(vec![Type::Text, Type::Text]),
                "ganti teks" => Some(vec![Type::Text, Type::Text, Type::Text]),
                "ubah teks ke angka" => Some(vec![Type::Text]),
                "panjang daftar" => Some(vec![Type::List]),
                "tambah elemen daftar" | "hapus elemen daftar" => Some(vec![Type::List, Type::Any]),
                "ambil indeks" => Some(vec![Type::List, Type::Number]),
                "nilai pemetaan" => Some(vec![Type::Map, Type::Text]),
                "atur pemetaan" => Some(vec![Type::Map, Type::Text, Type::Any]),
                "hapus kunci pemetaan" => Some(vec![Type::Map, Type::Text]),
                "nilai mutlak" => Some(vec![Type::Number]),
                "bagi bulat" | "angka sisa bagi" | "angka minimum" | "angka maksimum" => Some(vec![Type::Number, Type::Number]),
                _ => None,
            };
            if let Some(expected_types) = expected_types {
                for (index, (actual, expected)) in arg_types.iter().zip(expected_types.iter()).enumerate() {
                    if !compatible(*expected, *actual) {
                        return Err(DiagnosticBundle::single(Diagnostic::error(
                            "E2043",
                            format!("Argumen {} fungsi '{}' harus bertipe {}.", index + 1, name, expected.name()),
                            None,
                        )));
                    }
                }
            }
            Ok(match name.as_str() {
                "panjang teks" | "ubah teks ke angka" | "panjang daftar" | "jumlah daftar angka" | "nilai mutlak" | "bagi bulat" | "angka sisa bagi" | "angka minimum" | "angka maksimum" | "unix dari waktu utc" => Type::Number,
                "teks memuat" | "teks diawali" | "teks diakhiri" => Type::Boolean,
                "pangkas teks" | "ganti teks" | "ubah nilai ke json" | "waktu utc dari unix" | "huruf kecil" | "huruf besar" | "gabung teks" => Type::Text,
                "pisah teks" | "tambah elemen daftar" | "hapus elemen daftar" => Type::List,
                "atur pemetaan" | "hapus kunci pemetaan" => Type::Map,
                "ambil indeks" | "nilai dari json" => Type::Any,
                _ => Type::Any,
            }) } } }
}
fn compatible(expected: Type, actual: Type) -> bool { expected == actual || expected == Type::Any || actual == Type::Any }
