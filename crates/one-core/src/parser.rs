#![allow(clippy::possible_missing_else)]

use crate::ast::{BinaryOp, Expr, LoopKind, Program, Statement};
use crate::diagnostic::{Diagnostic, DiagnosticBundle};
use crate::token::{Token, TokenKind};

pub fn parse(source: &str) -> Result<Program, DiagnosticBundle> { let tokens = crate::lexer::lex(source)?; Parser::new(tokens).parse_program() }

struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn current(&self) -> &Token { &self.tokens[self.pos] }
    fn at(&self, kind: TokenKind) -> bool { self.current().kind == kind }
    fn lexeme_is(&self, value: &str) -> bool { self.current().lexeme == value }
    fn peek_lexeme(&self, distance: usize) -> Option<&str> { self.tokens.get(self.pos + distance).map(|t| t.lexeme.as_str()) }
    fn advance(&mut self) -> Token { let token = self.current().clone(); self.pos += 1; token }
    fn skip_newlines(&mut self) { while self.at(TokenKind::Newline) { self.advance(); } }
    fn error(&self, code: &'static str, message: impl Into<String>) -> DiagnosticBundle { DiagnosticBundle::single(Diagnostic::error(code, message, Some(self.current().span.clone()))) }
    fn expect_newline(&mut self) -> Result<(), DiagnosticBundle> { if self.at(TokenKind::Newline) { self.advance(); Ok(()) } else { Err(self.error("E1001", "Baris pernyataan harus diakhiri newline.")) } }
    fn expect_indent(&mut self) -> Result<(), DiagnosticBundle> { if self.at(TokenKind::Indent) { self.advance(); Ok(()) } else { Err(self.error("E1002", "Blok harus dimulai dengan indentasi empat spasi.")) } }
    fn expect_dedent(&mut self) -> Result<(), DiagnosticBundle> { if self.at(TokenKind::Dedent) { self.advance(); Ok(()) } else { Err(self.error("E1003", "Blok belum ditutup dengan dedent yang valid.")) } }
    fn is_word(&self) -> bool { matches!(self.current().kind, TokenKind::Identifier | TokenKind::Keyword) }

    fn parse_program(mut self) -> Result<Program, DiagnosticBundle> {
        let mut statements = Vec::new(); self.skip_newlines();
        while !self.at(TokenKind::Eof) { if self.at(TokenKind::Dedent) { return Err(self.error("E1004", "Dedent tidak memiliki blok induk.")); } statements.push(self.parse_statement()?); self.skip_newlines(); }
        Ok(Program { statements })
    }
    fn parse_statement(&mut self) -> Result<Statement, DiagnosticBundle> {
        if self.lexeme_is("kalau") { return self.parse_conditional(); }
        if self.lexeme_is("untuk") { return self.parse_for_each(); }
        if self.lexeme_is("selama") { return self.parse_while(); }
        if self.lexeme_is("ketika") { return self.parse_event(); }
        if self.lexeme_is("buat") && self.peek_lexeme(1) == Some("pekerjaan") { return self.parse_function(); }
        if self.is_word() && self.has_declaration_boundary() { return self.parse_declaration_or_assignment(); }
        self.parse_action()
    }
    fn has_declaration_boundary(&self) -> bool {
        let mut i = self.pos;
        while let Some(token) = self.tokens.get(i) { if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof { break; } if token.lexeme == "adalah" || token.lexeme == "menjadi" { return i > self.pos; } i += 1; }
        false
    }
    fn parse_declaration_or_assignment(&mut self) -> Result<Statement, DiagnosticBundle> {
        let start = self.current().span.clone(); let mut words = Vec::new();
        while self.is_word() && !self.lexeme_is("adalah") && !self.lexeme_is("menjadi") { words.push(self.advance().lexeme); }
        if words.is_empty() { return Err(self.error("E1001", "Nama membutuhkan kata penghubung.")); }
        let name = words.join(" "); let separator = self.advance().lexeme; let value = self.parse_expression_until_newline()?; self.expect_newline()?;
        if separator == "adalah" { Ok(Statement::Declaration { name, value, span: start }) } else { Ok(Statement::Assignment { name, value, span: start }) }
    }
    fn parse_action(&mut self) -> Result<Statement, DiagnosticBundle> {
        let first = self.advance(); let name = first.lexeme.clone(); let mut args = Vec::new();
        if name == "buat" && self.lexeme_is("daftar") { self.advance(); args.push(self.parse_name_reference()?); }
        else if name == "tambahkan" { args.push(self.parse_primary()?); if !self.lexeme_is("ke") { return Err(self.error("E1017", "'tambahkan' membutuhkan kata 'ke' sebelum nama daftar.")); } self.advance(); args.push(self.parse_name_reference()?); }
        else if name == "hapus" { args.push(self.parse_primary()?); if !self.lexeme_is("dari") { return Err(self.error("E1018", "'hapus' membutuhkan kata 'dari' sebelum nama daftar.")); } self.advance(); args.push(self.parse_name_reference()?); }
        else if !self.at(TokenKind::Newline) && !self.at(TokenKind::Eof) { args.push(self.parse_expression_until_newline()?); }
        self.expect_newline()?; let action_name = if name == "buat" && args.len() == 1 { "buat daftar".into() } else { name };
        Ok(Statement::Action { name: action_name, args, span: first.span })
    }
    fn parse_name_reference(&mut self) -> Result<Expr, DiagnosticBundle> {
        if !self.is_word() { return Err(self.error("E1016", "Nama harus berupa kata atau nama gabungan.")); }
        let mut words = Vec::new(); while self.is_word() && !self.lexeme_is("dengan") && !self.lexeme_is("dan") { words.push(self.advance().lexeme); }
        Ok(Expr::Identifier(words.join(" ")))
    }
    fn parse_conditional(&mut self) -> Result<Statement, DiagnosticBundle> {
        let start = self.advance().span; let condition = self.parse_expression_until_newline()?; self.expect_newline()?; self.expect_indent()?; let then_block = self.parse_block()?; self.expect_dedent()?; let mut else_block = Vec::new();
        if self.lexeme_is("kalau") && self.peek_lexeme(1) == Some("tidak") { self.advance(); self.advance(); self.expect_newline()?; self.expect_indent()?; else_block = self.parse_block()?; self.expect_dedent()?; }
        Ok(Statement::Conditional { condition, then_block, else_block, span: start })
    }
    fn parse_for_each(&mut self) -> Result<Statement, DiagnosticBundle> {
        let start = self.advance().span; if !self.lexeme_is("setiap") { return Err(self.error("E1005", "Loop harus menggunakan 'untuk setiap'.")); } self.advance();
        let mut variable_words = Vec::new(); while self.is_word() && !self.lexeme_is("di") { variable_words.push(self.advance().lexeme); }
        if variable_words.is_empty() { return Err(self.error("E1006", "Loop membutuhkan nama variabel.")); }
        let variable = variable_words.join(" "); if !self.lexeme_is("di") { return Err(self.error("E1007", "Loop membutuhkan kata 'di'.")); } self.advance(); let iterable = self.parse_name_reference()?; self.expect_newline()?; self.expect_indent()?; let body = self.parse_block()?; self.expect_dedent()?;
        Ok(Statement::Loop { kind: LoopKind::ForEach { variable, iterable }, body, span: start })
    }
    fn parse_while(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; let condition = self.parse_expression_until_newline()?; self.expect_newline()?; self.expect_indent()?; let body = self.parse_block()?; self.expect_dedent()?; Ok(Statement::Loop { kind: LoopKind::While { condition }, body, span: start }) }
    fn parse_function(&mut self) -> Result<Statement, DiagnosticBundle> {
        let start = self.advance().span; self.advance(); let mut name_words = Vec::new(); while self.is_word() { name_words.push(self.advance().lexeme); } let name = name_words.join(" "); self.expect_newline()?; self.expect_indent()?; let mut params = Vec::new();
        while self.lexeme_is("terima") { self.advance(); let mut param_words = Vec::new(); while self.is_word() { param_words.push(self.advance().lexeme); } if param_words.is_empty() { return Err(self.error("E1009", "Parameter membutuhkan nama.")); } params.push(param_words.join(" ")); self.expect_newline()?; }
        if !self.lexeme_is("hasilnya") || self.peek_lexeme(1) != Some("adalah") { return Err(self.error("E1010", "Pekerjaan membutuhkan baris 'hasilnya adalah ...'.")); }
        self.advance(); self.advance(); let result = self.parse_expression_until_newline()?; self.expect_newline()?; let body = if self.at(TokenKind::Dedent) { Vec::new() } else { self.parse_block()? }; self.expect_dedent()?;
        Ok(Statement::Function { name, params, result, body, span: start })
    }
    fn parse_event(&mut self) -> Result<Statement, DiagnosticBundle> { let start = self.advance().span; let mut trigger = Vec::new(); while !self.at(TokenKind::Newline) && !self.at(TokenKind::Eof) { trigger.push(self.advance().lexeme); } if trigger.is_empty() { return Err(self.error("E1011", "Event membutuhkan pemicu yang eksplisit.")); } self.expect_newline()?; self.expect_indent()?; let body = self.parse_block()?; self.expect_dedent()?; Ok(Statement::Event { trigger, body, span: start }) }
    fn parse_block(&mut self) -> Result<Vec<Statement>, DiagnosticBundle> { let mut statements = Vec::new(); self.skip_newlines(); if self.at(TokenKind::Dedent) { return Err(self.error("E1012", "Blok tidak boleh kosong.")); } while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) { statements.push(self.parse_statement()?); self.skip_newlines(); } if statements.is_empty() { return Err(self.error("E1012", "Blok tidak boleh kosong.")); } Ok(statements) }
    fn parse_expression_until_newline(&mut self) -> Result<Expr, DiagnosticBundle> {
        if self.at(TokenKind::Newline) || self.at(TokenKind::Eof) || self.at(TokenKind::Dedent) { return Err(self.error("E1013", "Pernyataan membutuhkan nilai atau ekspresi.")); }
        let mut expr = self.parse_primary()?; loop { if self.at(TokenKind::Newline) || self.at(TokenKind::Eof) || self.at(TokenKind::Dedent) { break; } let (op, consumed) = self.match_operator().ok_or_else(|| self.error("E1014", "Ekspresi mengandung kata yang tidak sesuai grammar."))?; for _ in 0..consumed { self.advance(); } let right = self.parse_primary()?; expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) }; if matches!(op, BinaryOp::AtLeast | BinaryOp::AtMost) && self.lexeme_is("tahun") { self.advance(); } } Ok(expr)
    }
    fn match_operator(&self) -> Option<(BinaryOp, usize)> {
        if self.lexeme_is("ditambah") { return Some((BinaryOp::Add, 1)); } if self.lexeme_is("dikurangi") { return Some((BinaryOp::Subtract, 1)); } if self.lexeme_is("dikali") { return Some((BinaryOp::Multiply, 1)); } if self.lexeme_is("dibagi") { return Some((BinaryOp::Divide, 1)); } if self.lexeme_is("sudah") { return Some((BinaryOp::AtLeast, 1)); } if self.lexeme_is("belum") { return Some((BinaryOp::AtMost, 1)); } if self.lexeme_is("sama") && self.peek_lexeme(1) == Some("dengan") { return Some((BinaryOp::Equal, 2)); } if self.lexeme_is("lebih") && self.peek_lexeme(1) == Some("besar") && self.peek_lexeme(2) == Some("dari") { return Some((BinaryOp::Greater, 3)); } if self.lexeme_is("lebih") && self.peek_lexeme(1) == Some("kecil") && self.peek_lexeme(2) == Some("dari") { return Some((BinaryOp::Less, 3)); } None
    }
    fn parse_primary(&mut self) -> Result<Expr, DiagnosticBundle> {
        let token = self.advance(); match token.kind {
            TokenKind::Text => Ok(Expr::Text(token.lexeme)), TokenKind::Number => token.lexeme.parse::<i64>().map(Expr::Number).map_err(|_| self.error("E1015", "Angka tidak valid.")),
            TokenKind::Identifier | TokenKind::Keyword => match token.lexeme.as_str() {
                "benar" => Ok(Expr::Boolean(true)), "salah" => Ok(Expr::Boolean(false)), "tidak" if self.lexeme_is("ada") => { self.advance(); Ok(Expr::Nothing) },
                _ => { let mut words = vec![token.lexeme]; while self.is_word() && !self.lexeme_is("dengan") && !self.lexeme_is("ke") && !self.lexeme_is("dari") && self.match_operator().is_none() { words.push(self.advance().lexeme); } let name = words.join(" "); if self.lexeme_is("dengan") { self.advance(); let mut args = vec![self.parse_primary()?]; while self.lexeme_is("dan") { self.advance(); args.push(self.parse_primary()?); } return Ok(Expr::Call { name, args }); } Ok(Expr::Identifier(name)) }
            },
            _ => Err(self.error("E1016", "Token ini tidak dapat menjadi ekspresi.")),
        }
    }
}