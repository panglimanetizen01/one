#![allow(clippy::possible_missing_else)]
#![allow(clippy::if_same_then_else)]

use crate::ast::{BinaryOp, Expr, LoopKind, Program, Statement};
use crate::diagnostic::{Diagnostic, DiagnosticBundle};
use crate::token::{Token, TokenKind};

const MAX_PARSE_DEPTH: usize = 128;
pub fn parse(source: &str) -> Result<Program, DiagnosticBundle> { let tokens = crate::lexer::lex(source)?; Parser::new(tokens).parse_program() }
pub fn parse_human_core(source: &str) -> Result<Program, DiagnosticBundle> { let tokens = crate::lexer::lex(source)?; Parser::new_human_core(tokens).parse_program() }
struct Parser { tokens: Vec<Token>, pos: usize, allow_mappings: bool, block_depth: usize, primary_depth: usize }
impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0, allow_mappings: false, block_depth: 0, primary_depth: 0 } }
    fn new_human_core(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0, allow_mappings: true, block_depth: 0, primary_depth: 0 } } fn current(&self) -> &Token { &self.tokens[self.pos] } fn at(&self, kind: TokenKind) -> bool { self.current().kind == kind } fn lexeme_is(&self, value: &str) -> bool { self.current().lexeme == value } fn peek_lexeme(&self, distance: usize) -> Option<&str> { self.tokens.get(self.pos + distance).map(|t| t.lexeme.as_str()) } fn advance(&mut self) -> Token { let token = self.current().clone(); self.pos += 1; token } fn skip_newlines(&mut self) { while self.at(TokenKind::Newline) { self.advance(); } } fn error(&self, code: &'static str, message: impl Into<String>) -> DiagnosticBundle { DiagnosticBundle::single(Diagnostic::error(code, message, Some(self.current().span.clone()))) } fn expect_newline(&mut self) -> Result<(), DiagnosticBundle> { if self.at(TokenKind::Newline) { self.advance(); Ok(()) } else { Err(self.error("E1001", "Baris pernyataan harus diakhiri newline.")) } } fn expect_indent(&mut self) -> Result<(), DiagnosticBundle> { if self.at(TokenKind::Indent) { self.advance(); Ok(()) } else { Err(self.error("E1002", "Blok harus dimulai dengan indentasi empat spasi.")) } } fn expect_dedent(&mut self) -> Result<(), DiagnosticBundle> { if self.at(TokenKind::Dedent) { self.advance(); Ok(()) } else { Err(self.error("E1003", "Blok belum ditutup dengan dedent yang valid.")) } } fn is_word(&self) -> bool { matches!(self.current().kind, TokenKind::Identifier | TokenKind::Keyword) }
    fn parse_program(mut self) -> Result<Program, DiagnosticBundle> { let mut statements = Vec::new(); self.skip_newlines(); while !self.at(TokenKind::Eof) { if self.at(TokenKind::Dedent) { return Err(self.error("E1004", "Dedent tidak memiliki blok induk.")); } statements.push(self.parse_statement()?); self.skip_newlines(); } Ok(Program { statements }) }
    fn parse_statement(&mut self) -> Result<Statement, DiagnosticBundle> { if self.lexeme_is("kalau") { return self.parse_conditional(); } if self.lexeme_is("untuk") { return self.parse_for_each(); } if self.lexeme_is("selama") { return self.parse_while(); } if self.lexeme_is("ketika") { return self.parse_event(); } if self.lexeme_is("buat") && self.peek_lexeme(1) == Some("pekerjaan") { return self.parse_function(); } if self.is_word() && self.has_declaration_boundary() { return self.parse_declaration_or_assignment(); } self.parse_action() }
    fn has_declaration_boundary(&self) -> bool { let mut i = self.pos; while let Some(token) = self.tokens.get(i) { if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof { break; } if token.lexeme == "adalah" || token.lexeme == "menjadi" { return i > self.pos; } i += 1; } false }
    fn parse_declaration_or_assignment(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.current().span.clone(); let mut words = Vec::new(); while self.is_word() && !self.lexeme_is("adalah") && !self.lexeme_is("menjadi") { words.push(self.advance().lexeme); } if words.is_empty() { return Err(self.error("E1001", "Nama membutuhkan kata penghubung.")); } let name = words.join(" "); let separator = self.advance().lexeme; let value = self.parse_expression_until_newline()?; self.expect_newline()?; if separator == "adalah" { Ok(Statement::Declaration { name, value, span: start }) } else { Ok(Statement::Assignment { name, value, span: start }) } }
    fn parse_action(&mut self) -> Result<Statement, DiagnosticBundle> { let first = self.advance(); let name = first.lexeme.clone(); let mut args = Vec::new(); let mut action_name_override: Option<String> = None; if name == "tampilkan" || name == "sapa" { args.push(self.parse_expression_until_newline()?); } else if name == "cari" && self.lexeme_is("alamat") { self.advance(); args.push(self.parse_expression_until_newline()?); } else if name == "baca" && self.lexeme_is("berkas") { self.advance(); args.push(self.parse_expression_until_newline()?); } else if name == "tulis" && self.lexeme_is("ke") && self.peek_lexeme(1) == Some("berkas") { self.advance(); self.advance(); args.push(self.parse_expression_until_boundary("dengan")?); if !self.lexeme_is("dengan") { return Err(self.error("E1019", "'tulis ke berkas' membutuhkan kata 'dengan' sebelum isi.")); } self.advance(); args.push(self.parse_expression_until_newline()?); } else if self.allow_mappings && name == "ambil" && self.lexeme_is("kunci") {
            self.advance();
            args.push(self.parse_expression_until_boundary("dari")?);
            if !self.lexeme_is("dari") { return Err(self.error("E1021", "'ambil kunci' membutuhkan kata 'dari' sebelum nama pemetaan.")); }
            self.advance();
            args.push(self.parse_name_reference()?);
            action_name_override = Some("ambil kunci dari pemetaan".into());
        } else if name == "ambil" && self.lexeme_is("dari") {
            self.advance();
            args.push(self.parse_expression_until_newline()?);
        } else if name == "jalankan" && self.lexeme_is("proses") { self.advance(); args.push(self.parse_expression_until_newline()?); } else if name == "kueri" && self.lexeme_is("database") { self.advance(); args.push(self.parse_expression_until_boundary("dengan")?); if !self.lexeme_is("dengan") { return Err(self.error("E1020", "'kueri database' membutuhkan kata 'dengan' sebelum SQL.")); } self.advance();
        args.push(self.parse_primary_with_boundary(None, true)?);
        while self.lexeme_is("dan") {
            self.advance();
            args.push(self.parse_primary_with_boundary(None, true)?);
        }
    } else if self.allow_mappings && name == "buat" && self.lexeme_is("pemetaan") {
            self.advance();
            args.push(self.parse_name_reference()?);
            action_name_override = Some("buat pemetaan".into());
        } else if name == "buat" && self.lexeme_is("daftar") { self.advance(); args.push(self.parse_name_reference()?); } else if self.allow_mappings && name == "masukkan" {
            args.push(self.parse_expression_until_boundary("dan")?);
            if !self.lexeme_is("dan") { return Err(self.error("E1022", "'masukkan' membutuhkan kata 'dan' antara kunci dan nilai.")); }
            self.advance();
            args.push(self.parse_expression_until_boundary("ke")?);
            if !self.lexeme_is("ke") { return Err(self.error("E1023", "'masukkan' membutuhkan kata 'ke' sebelum nama pemetaan.")); }
            self.advance();
            args.push(self.parse_name_reference()?);
            action_name_override = Some("masukkan ke pemetaan".into());
        } else if self.allow_mappings && name == "hapus" && self.lexeme_is("kunci") {
            self.advance();
            args.push(self.parse_expression_until_boundary("dari")?);
            if !self.lexeme_is("dari") { return Err(self.error("E1024", "'hapus kunci' membutuhkan kata 'dari' sebelum nama pemetaan.")); }
            self.advance();
            args.push(self.parse_name_reference()?);
            action_name_override = Some("hapus kunci dari pemetaan".into());
        } else if name == "tambahkan" { args.push(self.parse_expression_until_boundary("ke")?); if !self.lexeme_is("ke") { return Err(self.error("E1017", "'tambahkan' membutuhkan kata 'ke' sebelum nama daftar.")); } self.advance(); args.push(self.parse_name_reference()?); } else if name == "hapus" { args.push(self.parse_expression_until_boundary("dari")?); if !self.lexeme_is("dari") { return Err(self.error("E1018", "'hapus' membutuhkan kata 'dari' sebelum nama daftar.")); } self.advance(); args.push(self.parse_name_reference()?); } else { return Err(self.error("E1016", format!("Aksi '{}' tidak termasuk grammar One v0.1.", name))); } self.expect_newline()?; let action_name = action_name_override.unwrap_or_else(|| if name == "buat" { "buat daftar".into() } else if name == "cari" { "cari alamat".into() } else if name == "baca" { "baca berkas".into() } else if name == "tulis" { "tulis ke berkas".into() } else if name == "jalankan" { "jalankan proses".into() } else if name == "ambil" { "ambil dari".into() } else if name == "kueri" { "kueri database".into() } else { name }); Ok(Statement::Action { name: action_name, args, span: first.span }) }
    fn parse_name_reference(&mut self) -> Result<Expr, DiagnosticBundle> { if !self.is_word() { return Err(self.error("E1016", "Nama harus berupa kata atau nama gabungan.")); } let mut words = Vec::new(); while self.is_word() { words.push(self.advance().lexeme); } Ok(Expr::Identifier(words.join(" "))) }
    fn parse_conditional(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; let condition = self.parse_expression_until_newline()?; self.expect_newline()?; self.expect_indent()?; let then_block = self.parse_block()?; self.expect_dedent()?; let mut else_block = Vec::new(); if self.lexeme_is("kalau") && self.peek_lexeme(1) == Some("tidak") { self.advance(); self.advance(); self.expect_newline()?; self.expect_indent()?; else_block = self.parse_block()?; self.expect_dedent()?; } Ok(Statement::Conditional { condition, then_block, else_block, span: start }) }
    fn parse_for_each(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; if !self.lexeme_is("setiap") { return Err(self.error("E1005", "Loop harus menggunakan 'untuk setiap'.")); } self.advance(); let mut variable_words = Vec::new(); while self.is_word() && !self.lexeme_is("di") { variable_words.push(self.advance().lexeme); } if variable_words.is_empty() { return Err(self.error("E1006", "Loop membutuhkan nama variabel.")); } let variable = variable_words.join(" "); if !self.lexeme_is("di") { return Err(self.error("E1007", "Loop membutuhkan kata 'di'.")); } self.advance(); let iterable = self.parse_name_reference()?; self.expect_newline()?; self.expect_indent()?; let body = self.parse_block()?; self.expect_dedent()?; Ok(Statement::Loop { kind: LoopKind::ForEach { variable, iterable }, body, span: start }) }
    fn parse_while(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; let condition = self.parse_expression_until_newline()?; self.expect_newline()?; self.expect_indent()?; let body = self.parse_block()?; self.expect_dedent()?; Ok(Statement::Loop { kind: LoopKind::While { condition }, body, span: start }) }
    fn parse_function(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; self.advance(); let mut name_words = Vec::new(); while self.is_word() { name_words.push(self.advance().lexeme); } let name = name_words.join(" "); self.expect_newline()?; self.expect_indent()?; let mut params = Vec::new(); while self.lexeme_is("terima") { self.advance(); let mut param_words = Vec::new(); while self.is_word() { param_words.push(self.advance().lexeme); } if param_words.is_empty() { return Err(self.error("E1009", "Parameter membutuhkan nama.")); } params.push(param_words.join(" ")); self.expect_newline()?; } if !self.lexeme_is("hasilnya") || self.peek_lexeme(1) != Some("adalah") { return Err(self.error("E1010", "Pekerjaan membutuhkan baris 'hasilnya adalah ...'.")); } self.advance(); self.advance(); let result = self.parse_expression_until_newline()?; self.expect_newline()?; let body = if self.at(TokenKind::Dedent) { Vec::new() } else { self.parse_block()? }; self.expect_dedent()?; Ok(Statement::Function { name, params, result, body, span: start }) }
    fn parse_event(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; let mut trigger = Vec::new(); while !self.at(TokenKind::Newline) && !self.at(TokenKind::Eof) { trigger.push(self.advance().lexeme); } if trigger.is_empty() { return Err(self.error("E1011", "Event membutuhkan pemicu yang eksplisit.")); } self.expect_newline()?; self.expect_indent()?; let body = self.parse_block()?; self.expect_dedent()?; Ok(Statement::Event { trigger, body, span: start }) }
    fn parse_block(&mut self) -> Result<Vec<Statement>, DiagnosticBundle> {
        if self.block_depth >= MAX_PARSE_DEPTH {
            return Err(self.error("E1025", "Kedalaman blok melebihi batas 128 tingkat."));
        }
        self.block_depth += 1;
        let result = self.parse_block_inner();
        self.block_depth -= 1;
        result
    }
    fn parse_block_inner(&mut self) -> Result<Vec<Statement>, DiagnosticBundle> {
        let mut statements = Vec::new();
        self.skip_newlines();
        if self.at(TokenKind::Dedent) {
            return Err(self.error("E1012", "Blok tidak boleh kosong."));
        }
        while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }
        if statements.is_empty() {
            return Err(self.error("E1012", "Blok tidak boleh kosong."));
        }
        Ok(statements)
    }
    fn parse_expression_until_newline(&mut self) -> Result<Expr, DiagnosticBundle> { self.parse_expression_until(None) } fn parse_expression_until_boundary(&mut self, boundary: &str) -> Result<Expr, DiagnosticBundle> { self.parse_expression_until(Some(boundary)) }
    fn parse_expression_until(&mut self, boundary: Option<&str>) -> Result<Expr, DiagnosticBundle> { if self.at(TokenKind::Newline) || self.at(TokenKind::Eof) || self.at(TokenKind::Dedent) || boundary.is_some_and(|value| self.lexeme_is(value)) { return Err(self.error("E1013", "Pernyataan membutuhkan nilai atau ekspresi.")); } let mut expr = self.parse_primary_with_boundary(boundary, false)?; let mut operators = 0usize; loop { if self.at(TokenKind::Newline) || self.at(TokenKind::Eof) || self.at(TokenKind::Dedent) || boundary.is_some_and(|value| self.lexeme_is(value)) { break; } let (op, consumed) = self.match_operator().ok_or_else(|| self.error("E1014", "Ekspresi mengandung kata yang tidak sesuai grammar."))?; operators += 1; if operators > MAX_PARSE_DEPTH { return Err(self.error("E1027", "Jumlah operator dalam satu ekspresi melebihi batas 128.")); } for _ in 0..consumed { self.advance(); } let right = self.parse_primary_with_boundary(boundary, false)?; expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) }; if matches!(op, BinaryOp::AtLeast | BinaryOp::AtMost) && self.lexeme_is("tahun") { self.advance(); } } Ok(expr) }
    fn match_operator(&self) -> Option<(BinaryOp, usize)> { if self.lexeme_is("ditambah") { return Some((BinaryOp::Add, 1)); } if self.lexeme_is("dikurangi") { return Some((BinaryOp::Subtract, 1)); } if self.lexeme_is("dikali") { return Some((BinaryOp::Multiply, 1)); } if self.lexeme_is("dibagi") { return Some((BinaryOp::Divide, 1)); } if self.lexeme_is("sudah") { return Some((BinaryOp::AtLeast, 1)); } if self.lexeme_is("belum") { return Some((BinaryOp::AtMost, 1)); } if self.lexeme_is("sama") && self.peek_lexeme(1) == Some("dengan") { return Some((BinaryOp::Equal, 2)); } if self.lexeme_is("lebih") && self.peek_lexeme(1) == Some("besar") && self.peek_lexeme(2) == Some("dari") { return Some((BinaryOp::Greater, 3)); } if self.lexeme_is("lebih") && self.peek_lexeme(1) == Some("kecil") && self.peek_lexeme(2) == Some("dari") { return Some((BinaryOp::Less, 3)); } None }
    fn parse_primary_with_boundary(&mut self, boundary: Option<&str>, stop_at_and: bool) -> Result<Expr, DiagnosticBundle> {
        if self.primary_depth >= MAX_PARSE_DEPTH {
            return Err(self.error("E1026", "Kedalaman ekspresi melebihi batas 128 tingkat."));
        }
        self.primary_depth += 1;
        let result = self.parse_primary_inner(boundary, stop_at_and);
        self.primary_depth -= 1;
        result
    }
    fn parse_primary_inner(&mut self, boundary: Option<&str>, stop_at_and: bool) -> Result<Expr, DiagnosticBundle> { let token = self.advance(); match token.kind { TokenKind::Text => Ok(Expr::Text(token.lexeme)), TokenKind::Number => token.lexeme.parse::<i64>().map(Expr::Number).map_err(|_| self.error("E1015", "Angka tidak valid.")), TokenKind::Identifier | TokenKind::Keyword => match token.lexeme.as_str() { "benar" => Ok(Expr::Boolean(true)), "salah" => Ok(Expr::Boolean(false)), "tidak" if self.lexeme_is("ada") => { self.advance(); Ok(Expr::Nothing) }, _ => { let mut words = vec![token.lexeme]; while self.is_word() && !self.lexeme_is("dengan") && self.match_operator().is_none() && !boundary.is_some_and(|value| self.lexeme_is(value)) && !(stop_at_and && self.lexeme_is("dan")) { words.push(self.advance().lexeme); } let name = words.join(" "); if self.lexeme_is("dengan") { self.advance(); let mut args = vec![self.parse_primary_with_boundary(None, true)?]; while self.lexeme_is("dan") { self.advance(); args.push(self.parse_primary_with_boundary(None, true)?); } return Ok(Expr::Call { name, args }); } Ok(Expr::Identifier(name)) } }, _ => Err(self.error("E1016", "Token ini tidak dapat menjadi ekspresi.")) } 
    }
}

