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
    AcceptTaskTitle,
    AddTask,
    DisplayAllTasks,
    AcceptTaskId,
    CompleteTask,
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
    if !width.is_multiple_of(4) {
        return Err(error(line_number, "H0002", "Indentasi harus kelipatan empat spasi"));
    }
    Ok(width)
}

fn is_name(value: &str) -> bool {
    if value.is_empty() || value != value.trim() {
        return false;
    }
    let mut previous_space = false;
    for ch in value.chars() {
        if ch == ' ' {
            if previous_space {
                return false;
            }
            previous_space = true;
        } else if ch.is_alphanumeric() || ch == '_' || ch == '-' {
            previous_space = false;
        } else {
            return false;
        }
    }
    !previous_space
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
/// - Pengguna bisa <nama> with supported display or task-list instructions
/// - Simpan semua <nama> agar tidak hilang saat aplikasi ditutup
///
/// This parser is experimental and is selected only through the explicit
/// human-vNext CLI command; the default v0.1 parser remains unchanged.
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
    let mut declaration_lines = Vec::new();
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
            let mut next_source_line = header_line + 1;
            while index < meaningful.len() && meaningful[index].1 >= 8 {
                let (text_line, text_indent, text_content) = meaningful[index];
                if text_indent < 8 || text_indent % 4 != 0 {
                    return Err(error(text_line, "H0002", "Indentasi isi teks tidak valid"));
                }
                // Blank source lines inside a text block are data. Reinsert them
                // between meaningful lines instead of dropping them during the
                // structural-line scan above.
                for blank_line in next_source_line..text_line {
                    if lines[blank_line - 1].trim().is_empty() {
                        text_lines.push(String::new());
                    }
                }
                let raw = lines[text_line - 1];
                text_lines.push(raw.get(8..).unwrap_or(text_content).to_owned());
                next_source_line = text_line + 1;
                index += 1;
            }
            // Preserve blank lines at the end of the text block, including a
            // final newline when the source itself ends immediately after data.
            let boundary_line = meaningful.get(index).map_or(lines.len() + 1, |entry| entry.0);
            // If there is no nonblank payload line, the block denotes the empty
            // string. A final source newline alone must not turn it into a
            // one-newline value.
            if !text_lines.is_empty() {
                for blank_line in next_source_line..boundary_line {
                    if lines[blank_line - 1].trim().is_empty() {
                        text_lines.push(String::new());
                    }
                }
            }
            declaration_lines.push(line_number);
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
                match body_text {
                    "Terima judul tugas" => body.push(HumanInstruction::AcceptTaskTitle),
                    "Simpan tugas baru" => body.push(HumanInstruction::AddTask),
                    "Tampilkan semua tugas" => body.push(HumanInstruction::DisplayAllTasks),
                    "Terima tugas yang dipilih" => body.push(HumanInstruction::AcceptTaskId),
                    "Tandai tugas sebagai selesai" => body.push(HumanInstruction::CompleteTask),
                    _ if body_text.starts_with("Tampilkan ") => {
                        let reference = body_text.strip_prefix("Tampilkan ").unwrap_or_default();
                        if !is_name(reference) {
                            return Err(error(body_line, "H0012", "Tampilkan membutuhkan nama nilai"));
                        }
                        body.push(HumanInstruction::Display { name: reference.to_owned() });
                    }
                    _ => return Err(error(body_line, "H0013", "Instruksi tindakan belum didukung parser prototipe")),
                }
                index += 1;
            }
            if body.is_empty() {
                return Err(error(line_number, "H0014", "Tindakan harus memiliki isi"));
            }
            declaration_lines.push(line_number);
            declarations.push(HumanDeclaration::Action { name: action_name, body });
            continue;
        }

        if let Some(collection) = text.strip_prefix("Simpan semua ")
            .and_then(|rest| rest.strip_suffix(" agar tidak hilang saat aplikasi ditutup"))
        {
            if !is_name(collection) {
                return Err(error(line_number, "H0015", "Nama koleksi penyimpanan tidak boleh kosong"));
            }
            declaration_lines.push(line_number);
            declarations.push(HumanDeclaration::Persist { collection: collection.to_owned() });
            index += 1;
            continue;
        }

        return Err(error(line_number, "H0016", "Bentuk instruksi tidak dikenal dalam parser human-vnext"));
    }

    let mut declaration_names = std::collections::HashSet::new();
    let mut data_names = std::collections::HashSet::new();
    let mut persisted_collections = std::collections::HashSet::new();
    for (declaration_index, declaration) in declarations.iter().enumerate() {
        let declaration_line = declaration_lines[declaration_index];
        if let HumanDeclaration::Persist { collection } = declaration
            && !persisted_collections.insert(collection.as_str())
        {
            return Err(error(declaration_line, "H0019", format!("Deklarasi penyimpanan ganda untuk {}", collection)));
        }
        let name = match declaration {
            HumanDeclaration::Data { name, .. } => {
                data_names.insert(name.as_str());
                Some(name)
            }
            HumanDeclaration::Action { name, .. } => Some(name),
            HumanDeclaration::Persist { .. } => None,
        };
        if let Some(name) = name
            && !declaration_names.insert(name.as_str())
        {
            return Err(error(declaration_line, "H0017", format!("Nama deklarasi ganda {}", name)));
        }
    }

    for (declaration_index, declaration) in declarations.iter().enumerate() {
        if let HumanDeclaration::Action { body, .. } = declaration {
            let declaration_line = declaration_lines[declaration_index];
            for instruction in body {
                match instruction {
                    HumanInstruction::Display { name } if !data_names.contains(name.as_str()) => {
                        return Err(error(declaration_line, "H0018", format!("Nilai {} belum dideklarasikan sebagai data", name)));
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
                value: HumanValue::Text("Halo dunia\nIni tanda kutip \" dan titik koma ;\n".into()),
            }]
        );
    }

    #[test]
    fn preserves_blank_lines_and_trailing_newline_in_text_blocks() {
        let source = "Buat aplikasi catatan\nBuat data pesan\n    Isi teks berikut\n        Baris pertama\n\n        Baris ketiga\n";
        let program = parse(source).expect("blank lines and trailing newline are text data");
        assert_eq!(
            program.declarations,
            vec![HumanDeclaration::Data {
                name: "pesan".into(),
                value: HumanValue::Text("Baris pertama\n\nBaris ketiga\n".into()),
            }]
        );
    }

    #[test]
    fn preserves_relative_indentation_and_accepts_crlf_text_blocks() {
        let source = "Buat aplikasi catatan\r\nBuat data pesan\r\n    Isi teks berikut\r\n        Baris pertama\r\n            Baris menjorok\r\n        Baris terakhir\r\n";
        let program = parse(source).expect("CRLF and relative indentation should be preserved");
        assert_eq!(
            program.declarations,
            vec![HumanDeclaration::Data {
                name: "pesan".into(),
                value: HumanValue::Text(
                    "Baris pertama\n    Baris menjorok\nBaris terakhir\n".into(),
                ),
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
    fn accepts_empty_text_block_without_inventing_a_newline() {
        let source = "Buat aplikasi catatan\nBuat data pesan\n    Isi teks berikut\n";
        let program = parse(source).expect("empty text is a valid text value");
        assert_eq!(
            program.declarations,
            vec![HumanDeclaration::Data {
                name: "pesan".into(),
                value: HumanValue::Text(String::new()),
            }]
        );
    }

    #[test]
    fn preserves_unicode_text_without_normalization() {
        let source = "Buat aplikasi catatan\nBuat data pesan\n    Isi teks berikut\n        Halo 世界 — selamat pagi 👋\n";
        let program = parse(source).expect("Unicode text should be accepted");
        assert_eq!(
            program.declarations,
            vec![HumanDeclaration::Data {
                name: "pesan".into(),
                value: HumanValue::Text("Halo 世界 — selamat pagi 👋\n".into()),
            }]
        );
    }

    #[test]
    fn rejects_duplicate_persistence_declarations() {
        let source = "Buat aplikasi catatan\nSimpan semua tugas agar tidak hilang saat aplikasi ditutup\nSimpan semua tugas agar tidak hilang saat aplikasi ditutup\n";
        let error = parse(source).expect_err("duplicate persistence declarations must fail");
        assert_eq!(error.code, "H0019");
    }

    #[test]
    fn rejects_noncanonical_identifier_spacing() {
        let source = "Buat aplikasi catatan\nBuat data dua  kata\n    Isi teks berikut\n        isi\n";
        let error = parse(source).expect_err("ambiguous repeated spaces in names must fail");
        assert_eq!(error.code, "H0007");
    }

    #[test]
    fn accepts_unicode_and_punctuated_identifier_characters() {
        let source = "Buat aplikasi café 東京\nBuat data naïve_名-2\n    Isi teks berikut\n        isi\n";
        let program = parse(source).expect("Unicode alphanumeric identifiers, underscore, and hyphen are supported");
        assert_eq!(program.project, "café 東京");
        assert!(matches!(
            &program.declarations[0],
            HumanDeclaration::Data { name, .. } if name == "naïve_名-2"
        ));
    }

    #[test]
    fn rejects_noncanonical_identifier_edges_and_punctuation() {
        for name in [" leading", "trailing ", "two  spaces", "nama!", "nama/tugas", "nama🙂"] {
            let source = format!("Buat aplikasi {name}\n");
            let error = parse(&source).expect_err("invalid identifier must be rejected");
            assert_eq!(error.code, "H0004", "identifier {name:?}");
        }

        for name in [" leading", "trailing ", "nama!", "nama/tugas"] {
            let source = format!(
                "Buat aplikasi catatan\nBuat data {name}\n    Isi teks berikut\n        isi\n"
            );
            let error = parse(&source).expect_err("invalid data identifier must be rejected");
            assert_eq!(error.code, "H0007", "identifier {name:?}");
        }
    }

    #[test]
    fn rejects_tabs_and_noncanonical_indentation() {
        for source in [
            "Buat aplikasi catatan\nPengguna bisa melihat\n\tTampilkan pesan\n",
            "Buat aplikasi catatan\n Pengguna bisa melihat\n",
        ] {
            assert!(parse(source).is_err(), "invalid indentation must be rejected");
        }
    }

    #[test]
    fn rejects_duplicate_data_and_action_names() {
        let source = "Buat aplikasi catatan\nBuat data pesan\n    Isi teks berikut\n        isi\nPengguna bisa pesan\n    Tampilkan pesan\n";
        let error = parse(source).expect_err("declarations share one duplicate-name namespace");
        assert_eq!(error.code, "H0017");
    }

    #[test]
    fn rejects_orphaned_nested_instruction() {
        let source = "Buat aplikasi catatan\n    Tampilkan pesan\n";
        let error = parse(source).expect_err("nested instruction without a parent must fail");
        assert_eq!(error.code, "H0006");
    }


    #[test]
    fn semantic_errors_report_the_relevant_declaration_line() {
        let duplicate = concat!(
            "Buat aplikasi catatan\n",
            "Buat data pesan\n",
            "    Isi teks berikut\n",
            "        isi\n",
            "Pengguna bisa pesan\n",
            "    Tampilkan pesan\n",
        );
        let error = parse(duplicate).expect_err("duplicate declaration name");
        assert_eq!(error.code, "H0017");
        assert_eq!(error.line, 5);

        let unresolved = concat!(
            "Buat aplikasi catatan\n",
            "Pengguna bisa melihat\n",
            "    Tampilkan belum ada\n",
        );
        let error = parse(unresolved).expect_err("unresolved data reference");
        assert_eq!(error.code, "H0018");
        assert_eq!(error.line, 2);

        let duplicate_persistence = concat!(
            "Buat aplikasi catatan\n",
            "Simpan semua tugas agar tidak hilang saat aplikasi ditutup\n",
            "Simpan semua tugas agar tidak hilang saat aplikasi ditutup\n",
        );
        let error = parse(duplicate_persistence).expect_err("duplicate persistence declaration");
        assert_eq!(error.code, "H0019");
        assert_eq!(error.line, 3);
    }

    #[test]
    fn rejects_missing_text_block() {
        let source = "Buat aplikasi catatan\nBuat data pesan\n";
        let error = parse(source).expect_err("missing text block must fail");
        assert_eq!(error.code, "H0008");
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

fn load_tasks(path: &std::path::Path) -> Result<Vec<Task>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents = std::fs::read_to_string(path)
        .map_err(|error| format!("Tidak dapat membaca penyimpanan '{}': {error}", path.display()))?;
    let mut tasks = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let mut fields = line.splitn(3, '\t');
        let id = fields.next().and_then(|value| value.parse::<u64>().ok())
            .ok_or_else(|| format!("Data tugas rusak pada baris {}: ID tidak valid", index + 1))?;
        if id == 0 {
            return Err(format!("Data tugas rusak pada baris {}: ID tidak valid", index + 1));
        }
        let status = fields.next()
            .ok_or_else(|| format!("Data tugas rusak pada baris {}: status hilang", index + 1))?;
        let title = fields.next()
            .ok_or_else(|| format!("Data tugas rusak pada baris {}: judul hilang", index + 1))?;
        if !matches!(status, "0" | "1") || title.is_empty() || title.chars().any(|ch| ch.is_control()) {
            return Err(format!("Data tugas rusak pada baris {}: field tidak valid", index + 1));
        }
        if tasks.iter().any(|task: &Task| task.id == id) {
            return Err(format!("Data tugas rusak pada baris {}: ID duplikat", index + 1));
        }
        tasks.push(Task { id, title: title.to_owned(), done: status == "1" });
    }
    Ok(tasks)
}

#[cfg(windows)]
fn replace_file(temporary: &std::path::Path, destination: &std::path::Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        #[allow(non_snake_case)]
        fn MoveFileExW(existing_file_name: *const u16, new_file_name: *const u16, flags: u32) -> i32;
    }

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x1;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x8;
    let from: Vec<u16> = temporary.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = destination.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: both paths are NUL-terminated UTF-16 buffers that remain alive for
    // the call; MoveFileExW does not retain either pointer.
    let result = unsafe {
        MoveFileExW(
            from.as_ptr(),
            to.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(not(windows))]
fn replace_file(temporary: &std::path::Path, destination: &std::path::Path) -> std::io::Result<()> {
    std::fs::rename(temporary, destination)
}

static NEXT_STORAGE_TEMP_ID: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

fn save_tasks(path: &std::path::Path, tasks: &[Task]) -> Result<(), String> {
    use std::io::Write as _;

    if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("Tidak dapat membuat direktori penyimpanan: {error}"))?;
    }
    let mut contents = String::new();
    for task in tasks {
        if task.title.is_empty() || task.title.chars().any(|ch| ch.is_control()) {
            return Err("Judul tugas tidak boleh kosong atau mengandung karakter kontrol".into());
        }
        contents.push_str(&format!("{}\t{}\t{}\n", task.id, if task.done { 1 } else { 0 }, task.title));
    }
    let mut temporary_name = path.as_os_str().to_os_string();
    temporary_name.push(format!(
        ".tmp.{}.{}",
        std::process::id(),
        NEXT_STORAGE_TEMP_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
    ));
    let temporary = std::path::PathBuf::from(temporary_name);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| format!("Tidak dapat membuat data sementara: {error}"))?;
    if let Err(error) = file.write_all(contents.as_bytes()).and_then(|()| file.sync_all()) {
        drop(file);
        let _ = std::fs::remove_file(&temporary);
        return Err(format!("Tidak dapat menulis data sementara: {error}"));
    }
    drop(file);
    if let Err(error) = replace_file(&temporary, path) {
        let _ = std::fs::remove_file(&temporary);
        return Err(format!("Tidak dapat menyimpan data tugas secara atomik: {error}"));
    }
    Ok(())
}

fn validate_task_app(program: &HumanProgram) -> Result<(), String> {
    let mut persistence = false;
    let mut add = false;
    let mut list = false;
    let mut complete = false;

    for declaration in &program.declarations {
        match declaration {
            HumanDeclaration::Persist { collection } if collection == "tugas" && !persistence => {
                persistence = true;
            }
            HumanDeclaration::Persist { collection } if collection == "tugas" => {
                return Err("Deklarasi penyimpanan tugas tidak boleh berulang".into());
            }
            HumanDeclaration::Persist { collection } => {
                return Err(format!("Koleksi penyimpanan '{}' belum didukung runner daftar tugas", collection));
            }
            HumanDeclaration::Data { name, .. } => {
                return Err(format!("Data '{}' belum didukung runner daftar tugas", name));
            }
            HumanDeclaration::Action { name, body } => match (name.as_str(), body.as_slice()) {
                ("menambahkan tugas", [HumanInstruction::AcceptTaskTitle, HumanInstruction::AddTask]) if !add => {
                    add = true;
                }
                ("melihat semua tugas", [HumanInstruction::DisplayAllTasks]) if !list => {
                    list = true;
                }
                ("menandai tugas sebagai selesai", [HumanInstruction::AcceptTaskId, HumanInstruction::CompleteTask]) if !complete => {
                    complete = true;
                }
                _ => {
                    return Err(format!("Aksi '{}' atau urutan instruksinya belum didukung runner daftar tugas", name));
                }
            },
        }
    }

    if !persistence {
        return Err("Aplikasi daftar tugas wajib mendeklarasikan: Simpan semua tugas agar tidak hilang saat aplikasi ditutup".into());
    }
    if !add || !list || !complete {
        return Err("Aplikasi daftar tugas membutuhkan aksi tambah, lihat semua, dan tandai selesai dengan nama serta urutan instruksi yang tepat".into());
    }
    Ok(())
}

/// Run the explicitly experimental human-vNext task-list vertical slice.
/// The existing v0.1 compiler and CLI path remain untouched.
pub fn run_task_app<R: std::io::BufRead, W: std::io::Write>(
    program: &HumanProgram,
    storage_path: &std::path::Path,
    input: &mut R,
    output: &mut W,
) -> Result<(), String> {
    validate_task_app(program)?;
    let mut tasks = load_tasks(storage_path)?;
    writeln!(output, "ONE human-vNext — {}", program.project).map_err(|error| error.to_string())?;
    writeln!(output, "Perintah: tambah, lihat, selesai, keluar").map_err(|error| error.to_string())?;
    loop {
        write!(output, "> ").map_err(|error| error.to_string())?;
        output.flush().map_err(|error| error.to_string())?;
        let mut command = String::new();
        if input.read_line(&mut command).map_err(|error| format!("Gagal membaca perintah: {error}"))? == 0 {
            break;
        }
        match command.trim() {
            "tambah" => {
                write!(output, "Judul tugas: ").map_err(|error| error.to_string())?;
                output.flush().map_err(|error| error.to_string())?;
                let mut title = String::new();
                if input.read_line(&mut title).map_err(|error| format!("Gagal membaca judul: {error}"))? == 0 {
                    break;
                }
                let title = title.trim_end_matches(['\r', '\n']);
                if title.trim().is_empty() || title.chars().any(char::is_control) {
                    writeln!(output, "Kesalahan: judul tidak boleh kosong atau mengandung karakter kontrol").map_err(|error| error.to_string())?;
                    continue;
                }
                let id = tasks.iter().map(|task| task.id).max().unwrap_or(0).checked_add(1)
                    .ok_or_else(|| "ID tugas habis".to_owned())?;
                tasks.push(Task { id, title: title.to_owned(), done: false });
                save_tasks(storage_path, &tasks)?;
                writeln!(output, "Tugas {id} tersimpan.").map_err(|error| error.to_string())?;
            }
            "lihat" => {
                if tasks.is_empty() {
                    writeln!(output, "Belum ada tugas.").map_err(|error| error.to_string())?;
                }
                for task in &tasks {
                    writeln!(output, "{} [{}] {}", task.id, if task.done { "selesai" } else { "belum selesai" }, task.title)
                        .map_err(|error| error.to_string())?;
                }
            }
            "selesai" => {
                write!(output, "ID tugas: ").map_err(|error| error.to_string())?;
                output.flush().map_err(|error| error.to_string())?;
                let mut raw_id = String::new();
                if input.read_line(&mut raw_id).map_err(|error| format!("Gagal membaca ID: {error}"))? == 0 {
                    break;
                }
                let id = match raw_id.trim().parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => {
                        writeln!(output, "Kesalahan: ID harus berupa angka positif").map_err(|error| error.to_string())?;
                        continue;
                    }
                };
                match tasks.iter_mut().find(|task| task.id == id) {
                    Some(task) => {
                        task.done = true;
                        save_tasks(storage_path, &tasks)?;
                        writeln!(output, "Tugas {id} ditandai selesai.").map_err(|error| error.to_string())?;
                    }
                    None => writeln!(output, "Kesalahan: tugas {id} tidak ditemukan").map_err(|error| error.to_string())?,
                }
            }
            "keluar" => break,
            "" => {}
            other => writeln!(output, "Kesalahan: perintah '{}' tidak dikenal", other).map_err(|error| error.to_string())?,
        }
    }
    Ok(())
}


#[cfg(test)]
mod task_runtime_tests {
    use super::*;
    use std::io::Cursor;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    fn storage_path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "one-human-vnext-tasks-{}-{}.db",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ))
    }

    fn task_program() -> HumanProgram {
        parse(concat!(
            "Buat aplikasi daftar tugas\n",
            "Pengguna bisa menambahkan tugas\n",
            "    Terima judul tugas\n",
            "    Simpan tugas baru\n",
            "Pengguna bisa melihat semua tugas\n",
            "    Tampilkan semua tugas\n",
            "Pengguna bisa menandai tugas sebagai selesai\n",
            "    Terima tugas yang dipilih\n",
            "    Tandai tugas sebagai selesai\n",
            "Simpan semua tugas agar tidak hilang saat aplikasi ditutup\n",
        ))
        .expect("task app source should parse")
    }
    #[test]
    fn task_app_persists_across_sessions_and_marks_completion() {
        let path = storage_path();
        let program = task_program();
        let mut first_input = Cursor::new(b"tambah\nBelanja kebutuhan\nkeluar\n".to_vec());
        let mut first_output = Vec::new();
        run_task_app(&program, &path, &mut first_input, &mut first_output)
            .expect("adding a task should succeed");

        let mut second_input = Cursor::new(b"lihat\nselesai\n1\nlihat\nkeluar\n".to_vec());
        let mut second_output = Vec::new();
        run_task_app(&program, &path, &mut second_input, &mut second_output)
            .expect("reopened task app should read and update saved data");
        let output = String::from_utf8(second_output).expect("output is UTF-8");
        assert!(output.contains("Belanja kebutuhan"), "task must survive restart");
        assert!(output.contains("[selesai] Belanja kebutuhan"), "completion must persist");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn task_app_rejects_missing_persistence_declaration() {
        let mut program = task_program();
        program.declarations.retain(|declaration| !matches!(declaration, HumanDeclaration::Persist { .. }));
        let path = storage_path();
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();
        let error = run_task_app(&program, &path, &mut input, &mut output)
            .expect_err("durable storage must be declared");
        assert!(error.contains("wajib mendeklarasikan"));
        assert!(!path.exists());
    }

    #[test]
    fn task_app_rejects_actions_the_runner_cannot_execute() {
        let mut program = task_program();
        program.declarations.push(HumanDeclaration::Action {
            name: "aksi tambahan".into(),
            body: vec![HumanInstruction::Display { name: "pesan".into() }],
        });
        let path = storage_path();
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();
        let error = run_task_app(&program, &path, &mut input, &mut output)
            .expect_err("unsupported action must not be silently ignored");
        assert!(error.contains("belum didukung runner daftar tugas"));
        assert!(!path.exists());
    }

    #[test]
    fn task_app_rejects_data_the_runner_cannot_execute() {
        let mut program = task_program();
        program.declarations.push(HumanDeclaration::Data {
            name: "pesan".into(),
            value: HumanValue::Text("halo".into()),
        });
        let path = storage_path();
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();
        let error = run_task_app(&program, &path, &mut input, &mut output)
            .expect_err("unexecuted data must not be silently accepted");
        assert!(error.contains("belum didukung runner daftar tugas"));
        assert!(!path.exists());
    }

    #[test]
    fn task_app_rejects_unhandled_persistence_collections() {
        let mut program = task_program();
        program.declarations.push(HumanDeclaration::Persist { collection: "catatan".into() });
        let path = storage_path();
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();
        let error = run_task_app(&program, &path, &mut input, &mut output)
            .expect_err("unsupported persistence collection must be rejected");
        assert!(error.contains("belum didukung runner daftar tugas"));
        assert!(!path.exists());
    }

    #[test]
    fn task_storage_replaces_existing_records_on_repeated_saves() {
        let path = storage_path();
        save_tasks(&path, &[Task { id: 1, title: "lama".into(), done: false }])
            .expect("initial save should succeed");
        save_tasks(&path, &[Task { id: 1, title: "baru".into(), done: true }])
            .expect("replacement save should succeed on every supported platform");
        assert_eq!(
            load_tasks(&path).expect("replacement storage should load"),
            vec![Task { id: 1, title: "baru".into(), done: true }],
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn task_storage_rejects_corrupt_records() {
        let path = storage_path();
        for (contents, expected) in [
            ("not-an-id\t0\tjudul\n", "ID tidak valid"),
            ("0\t0\tjudul\n", "ID tidak valid"),
            ("1\n", "status hilang"),
            ("1\t0\n", "judul hilang"),
            ("1\t2\tjudul\n", "field tidak valid"),
            ("1\t0\t\n", "field tidak valid"),
            ("1\t0\tjudul\u{1}\n", "field tidak valid"),
            ("1\t0\tjudul\n1\t1\tduplikat\n", "ID duplikat"),
        ] {
            std::fs::write(&path, contents).expect("write corrupt fixture");
            let error = load_tasks(&path).expect_err("corrupt record must fail");
            assert!(error.contains(expected), "expected {expected:?} in {error:?}");
        }
        let _ = std::fs::remove_file(path);
    }
    
    #[test]
    fn task_app_rejects_blank_title_without_persisting_it() {
        let path = storage_path();
        let mut input = Cursor::new(b"tambah\n   \nkeluar\n".to_vec());
        let mut output = Vec::new();
        run_task_app(&task_program(), &path, &mut input, &mut output)
            .expect("invalid title is reported without aborting the session");
        assert!(String::from_utf8(output).unwrap().contains("judul tidak boleh kosong"));
        assert!(!path.exists(), "invalid title must not create storage");
    }

    #[test]
    fn task_app_rejects_non_numeric_and_unknown_task_ids() {
        let path = storage_path();
        let mut input = Cursor::new(b"selesai\nbukan-angka\nselesai\n999\nkeluar\n".to_vec());
        let mut output = Vec::new();
        run_task_app(&task_program(), &path, &mut input, &mut output)
            .expect("invalid and unknown IDs are reported without aborting the session");
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("ID harus berupa angka positif"));
        assert!(output.contains("tugas 999 tidak ditemukan"));
        assert!(!path.exists(), "unknown IDs must not create storage");
    }

    #[test]
    fn task_app_rejects_non_tab_control_characters_in_titles() {
        let path = storage_path();
        let mut input = Cursor::new(b"tambah\njudul\x0bberbahaya\nkeluar\n".to_vec());
        let mut output = Vec::new();
        run_task_app(&task_program(), &path, &mut input, &mut output)
            .expect("invalid title is reported without aborting the session");
        assert!(
            String::from_utf8(output).unwrap().contains("judul tidak boleh kosong atau mengandung karakter kontrol")
        );
        assert!(!path.exists(), "control characters must never reach the storage format");
    }

    #[test]
    fn task_storage_reports_parent_path_write_failure() {
        let root = storage_path();
        std::fs::write(&root, "file where a directory is required").unwrap();
        let path = root.join("tasks.db");
        let error = save_tasks(&path, &[]).expect_err("a file cannot be used as a parent directory");
        assert!(error.contains("Tidak dapat membuat direktori penyimpanan"));
        let _ = std::fs::remove_file(root);
    }
}


#[cfg(test)]
mod grammar_fixture_tests {
    use super::parse;

    #[test]
    fn valid_human_vnext_fixtures_parse() {
        for (name, source) in [
            ("minimal-data", include_str!("../../../tests/human-vnext/valid/minimal-data.one")),
            ("unicode-name", include_str!("../../../tests/human-vnext/valid/unicode-name.one")),
            ("task-list", include_str!("../../../tests/human-vnext/valid/task-list.one")),
            ("text-block-boundaries", include_str!("../../../tests/human-vnext/valid/text-block-boundaries.one")),
            ("instruction-like-text", include_str!("../../../tests/human-vnext/valid/instruction-like-text.one")),
        ] {
            parse(source).unwrap_or_else(|error| panic!("valid fixture {name} rejected: {error}"));
        }
    }

    #[test]
    fn instruction_like_lines_remain_opaque_text_data() {
        let source = include_str!("../../../tests/human-vnext/valid/instruction-like-text.one");
        let program = parse(source).expect("instruction-like lines inside text blocks are data");
        assert_eq!(
            program.declarations,
            vec![super::HumanDeclaration::Data {
                name: "instruksi".into(),
                value: super::HumanValue::Text(
                    "Pengguna bisa menghapus data\nTampilkan rahasia\nSimpan semua data agar tidak hilang saat aplikasi ditutup\n".into()
                ),
            }],
        );
    }

    #[test]
    fn invalid_human_vnext_fixtures_are_rejected() {
        for (name, source, expected_code, expected_line) in [
            ("empty", include_str!("../../../tests/human-vnext/invalid/empty.one"), "H0003", 1),
            ("orphan-instruction", include_str!("../../../tests/human-vnext/invalid/orphan-instruction.one"), "H0005", 1),
            ("unknown-instruction", include_str!("../../../tests/human-vnext/invalid/unknown-instruction.one"), "H0013", 3),
            ("unresolved-display", include_str!("../../../tests/human-vnext/invalid/unresolved-display.one"), "H0018", 2),
            ("indented-application", include_str!("../../../tests/human-vnext/invalid/indented-application.one"), "H0005", 1),
            ("missing-text-block", include_str!("../../../tests/human-vnext/invalid/missing-text-block.one"), "H0008", 2),
            ("duplicate-data", include_str!("../../../tests/human-vnext/invalid/duplicate-data.one"), "H0017", 5),
            ("duplicate-persistence", include_str!("../../../tests/human-vnext/invalid/duplicate-persistence.one"), "H0019", 3),
            ("action-name-as-data", include_str!("../../../tests/human-vnext/invalid/action-name-as-data.one"), "H0018", 2),
            ("repeated-spaces", include_str!("../../../tests/human-vnext/invalid/repeated-spaces.one"), "H0007", 2),
            ("duplicate-data-action-name", include_str!("../../../tests/human-vnext/invalid/duplicate-data-action-name.one"), "H0017", 5),
            ("second-application", include_str!("../../../tests/human-vnext/invalid/second-application.one"), "H0016", 2),
            ("empty-action", include_str!("../../../tests/human-vnext/invalid/empty-action.one"), "H0014", 2),
            ("tab-indentation", include_str!("../../../tests/human-vnext/invalid/tab-indentation.one"), "H0001", 3),
            ("noncanonical-indent", include_str!("../../../tests/human-vnext/invalid/noncanonical-indent.one"), "H0002", 2),
            ("invalid-action-name", include_str!("../../../tests/human-vnext/invalid/invalid-action-name.one"), "H0010", 2),
        ] {
            let error = parse(source).expect_err(&format!("invalid fixture {name} was accepted"));
            assert_eq!(error.code, expected_code, "fixture {name} diagnostic code changed");
            assert_eq!(error.line, expected_line, "fixture {name} diagnostic line changed");
        }
    }
}
