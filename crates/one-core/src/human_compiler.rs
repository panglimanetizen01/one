//! Human-instruction vNext frontend adapter to ONE's canonical semantic pipeline.
//!
//! The first line declares the program identity. The body uses the accepted Indonesian
//! instruction grammar and is compiled by the same parser, semantic analyzer, canonical
//! IR lowering and reference interpreter as v0.1. This adapter deliberately does not
//! implement a second evaluator or reinterpret expressions at runtime.
use crate::{Diagnostic, DiagnosticBundle, IrProgram};

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
    let ast = crate::parse(&parsed.body).map_err(|bundle| shift_diagnostics(bundle, parsed.body_line_offset))?;
    let typed = crate::analyze(&ast).map_err(|bundle| shift_diagnostics(bundle, parsed.body_line_offset))?;
    let ir = crate::lower(&typed).map_err(|bundle| shift_diagnostics(bundle, parsed.body_line_offset))?;
    Ok(CompiledHumanProgram { name: parsed.name, ir })
}

fn shift_diagnostics(mut bundle: DiagnosticBundle, offset: usize) -> DiagnosticBundle {
    for diagnostic in &mut bundle.0 {
        if let Some(span) = &mut diagnostic.span {
            span.line += offset;
        }
    }
    bundle
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
