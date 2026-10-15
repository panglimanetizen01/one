use std::{env, fs, path::Path, process::ExitCode};
use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

fn usage() {
    eprintln!("One v0.1\n\nPenggunaan:\n  one buat <folder>\n  one jalankan <file.one>\n  one periksa <file.one>\n  one bangun <file.one>\n  one lihat struktur <file.one>\n\nAlias teknis:\n  one new <folder>\n  one run <file.one>\n  one check <file.one>\n  one build <file.one>\n  one ast <file.one>\n");
}

fn print_diagnostics(bundle: &one_core::DiagnosticBundle) {
    for diagnostic in &bundle.0 {
        eprintln!("{}: {}", diagnostic.code, diagnostic.message);
        if let Some(span) = &diagnostic.span {
            eprintln!("  baris {}, kolom {}", span.line, span.column);
        }
        for note in &diagnostic.notes {
            eprintln!("  catatan: {}", note);
        }
        for suggestion in &diagnostic.suggestions {
            eprintln!("  saran: {}", suggestion);
        }
    }
}

fn compile_source(source: &str) -> Result<(one_core::Program, one_core::TypedProgram), one_core::DiagnosticBundle> {
    let program = parse(source)?;
    let typed = analyze(&program)?;
    Ok((program, typed))
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(first) = args.next() else {
        usage();
        return ExitCode::from(2);
    };

    let command = match first.as_str() {
        "buat" | "new" => "new",
        "jalankan" | "run" => "run",
        "periksa" | "check" => "check",
        "bangun" | "build" => "build",
        "ast" => "ast",
        "lihat" => {
            if args.next().as_deref() == Some("struktur") {
                "ast"
            } else {
                usage();
                return ExitCode::from(2);
            }
        }
        _ => {
            usage();
            return ExitCode::from(2);
        }
    };

    if command == "new" {
        let Some(folder) = args.next() else {
            usage();
            return ExitCode::from(2);
        };
        if args.next().is_some() {
            usage();
            return ExitCode::from(2);
        }
        let root = Path::new(&folder);
        let src = root.join("src");
        let main_file = src.join("main.one");
        if root.exists() {
            eprintln!("Folder '{}' sudah ada.", folder);
            return ExitCode::from(2);
        }
        if let Err(error) = fs::create_dir_all(&src)
            .and_then(|_| fs::write(root.join("one.toml"), "nama = \"proyek-one\"\nversi = \"0.1.0\"\n"))
            .and_then(|_| fs::write(main_file, "tampilkan \"Halo dari One\"\n"))
        {
            eprintln!("Gagal membuat proyek: {}", error);
            return ExitCode::from(2);
        }
        println!("Proyek One dibuat di {}", root.display());
        return ExitCode::SUCCESS;
    }

    let Some(path) = args.next() else {
        usage();
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        usage();
        return ExitCode::from(2);
    }

    let source = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Tidak dapat membaca '{}': {}", path, error);
            return ExitCode::from(2);
        }
    };
    let (program, typed) = match compile_source(&source) {
        Ok(value) => value,
        Err(error) => {
            print_diagnostics(&error);
            return ExitCode::from(1);
        }
    };

    match command {
        "check" => {
            println!("OK: {}", path);
            ExitCode::SUCCESS
        }
        "ast" => {
            println!("{:#?}", program);
            ExitCode::SUCCESS
        }
        "run" => match lower(&typed) {
            Ok(ir) => {
                let mut interpreter = Interpreter::new(CapabilitySet::none());
                match interpreter.execute(&ir) {
                    Ok(result) => {
                        for line in result.output {
                            println!("{}", line);
                        }
                        ExitCode::SUCCESS
                    }
                    Err(error) => {
                        eprintln!("{}: {}", error.code, error.message);
                        ExitCode::from(1)
                    }
                }
            }
            Err(error) => {
                print_diagnostics(&error);
                ExitCode::from(1)
            }
        },
        "build" => match lower(&typed) {
            Ok(ir) => {
                let input = Path::new(&path);
                let parent = input.parent().unwrap_or_else(|| Path::new("."));
                let target = parent.join("target");
                if let Err(error) = fs::create_dir_all(&target) {
                    eprintln!("Gagal membuat target: {}", error);
                    return ExitCode::from(2);
                }
                let output = target.join(format!(
                    "{}.oneir",
                    input.file_stem().and_then(|v| v.to_str()).unwrap_or("program")
                ));
                if let Err(error) = fs::write(&output, format!("{:#?}\n", ir)) {
                    eprintln!("Gagal menulis artefak build: {}", error);
                    return ExitCode::from(2);
                }
                println!("Build berhasil: {}", output.display());
                ExitCode::SUCCESS
            }
            Err(error) => {
                print_diagnostics(&error);
                ExitCode::from(1)
            }
        },
        _ => unreachable!("command is normalized before dispatch"),
    }
}
