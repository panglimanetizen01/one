//! Human-instruction vNext frontend adapter to ONE's canonical semantic pipeline.
//!
//! The first line declares the program identity. The body uses the accepted Indonesian
//! instruction grammar and is compiled by the same parser, semantic analyzer, canonical
//! IR lowering and reference interpreter as v0.1. This adapter deliberately does not
//! implement a second evaluator or reinterpret expressions at runtime.
use crate::{Diagnostic, DiagnosticBundle, IrProgram};

#[cfg(any(target_os = "linux", target_os = "android"))]
fn read_confined_module(
    root_dir: &std::path::Path,
    current_file: &std::path::Path,
    relative_path: &std::path::Path,
    _candidate: &std::path::Path,
) -> Result<String, String> {
    use std::ffi::CString;
    use std::fs::File;
    use std::io::Read;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;

    let root_path = CString::new(root_dir.as_os_str().as_bytes())
        .map_err(|_| "Direktori proyek memiliki byte NUL.".to_owned())?;
    let root_fd = unsafe {
        libc::open(
            root_path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if root_fd < 0 {
        return Err(format!("Tidak dapat membuka direktori proyek dengan aman: {}", std::io::Error::last_os_error()));
    }
    // SAFETY: root_fd is a newly opened descriptor and ownership moves into File.
    let mut current = unsafe { File::from_raw_fd(root_fd) };

    let base = current_file
        .parent()
        .unwrap_or(root_dir)
        .strip_prefix(root_dir)
        .map_err(|_| "Modul berada di luar direktori proyek.".to_owned())?;
    let mut components = Vec::new();
    for component in base.components().chain(relative_path.components()) {
        match component {
            std::path::Component::Normal(value) => components.push(value.to_os_string()),
            std::path::Component::CurDir => {}
            _ => return Err("Jalur modul harus tetap berada di dalam direktori proyek.".to_owned()),
        }
    }
    if components.is_empty() {
        return Err("Jalur modul kosong.".to_owned());
    }

    for (index, component) in components.iter().enumerate() {
        let name = CString::new(component.as_os_str().as_bytes())
            .map_err(|_| "Nama modul memiliki byte NUL.".to_owned())?;
        let final_component = index + 1 == components.len();
        let mut flags = libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK;
        if !final_component {
            flags |= libc::O_DIRECTORY;
        }
        let fd = unsafe { libc::openat(current.as_raw_fd(), name.as_ptr(), flags) };
        if fd < 0 {
            return Err(format!("Tidak dapat membuka komponen modul '{}' tanpa mengikuti symlink: {}", component.to_string_lossy(), std::io::Error::last_os_error()));
        }
        // SAFETY: fd is a newly opened descriptor and ownership moves into File.
        let opened = unsafe { File::from_raw_fd(fd) };
        if final_component {
            let metadata = opened.metadata().map_err(|error| format!("Tidak dapat memeriksa modul: {error}"))?;
            if !metadata.is_file() {
                return Err("Modul harus berupa berkas reguler.".to_owned());
            }
            let mut source = String::new();
            opened.take(16 * 1024 * 1024 + 1)
                .read_to_string(&mut source)
                .map_err(|error| format!("Tidak dapat membaca modul: {error}"))?;
            if source.len() > 16 * 1024 * 1024 {
                return Err("Ukuran modul melampaui batas 16 MiB.".to_owned());
            }
            return Ok(source);
        }
        current = opened;
    }
    Err("Jalur modul tidak valid.".to_owned())
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn read_confined_module(
    root_dir: &std::path::Path,
    _current_file: &std::path::Path,
    _relative_path: &std::path::Path,
    candidate: &std::path::Path,
) -> Result<String, String> {
    let canonical = std::fs::canonicalize(candidate).map_err(|error| error.to_string())?;
    if !canonical.starts_with(root_dir) {
        return Err("Modul berada di luar direktori proyek.".to_owned());
    }
    std::fs::read_to_string(canonical).map_err(|error| error.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanSource {
    pub name: String,
    pub body: String,
    pub body_line_offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledHumanProgram {
    pub name: String,
    pub ir: IrProgram,
    /// Canonical root and relative module files in deterministic traversal order.
    pub modules: Vec<std::path::PathBuf>,
}

pub fn parse(source: &str) -> Result<HumanSource, DiagnosticBundle> {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = normalized.split('\n');
    let first = lines.next().unwrap_or_default();
    let Some(name) = first.strip_prefix("Buat program ") else {
        return Err(DiagnosticBundle::single(Diagnostic::error(
            "HV1001",
            "Baris pertama harus berbentuk 'Buat program <nama>'.",
            Some(crate::token::Span::new(0, first.len(), 1, 1)),
        )));
    };
    let name = name.trim();
    if name.is_empty()
        || name.chars().any(|ch| !(ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == ' '))
        || name.contains("  ")
    {
        return Err(DiagnosticBundle::single(Diagnostic::error(
            "HV1002",
            "Nama program kosong atau mengandung karakter yang tidak didukung.",
            Some(crate::token::Span::new(13, first.len(), 1, 14)),
        )));
    }
    let body = lines.collect::<Vec<_>>().join("\n");
    if body.lines().all(|line| line.trim().is_empty()) {
        return Err(DiagnosticBundle::single(Diagnostic::error(
            "HV1003",
            "Program membutuhkan sedikitnya satu instruksi setelah deklarasi.",
            Some(crate::token::Span::new(first.len(), first.len(), 2, 1)),
        )));
    }
    Ok(HumanSource { name: name.to_owned(), body, body_line_offset: 1 })
}

/// Compile a human-declared program through the canonical parser, semantic analyzer
/// and IR lowering. There is intentionally no vNext-only execution semantics here.
pub fn compile(source: &str) -> Result<CompiledHumanProgram, DiagnosticBundle> {
    let parsed = parse(source)?;
    let ast = crate::parse_human_core(&parsed.body).map_err(|bundle| shift_diagnostics(bundle, parsed.body_line_offset))?;
    let typed = crate::analyze(&ast).map_err(|bundle| shift_diagnostics(bundle, parsed.body_line_offset))?;
    let ir = crate::lower(&typed).map_err(|bundle| shift_diagnostics(bundle, parsed.body_line_offset))?;
    Ok(CompiledHumanProgram { name: parsed.name, ir, modules: Vec::new() })
}

fn shift_diagnostics(mut bundle: DiagnosticBundle, offset: usize) -> DiagnosticBundle {
    for diagnostic in &mut bundle.0 {
        if let Some(span) = &mut diagnostic.span {
            span.line += offset;
        }
    }
    bundle
}

/// Compile a multi-file human project. Imports are explicit, relative .one paths
/// declared immediately after the root/module header; path traversal and import cycles
/// are rejected. All module bodies enter the same canonical semantic pipeline.
pub fn compile_file(path: &std::path::Path) -> Result<CompiledHumanProgram, DiagnosticBundle> {
    let root_source = std::fs::read_to_string(path).map_err(|error| {
        DiagnosticBundle::single(Diagnostic::error(
            "HV1010",
            format!("Tidak dapat membaca '{}': {}", path.display(), error),
            None,
        ))
    })?;
    let root_header = root_source.lines().next().unwrap_or_default();
    let root = parse(&format!("{}\ntampilkan \"header probe\"\n", root_header))?;
    let canonical_root = std::fs::canonicalize(path).map_err(|error| {
        DiagnosticBundle::single(Diagnostic::error("HV1011", format!("Tidak dapat menyelesaikan jalur proyek '{}': {}", path.display(), error), None))
    })?;
    let root_dir = canonical_root.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
    let mut collection = ModuleCollection::new(root_dir);
    collection.collect(&canonical_root, &root_source, true)?;
    let wrapped = format!("Buat program {}\n{}", root.name, collection.output);
    let mut compiled = compile(&wrapped)?;
    compiled.modules = collection.modules;
    Ok(compiled)
}

struct ModuleCollection {
    root_dir: std::path::PathBuf,
    seen: std::collections::HashSet<std::path::PathBuf>,
    stack: std::collections::HashSet<std::path::PathBuf>,
    modules: Vec<std::path::PathBuf>,
    output: String,
}

impl ModuleCollection {
    fn new(root_dir: std::path::PathBuf) -> Self {
        Self {
            root_dir,
            seen: std::collections::HashSet::new(),
            stack: std::collections::HashSet::new(),
            modules: Vec::new(),
            output: String::new(),
        }
    }

    fn collect(
        &mut self,
        path: &std::path::Path,
        source: &str,
        is_root: bool,
    ) -> Result<(), DiagnosticBundle> {
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            DiagnosticBundle::single(Diagnostic::error(
                "HV1011", format!("Tidak dapat menyelesaikan jalur modul '{}': {}", path.display(), error), None,
            ))
        })?;
        if !canonical.starts_with(&self.root_dir) {
            return Err(DiagnosticBundle::single(Diagnostic::error(
                "HV1018", format!("Modul '{}' berada di luar direktori proyek.", path.display()), None,
            )));
        }
        if self.stack.contains(&canonical) {
            return Err(DiagnosticBundle::single(Diagnostic::error(
                "HV1012", format!("Siklus import terdeteksi pada '{}'.", path.display()), None,
            )));
        }
        if !self.seen.insert(canonical.clone()) {
            return Err(DiagnosticBundle::single(Diagnostic::error(
                "HV1013", format!("Modul '{}' diimpor lebih dari sekali.", path.display()), None,
            )));
        }
        self.stack.insert(canonical.clone());
        self.modules.push(canonical.clone());
        let mut lines = source.lines();
        let header = lines.next().unwrap_or_default();
        let expected = if is_root { "Buat program " } else { "Buat modul " };
        if !header.starts_with(expected) || header[expected.len()..].trim().is_empty() {
            return Err(DiagnosticBundle::single(Diagnostic::error(
                "HV1014", format!("Header '{}' harus diawali '{}'.", path.display(), expected), None,
            )));
        }
        let remaining: Vec<&str> = lines.collect();
        let mut body_start = 0;
        while body_start < remaining.len() && remaining[body_start].trim().is_empty() {
            body_start += 1;
        }
        while body_start < remaining.len() && remaining[body_start].starts_with("Gunakan modul ") {
            let declaration = remaining[body_start].trim();
            let Some(raw) = declaration.strip_prefix("Gunakan modul ") else { unreachable!() };
            let Some(relative) = raw.strip_prefix('"').and_then(|value| value.strip_suffix('"')) else {
                return Err(DiagnosticBundle::single(Diagnostic::error(
                    "HV1015", "Jalur modul harus diapit tanda kutip.", None,
                )));
            };
            let relative_path = std::path::Path::new(relative);
            if relative_path.is_absolute()
                || relative_path.extension().and_then(|value| value.to_str()) != Some("one")
                || relative_path.components().any(|component| !matches!(component, std::path::Component::Normal(_)))
            {
                return Err(DiagnosticBundle::single(Diagnostic::error(
                    "HV1016", format!("Jalur modul '{}' harus relatif, berakhiran .one, dan tidak boleh keluar dari direktori proyek.", relative), None,
                )));
            }
            let child = canonical.parent().unwrap_or_else(|| std::path::Path::new(".")).join(relative_path);
            let child_source = read_confined_module(&self.root_dir, &canonical, relative_path, &child).map_err(|error| {
                DiagnosticBundle::single(Diagnostic::error(
                    "HV1017", format!("Tidak dapat membaca modul '{}' dengan aman: {}", child.display(), error), None,
                ))
            })?;
            self.collect(&child, &child_source, false)?;
            body_start += 1;
            while body_start < remaining.len() && remaining[body_start].trim().is_empty() {
                body_start += 1;
            }
        }
        for line in &remaining[body_start..] {
            self.output.push_str(line);
            self.output.push('\n');
        }
        self.stack.remove(&canonical);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_program_uses_canonical_ir_for_values_control_flow_and_functions() {
        let source = concat!(
            "Buat program jumlahkan nilai\n",
            "buat pekerjaan jumlahkan\n",
            "    terima kiri\n",
            "    terima kanan\n",
            "    hasilnya adalah kiri ditambah kanan\n",
            "hasil adalah jumlahkan dengan 20 dan 22\n",
            "kalau hasil sama dengan 42\n",
            "    tampilkan hasil\n",
            "kalau tidak\n",
            "    tampilkan \"hasil salah\"\n",
        );
        let compiled = compile(source).unwrap();
        assert_eq!(compiled.name, "jumlahkan nilai");
        assert!(compiled.ir.canonical_text().contains("function jumlahkan"));
        assert!(compiled.ir.canonical_text().contains("if (hasil equal 42)"));
    }

    #[test]
    fn human_core_builtin_calls_execute_through_canonical_interpreter() {
        let source = concat!(
            "Buat program uji pustaka\n",
            "panjang adalah panjang teks dengan \"kucing\"\n",
            "tampilkan panjang\n",
        );
        let compiled = compile(source).unwrap();
        let mut interpreter = crate::interpreter::Interpreter::new(crate::capability::CapabilitySet::none());
        let result = interpreter.execute(&compiled.ir).unwrap();
        assert!(result.output.iter().any(|line| line == "6"), "unexpected output: {:?}", result.output);
    }

    #[test]
    fn rejects_missing_program_declaration() {
        let error = compile("tampilkan \"halo\"\n").unwrap_err();
        assert_eq!(error.0[0].code, "HV1001");
    }

    #[test]
    fn shifts_semantic_diagnostic_to_original_source_line() {
        let error = compile("Buat program rusak\ntampilkan tidak_ada\n").unwrap_err();
        assert_eq!(error.0[0].span.as_ref().unwrap().line, 2);
    }
}
