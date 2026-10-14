use std::{env, fs, process::ExitCode};
use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

fn usage() { eprintln!("One v0.1\n\nPenggunaan:\n  one check <file.one>\n  one run <file.one>\n  one ast <file.one>\n"); }

fn print_diagnostics(bundle: &one_core::DiagnosticBundle) {
    for diagnostic in &bundle.0 {
        eprintln!("{}: {}", diagnostic.code, diagnostic.message);
        if let Some(span) = &diagnostic.span { eprintln!("  baris {}, kolom {}", span.line, span.column); }
        for note in &diagnostic.notes { eprintln!("  catatan: {}", note); }
        for suggestion in &diagnostic.suggestions { eprintln!("  saran: {}", suggestion); }
    }
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else { usage(); return ExitCode::from(2); };
    let Some(path) = args.next() else { usage(); return ExitCode::from(2); };
    if args.next().is_some() { usage(); return ExitCode::from(2); }
    let source = match fs::read_to_string(&path) { Ok(value) => value, Err(error) => { eprintln!("Tidak dapat membaca '{}': {}", path, error); return ExitCode::from(2); } };
    let program = match parse(&source) { Ok(value) => value, Err(error) => { print_diagnostics(&error); return ExitCode::from(1); } };
    let typed = match analyze(&program) { Ok(value) => value, Err(error) => { print_diagnostics(&error); return ExitCode::from(1); } };
    match command.as_str() {
        "check" => { println!("OK: {}", path); ExitCode::SUCCESS }
        "ast" => { println!("{:#?}", program); ExitCode::SUCCESS }
        "run" => match lower(&typed) {
            Ok(ir) => { let mut interpreter = Interpreter::new(CapabilitySet::none()); match interpreter.execute(&ir) { Ok(result) => { for line in result.output { println!("{}", line); } ExitCode::SUCCESS }, Err(error) => { eprintln!("{}: {}", error.code, error.message); ExitCode::from(1) } } }
            Err(error) => { print_diagnostics(&error); ExitCode::from(1) }
        },
        _ => { usage(); ExitCode::from(2) }
    }
}
