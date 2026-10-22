//! Experimental deterministic parser for the proposed ONE human-instruction surface.
//!
//! This module is isolated from the normative v0.1 lexer and parser. It does not
//! execute programs and must not be advertised as a complete vNext implementation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanProgram {
    pub project: String,
    pub declarations: Vec<HumanDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumanDeclaration {
    Data { name: String, value: HumanValue },
    Action { name: String, body: Vec<HumanInstruction> },
    Persist { collection: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumanValue {
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumanInstruction {
    Display { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanParseError {
    pub line: usize,
    pub code: &'static str,
    pub message: String,
}

impl std::fmt::Display for HumanParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "baris {} [{}] {}", self.line, self.code, self.message)
    }
}

impl std::error::Error for HumanParseError {}

fn error(line: usize, code: &'static str, message: impl Into<String>) -> HumanParseError {
    HumanParseError { line, code, message: message.into() }
}

fn trim_line_ending(line: &str) -> &str {
    line.strip_suffix('\r').unwrap_or(line)
}

fn leading_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}

fn validate_indentation(line: &str, line_number: usize) -> Result<usize, HumanParseError> {
    let width = leading_spaces(line);
    if line.as_bytes().get(width) == Some(&b'\t') {
        return Err(error(line_number, "H0001", "Indentasi tidak boleh memakai tab"));
    }
    if width % 4 != 0 {
        return Err(error(line_number, "H0002", "Indentasi harus kelipatan empat spasi"));
    }
    Ok(width)
}

fn is_name(value: &str) -> bool {
    !value.trim().is_empty()
        && value == value.trim()
        && !value.chars().any(|ch| ch.is_control())
}

fn parse_project(line: &str, line_number: usize) -> Result<String, HumanParseError> {
    let name = line.strip_prefix("Buat aplikasi ")
        .ok_or_else(|| error(line_number, "H0003", "Instruksi pertama harus menyatakan nama aplikasi"))?;
    if !is_name(name) {
        return Err(error(line_number, "H0004", "Nama aplikasi tidak boleh kosong"));
    }
    Ok(name.to_owned())
}

/// Parse the deliberately small prototype grammar.
///
/// Supported top-level forms:
/// - Buat aplikasi <nama>
/// - Buat data <nama> followed by an indented Isi teks berikut block
/// - Pengguna bisa <nama> followed by an indented body containing Tampilkan <nama>
/// - Simpan semua <nama> agar tidak hilang saat aplikasi ditutup
///
/// This parser is experimental and is not wired into the default v0.1 CLI.
pub fn parse(source: &str) -> Result<HumanProgram, HumanParseError> {
    let lines: Vec<&str> = source.split('\n').map(trim_line_ending).collect();
    let mut meaningful: Vec<(usize, usize, &str)> = Vec::new();
    for (index, raw) in lines.iter().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        let line_number = index + 1;
        let indent = validate_indentation(raw, line_number)?;
        meaningful.push((line_number, indent, &raw[indent..]));
    }

    let Some((first_line, first_indent, first_text)) = meaningful.first().copied() else {
        return Err(error(1, "H0003", "Program kosong"));
    };
    if first_indent != 0 {
        return Err(error(first_line, "H0005", "Deklarasi aplikasi harus dimulai dari kolom pertama"));
    }
    let project = parse_project(first_text, first_line)?;
    let mut declarations = Vec::new();
    let mut index = 1;

    while index < meaningful.len() {
        let (line_number, indent, text) = meaningful[index];
        if indent != 0 {
            return Err(error(line_number, "H0006", "Instruksi ini tidak memiliki blok induk"));
        }

        if let Some(rest) = text.strip_prefix("Buat data ") {
            if !is_name(rest) {
                return Err(error(line_number, "H0007", "Nama data tidak boleh kosong"));
            }
            let name = rest.to_owned();
            index += 1;
            let Some((header_line, header_indent, header_text)) = meaningful.get(index).copied() else {
                return Err(error(line_number, "H0008", "Data membutuhkan blok Isi teks berikut"));
            };
            if header_indent != 4 || header_text != "Isi teks berikut" {
                return Err(error(header_line, "H0008", "Data membutuhkan blok Isi teks berikut pada indentasi empat spasi"));
            }
            index += 1;
            let mut text_lines = Vec::new();
            while index < meaningful.len() && meaningful[index].1 >= 8 {
                let (text_line, text_indent, text_content) = meaningful[index];
                if text_indent < 8 || text_indent % 4 != 0 {
                    return Err(error(text_line, "H0002", "Indentasi isi teks tidak valid"));
                }
                let raw = lines[text_line - 1];
                let content_start = text_indent;
                text_lines.push(raw.get(content_start..).unwrap_or(text_content).to_owned());
                index += 1;
            }
            if text_lines.is_empty() {
                return Err(error(header_line, "H0009", "Blok teks harus memiliki setidaknya satu baris"));
            }
            declarations.push(HumanDeclaration::Data {
                name,
                value: HumanValue::Text(text_lines.join("\n")),
            });
            continue;
        }

        if let Some(name) = text.strip_prefix("Pengguna bisa ") {
            if !is_name(name) {
                return Err(error(line_number, "H0010", "Nama tindakan tidak boleh kosong"));
            }
            let action_name = name.to_owned();
            index += 1;
            let mut body = Vec::new();
            while index < meaningful.len() && meaningful[index].1 >= 4 {
                let (body_line, body_indent, body_text) = meaningful[index];
                if body_indent != 4 {
                    return Err(error(body_line, "H0011", "Isi tindakan harus menggunakan satu tingkat indentasi"));
                }
                if let Some(reference) = body_text.strip_prefix("Tampilkan ") {
                    if !is_name(reference) {
                        return Err(error(body_line, "H0012", "Tampilkan membutuhkan nama nilai"));
                    }
                    body.push(HumanInstruction::Display { name: reference.to_owned() });
                } else {
                    return Err(error(body_line, "H0013", "Instruksi tindakan belum didukung parser prototipe"));
                }
                index += 1;
            }
            if body.is_empty() {
                return Err(error(line_number, "H0014", "Tindakan harus memiliki isi"));
            }
            declarations.push(HumanDeclaration::Action { name: action_name, body });
            continue;
        }

        if let Some(collection) = text.strip_prefix("Simpan semua ")
            .and_then(|rest| rest.strip_suffix(" agar tidak hilang saat aplikasi ditutup"))
        {
            if !is_name(collection) {
                return Err(error(line_number, "H0015", "Nama koleksi penyimpanan tidak boleh kosong"));
            }
            declarations.push(HumanDeclaration::Persist { collection: collection.to_owned() });
            index += 1;
            continue;
        }

        return Err(error(line_number, "H0016", "Bentuk instruksi tidak dikenal dalam parser human-vnext"));
    }

    let mut declaration_names = std::collections::HashSet::new();
    let mut data_names = std::collections::HashSet::new();
    for declaration in &declarations {
        let name = match declaration {
            HumanDeclaration::Data { name, .. } => {
                data_names.insert(name.as_str());
                Some(name)
            }
            HumanDeclaration::Action { name, .. } => Some(name),
            HumanDeclaration::Persist { .. } => None,
        };
        if let Some(name) = name {
            if !declaration_names.insert(name.as_str()) {
                return Err(error(1, "H0017", format!("Nama deklarasi ganda {}", name)));
            }
        }
    }

    for declaration in &declarations {
        if let HumanDeclaration::Action { body, .. } = declaration {
            for instruction in body {
                match instruction {
                    HumanInstruction::Display { name } if !data_names.contains(name.as_str()) => {
                        return Err(error(1, "H0018", format!("Nilai {} belum dideklarasikan sebagai data", name)));
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(HumanProgram { project, declarations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_project_and_unquoted_multiline_text() {
        let source = "Buat aplikasi catatan\n\nBuat data pesan\n    Isi teks berikut\n        Halo dunia\n        Ini tanda kutip \" dan titik koma ;\n";
        let program = parse(source).expect("prototype source should parse");
        assert_eq!(program.project, "catatan");
        assert_eq!(
            program.declarations,
            vec![HumanDeclaration::Data {
                name: "pesan".into(),
                value: HumanValue::Text("Halo dunia\nIni tanda kutip \" dan titik koma ;".into()),
            }]
        );
    }

    #[test]
    fn parses_action_and_display_reference() {
        let source = "Buat aplikasi daftar tugas\nBuat data judul\n    Isi teks berikut\n        Belanja\nPengguna bisa melihat tugas\n    Tampilkan judul\n";
        let program = parse(source).expect("supported action should parse");
        assert_eq!(program.declarations.len(), 2);
        assert_eq!(
            program.declarations[1],
            HumanDeclaration::Action {
                name: "melihat tugas".into(),
                body: vec![HumanInstruction::Display { name: "judul".into() }],
            }
        );
    }

    #[test]
    fn rejects_display_reference_to_action_name() {
        let source = "Buat aplikasi catatan\nPengguna bisa melihat catatan\n    Tampilkan melihat catatan\n";
        let error = parse(source).expect_err("an action name is not a data value");
        assert_eq!(error.code, "H0018");
    }

    #[test]
    fn rejects_unknown_instruction_without_ai_guessing() {
        let source = "Buat aplikasi catatan\nTolong lakukan sesuatu\n";
        let error = parse(source).expect_err("unknown instruction must fail");
        assert_eq!(error.code, "H0016");
    }

    #[test]
    fn rejects_missing_text_block() {
        let source = "Buat aplikasi catatan\nBuat data pesan\n";
        let error = parse(source).expect_err("missing text block must fail");
        assert_eq!(error.code, "H0008");
    }
}
