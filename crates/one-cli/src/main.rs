use std::{env, fs, path::{Path, PathBuf}, process::ExitCode};
use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

const VERSION: &str = "0.1.0";

fn usage() {
    println!("One v{VERSION}\n\nPenggunaan:\n  one new <folder>\n  one check <file.one>\n  one run <file.one>\n  one test <folder>\n  one build <file.one>\n  one package <folder>\n  one version\n  one help");
}

fn print_diagnostics(bundle: &one_core::DiagnosticBundle) {
    for diagnostic in &bundle.0 {
        eprintln!("{}: {}", diagnostic.code, diagnostic.message);
        if let Some(span) = &diagnostic.span { eprintln!("  baris {}, kolom {}", span.line, span.column); }
        for note in &diagnostic.notes { eprintln!("  catatan: {}", note); }
        for suggestion in &diagnostic.suggestions { eprintln!("  saran: {}", suggestion); }
    }
}

fn compile_source(source: &str) -> Result<(one_core::Program, one_core::TypedProgram), one_core::DiagnosticBundle> {
    let program = parse(source)?;
    let typed = analyze(&program)?;
    Ok((program, typed))
}

fn project_main(root: &Path) -> Result<PathBuf, String> {
    let path = root.join("src").join("main.one");
    if path.is_file() { Ok(path) } else { Err(format!("Proyek tidak memiliki '{}'.", path.display())) }
}

fn project_metadata(root: &Path) -> Result<(String, String), String> {
    let manifest = root.join("one.toml");
    let text = fs::read_to_string(&manifest).map_err(|e| format!("Tidak dapat membaca '{}': {}", manifest.display(), e))?;
    let mut name = None;
    let mut version = None;
    for line in text.lines() {
        let mut parts = line.splitn(2, '=');
        let key = parts.next().map(str::trim);
        let value = parts.next().map(str::trim).and_then(|v| v.strip_prefix('"')).and_then(|v| v.strip_suffix('"'));
        match (key, value) {
            (Some("nama"), Some(v)) => name = Some(v.to_owned()),
            (Some("versi"), Some(v)) => version = Some(v.to_owned()),
            _ => {}
        }
    }
    match (name, version) {
        (Some(name), Some(version)) if !name.is_empty() && !version.is_empty() => Ok((name, version)),
        _ => Err(format!("Manifest '{}' harus memuat nama dan versi.", manifest.display()))
    }
}