#[cfg(test)]
mod ast_contract_tests {
    use super::*;
    use crate::token::Span;

    #[test]
    fn declaration_normalizes_multi_word_name_and_anchors_span() {
        let program = parse("nama pengguna adalah \"Budi\"\n").expect("valid declaration");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Declaration { name, value: Expr::Text(value), span } => {
                assert_eq!(name, "nama pengguna");
                assert_eq!(value, "Budi");
                assert_eq!(span, &Span::new(0, 4, 1, 1));
            }
            other => panic!("unexpected AST: {other:?}"),
        }
    }

    #[test]
    fn assignment_normalizes_multi_word_name_and_keeps_integer_value() {
        let program = parse("saldo rekening menjadi 100000\n").expect("valid assignment");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Assignment { name, value: Expr::Number(value), span } => {
                assert_eq!(name, "saldo rekening");
                assert_eq!(*value, 100000);
                assert_eq!(span, &Span::new(0, 5, 1, 1));
            }
            other => panic!("unexpected AST: {other:?}"),
        }
    }

    #[test]
    fn list_action_keeps_grammar_selected_multi_word_target() {
        let program = parse("tambahkan apel ke daftar buah\n").expect("valid list action");
        match &program.statements[0] {
            Statement::Action { name, args, .. } => {
                assert_eq!(name, "tambahkan");
                assert_eq!(args, &vec![
                    Expr::Identifier("apel".into()),
                    Expr::Identifier("daftar buah".into()),
                ]);
            }
            other => panic!("unexpected AST: {other:?}"),
        }
    }

    #[test]
    fn unsupported_action_does_not_fall_back_to_generic_ast() {
        let error = parse("apel\n").expect_err("unsupported action must be rejected");
        assert_eq!(error.0[0].code, "E1016");
    }

    #[test]
    fn repeated_parse_produces_identical_ast_and_span_anchors() {
        let source = "nama pengguna adalah \"Budi\"\n";
        let first = parse(source).expect("valid source");
        let second = parse(source).expect("same valid source");
        assert_eq!(first, second);
    }
    #[test]
    fn deeply_nested_blocks_return_a_diagnostic_instead_of_overflowing_the_stack() {
        let mut source = String::new();
        for depth in 0..=MAX_PARSE_DEPTH {
            source.push_str(&" ".repeat(depth * 4));
            source.push_str("kalau benar\n");
        }
        source.push_str(&" ".repeat((MAX_PARSE_DEPTH + 1) * 4));
        source.push_str("tampilkan \"aman\"\n");

        let error = parse(&source).expect_err("excessive block nesting must be rejected");
        assert_eq!(error.0[0].code, "E1025");
    }

    #[test]
    fn deeply_nested_call_arguments_return_a_diagnostic() {
        let source = format!("tampilkan {}1\n", "fungsi dengan ".repeat(MAX_PARSE_DEPTH + 1));
        let error = parse(&source).expect_err("excessive nested calls must be rejected");
        assert_eq!(error.0[0].code, "E1026");
    }

    #[test]
    fn excessively_long_binary_expression_returns_a_diagnostic() {
        let source = format!("tampilkan 1 {}1\n", "ditambah 1 ".repeat(MAX_PARSE_DEPTH));
        let error = parse(&source).expect_err("excessive binary expression depth must be rejected");
        assert_eq!(error.0[0].code, "E1027");
    }


}
