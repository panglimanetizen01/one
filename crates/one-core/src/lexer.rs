use crate::diagnostic::{Diagnostic, DiagnosticBundle};
use crate::token::{Span, Token, TokenKind};

const INDENT_WIDTH: usize = 4;

pub fn lex(source: &str) -> Result<Vec<Token>, DiagnosticBundle> { Lexer::new(source).scan() }

pub struct Lexer<'a> { source: &'a str }

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self { Self { source } }

    pub fn scan(&mut self) -> Result<Vec<Token>, DiagnosticBundle> {
        let mut tokens = Vec::new();
        let mut indent_stack = vec![0usize];
        let mut offset = 0usize;
        let mut line_no = 1usize;

        for raw_line in self.source.split('\n') {
            let line_start = offset;
            offset += raw_line.len() + 1;
            let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
            let trimmed = line.trim_start_matches(' ');
            if trimmed.trim().is_empty() || trimmed.starts_with('#') { line_no += 1; continue; }
            let leading = line.len() - trimmed.len();
            if line[..leading].contains('\t') { return Err(DiagnosticBundle::single(Diagnostic::error("E0001", "Indentasi menggunakan tab tidak didukung. Gunakan empat spasi per tingkat.", Some(Span::new(line_start, line_start + leading, line_no, 1))))); }
            if leading % INDENT_WIDTH != 0 { return Err(DiagnosticBundle::single(Diagnostic::error("E0002", "Indentasi harus menggunakan kelipatan empat spasi.", Some(Span::new(line_start, line_start + leading, line_no, 1))))); }
            let current = *indent_stack.last().unwrap();
            if leading > current {
                if leading != current + INDENT_WIDTH { return Err(DiagnosticBundle::single(Diagnostic::error("E0003", "Blok baru harus menjorok tepat satu tingkat.", Some(Span::new(line_start, line_start + leading, line_no, 1))))); }
                indent_stack.push(leading); tokens.push(Token::new(TokenKind::Indent, "", Span::new(line_start, line_start + leading, line_no, 1)));
            } else if leading < current {
                while *indent_stack.last().unwrap() > leading { indent_stack.pop(); tokens.push(Token::new(TokenKind::Dedent, "", Span::new(line_start, line_start + leading, line_no, 1))); }
                if *indent_stack.last().unwrap() != leading { return Err(DiagnosticBundle::single(Diagnostic::error("E0004", "Indentasi kembali ke tingkat yang tidak pernah digunakan sebelumnya.", Some(Span::new(line_start, line_start + leading, line_no, 1))))); }
            }

            let mut chars = trimmed.char_indices().peekable();
            while let Some((rel, ch)) = chars.next() {
                if ch == ' ' { continue; }
                if ch == '#' { break; }
                let absolute = line_start + leading + rel;
                if ch == '"' {
                    let start = absolute; let mut value = String::new(); let mut closed = false;
                    while let Some((_, c)) = chars.next() {
                        match c {
                            '"' => { closed = true; break; }
                            '\\' => match chars.next().map(|(_, e)| e) {
                                Some('"') => value.push('"'), Some('\\') => value.push('\\'), Some('n') => value.push('\n'), Some('t') => value.push('\t'),
                                _ => return Err(DiagnosticBundle::single(Diagnostic::error("E0005", "Escape string tidak dikenal.", Some(Span::new(start, absolute + 1, line_no, rel + 1))))),
                            },
                            other => value.push(other),
                        }
                    }
                    if !closed { return Err(DiagnosticBundle::single(Diagnostic::error("E0006", "String belum ditutup dengan tanda kutip.", Some(Span::new(start, absolute + 1, line_no, rel + 1))))); }
                    tokens.push(Token::new(TokenKind::Text, value, Span::new(start, absolute + 1, line_no, rel + 1))); continue;
                }
                if ch.is_ascii_digit() {
                    let start = rel; let mut end = rel + ch.len_utf8();
                    while let Some((idx, next)) = chars.peek().copied() { if next.is_ascii_digit() { chars.next(); end = idx + next.len_utf8(); } else { break; } }
                    let text = &trimmed[start..end]; tokens.push(Token::new(TokenKind::Number, text, Span::new(absolute, absolute + text.len(), line_no, rel + 1))); continue;
                }
                if ch.is_alphabetic() || ch == '_' {
                    let start = rel; let mut end = rel + ch.len_utf8();
                    while let Some((idx, next)) = chars.peek().copied() { if next.is_alphanumeric() || next == '_' { chars.next(); end = idx + next.len_utf8(); } else { break; } }
                    let text = &trimmed[start..end]; let kind = if is_keyword(text) { TokenKind::Keyword } else { TokenKind::Identifier };
                    tokens.push(Token::new(kind, text, Span::new(absolute, absolute + text.len(), line_no, rel + 1))); continue;
                }
                return Err(DiagnosticBundle::single(Diagnostic::error("E0007", format!("Karakter '{}' tidak dikenal oleh One v0.1.", ch), Some(Span::new(absolute, absolute + ch.len_utf8(), line_no, rel + 1)))));
            }
            tokens.push(Token::new(TokenKind::Newline, "", Span::new(line_start + line.len(), line_start + line.len(), line_no, line.len() + 1)));
            line_no += 1;
        }
        while indent_stack.len() > 1 { indent_stack.pop(); tokens.push(Token::new(TokenKind::Dedent, "", Span::new(self.source.len(), self.source.len(), line_no, 1))); }
        tokens.push(Token::new(TokenKind::Eof, "", Span::new(self.source.len(), self.source.len(), line_no, 1)));
        Ok(tokens)
    }
}

fn is_keyword(word: &str) -> bool {
    matches!(word, "buat" | "daftar" | "tambahkan" | "hapus" | "dari" | "ke" | "adalah" | "menjadi" | "tampilkan" | "kalau" | "tidak" | "umur" | "sudah" | "belum" | "tahun" | "untuk" | "setiap" | "di" | "selama" | "masih" | "ada" | "barang" | "ambil" | "berikutnya" | "pekerjaan" | "terima" | "hasilnya" | "ketika" | "pengguna" | "menekan" | "tombol" | "benar" | "salah" | "ditambah" | "dikurangi" | "dikali" | "dibagi" | "lebih" | "besar" | "kecil" | "sama" | "dengan")
}