fn compile_file(path: &Path) -> Result<(one_core::Program, one_core::TypedProgram), one_core::DiagnosticBundle> {
    let source = fs::read_to_string(path).map_err(|error| one_core::DiagnosticBundle::single(one_core::Diagnostic::error("ETOOL001", format!("Tidak dapat membaca '{}': {}", path.display(), error), None)))?;
    compile_source(&source)
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else { usage(); return ExitCode::from(2); };

    if command == "help" {
        if args.next().is_some() { usage(); return ExitCode::from(2); }
        usage();
        return ExitCode::SUCCESS;
    }
    if command == "version" {
        if args.next().is_some() { usage(); return ExitCode::from(2); }
        println!("one {VERSION}");
        return ExitCode::SUCCESS;
    }
    if command == "new" {
        let Some(folder) = args.next() else { usage(); return ExitCode::from(2); };
        if args.next().is_some() { usage(); return ExitCode::from(2); }
        let root = Path::new(&folder);
        if root.exists() { eprintln!("Folder '{}' sudah ada.", folder); return ExitCode::from(2); }
        let src = root.join("src");
        if let Err(error) = fs::create_dir_all(&src)
            .and_then(|_| fs::write(root.join("one.toml"), "nama = \"proyek-one\"\nversi = \"0.1.0\"\n"))
            .and_then(|_| fs::write(src.join("main.one"), "tampilkan \"Halo dari One\"\n"))
        {
            eprintln!("Gagal membuat proyek: {}", error);
            return ExitCode::from(2);
        }
        println!("Proyek One dibuat di {}", root.display());
        return ExitCode::SUCCESS;
    }

    let Some(path_arg) = args.next() else { usage(); return ExitCode::from(2); };
    if args.next().is_some() { usage(); return ExitCode::from(2); }
    let path = Path::new(&path_arg);

    if command == "test" || command == "package" {
        let root = if path.is_dir() { path.to_path_buf() } else { path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf() };
        let main_file = match project_main(&root) {
            Ok(value) => value,
            Err(message) => { eprintln!("ETOOL001: {message}"); return ExitCode::from(2); }
        };
        let (_, typed) = match compile_file(&main_file) {
            Ok(value) => value,
            Err(error) => { print_diagnostics(&error); return ExitCode::from(1); }
        };
        let ir = match lower(&typed) {
            Ok(value) => value,
            Err(error) => { print_diagnostics(&error); return ExitCode::from(1); }
        };
        if command == "test" {
            let mut interpreter = Interpreter::new(CapabilitySet::none());
            match interpreter.execute(&ir) {
                Ok(_) => { println!("Tests berhasil: {}", root.display()); ExitCode::SUCCESS }
                Err(error) => { eprintln!("{}: {}", error.code, error.message); ExitCode::from(1) }
            }
        } else {
            let (name, version) = match project_metadata(&root) {
                Ok(value) => value,
                Err(message) => { eprintln!("ETOOL002: {message}"); return ExitCode::from(2); }
            };
            let target = root.join("target");
            if let Err(error) = fs::create_dir_all(&target) { eprintln!("Gagal membuat target: {}", error); return ExitCode::from(2); }
            let artifact = target.join(format!("{name}-{version}.onepkg"));
            let manifest = fs::read_to_string(root.join("one.toml")).unwrap_or_default();
            let contents = format!("One package v0.1\n\n[manifest]\n{manifest}\n[ir]\n{ir:#?}\n");
            if let Err(error) = fs::write(&artifact, contents) { eprintln!("Gagal menulis paket: {}", error); return ExitCode::from(2); }
            println!("Paket berhasil: {}", artifact.display());
            ExitCode::SUCCESS
        }
    } else {
        let (program, typed) = match compile_file(path) {
            Ok(value) => value,
            Err(error) => { print_diagnostics(&error); return ExitCode::from(1); }
        };
        match command.as_str() {
            "check" => { println!("OK: {}", path.display()); ExitCode::SUCCESS }
            "ast" => { println!("{:#?}", program); ExitCode::SUCCESS }
            "run" => match lower(&typed) {
                Ok(ir) => {
                    let mut interpreter = Interpreter::new(CapabilitySet::none());
                    match interpreter.execute(&ir) {
                        Ok(result) => { for line in result.output { println!("{}", line); } ExitCode::SUCCESS }
                        Err(error) => { eprintln!("{}: {}", error.code, error.message); ExitCode::from(1) }
                    }
                }
                Err(error) => { print_diagnostics(&error); ExitCode::from(1) }
            },
            "build" => match lower(&typed) {
                Ok(ir) => {
                    let parent = path.parent().unwrap_or_else(|| Path::new("."));
                    let target = parent.join("target");
                    if let Err(error) = fs::create_dir_all(&target) { eprintln!("Gagal membuat target: {}", error); return ExitCode::from(2); }
                    let output = target.join(format!("{}.oneir", path.file_stem().and_then(|v| v.to_str()).unwrap_or("program")));
                    if let Err(error) = fs::write(&output, format!("{ir:#?}\n")) { eprintln!("Gagal menulis artefak build: {}", error); return ExitCode::from(2); }
                    println!("Build berhasil: {}", output.display()); ExitCode::SUCCESS
                }
                Err(error) => { print_diagnostics(&error); ExitCode::from(1) }
            },
            _ => { usage(); ExitCode::from(2) }
        }
    }
}
