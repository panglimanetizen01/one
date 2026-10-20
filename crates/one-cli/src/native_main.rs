use std::{env, fs, path::Path, process::ExitCode};

use one_core::{
    NativeAndroidAarch64Backend, NativeLinuxAarch64Backend, NativeLinuxX86_64Backend,
    NativeWindowsX86_64Backend, analyze, lower, target_definition,
};

fn usage() {
    eprintln!(
        "Penggunaan: one-native build [--target <target>] <file.one> [artifact]\n\
         Target: android-aarch64 | linux-aarch64 | linux-x86_64 | windows-x86_64"
    );
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);

    let Some(command) = args.next() else {
        usage();
        return ExitCode::from(2);
    };

    if command != "build" && command != "bangun" {
        usage();
        return ExitCode::from(2);
    }

    let first = args.next();

    let (target, source_path) = match first {
        Some(value) if value == "--target" => {
            let Some(selected_target) = args.next() else {
                usage();
                return ExitCode::from(2);
            };

            if target_definition(&selected_target).is_none() {
                usage();
                return ExitCode::from(2);
            }

            let Some(source_path) = args.next() else {
                usage();
                return ExitCode::from(2);
            };

            (selected_target, source_path)
        }
        Some(source_path) => ("android-aarch64".to_owned(), source_path),
        None => {
            usage();
            return ExitCode::from(2);
        }
    };

    let output_name = args.next();

    if args.next().is_some() {
        usage();
        return ExitCode::from(2);
    }

    let source = match fs::read_to_string(&source_path) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("NATIVE-IO: tidak dapat membaca '{}': {error}", source_path);
            return ExitCode::from(2);
        }
    };

    let program = match one_core::parse(&source) {
        Ok(value) => value,
        Err(bundle) => {
            for diagnostic in bundle.0 {
                eprintln!("{}: {}", diagnostic.code, diagnostic.message);
            }
            return ExitCode::from(1);
        }
    };

    let typed = match analyze(&program) {
        Ok(value) => value,
        Err(bundle) => {
            for diagnostic in bundle.0 {
                eprintln!("{}: {}", diagnostic.code, diagnostic.message);
            }
            return ExitCode::from(1);
        }
    };

    let ir = match lower(&typed) {
        Ok(value) => value,
        Err(bundle) => {
            for diagnostic in bundle.0 {
                eprintln!("{}: {}", diagnostic.code, diagnostic.message);
            }
            return ExitCode::from(1);
        }
    };

    let source_stem = Path::new(&source_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("program");

    let default_name = format!("{source_stem}-{target}");

    let output = Path::new(&source_path)
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(output_name.unwrap_or(default_name));

    let result = match target.as_str() {
        "android-aarch64" => NativeAndroidAarch64Backend::new().build(&ir, &output),
        "linux-aarch64" => NativeLinuxAarch64Backend::new().build(&ir, &output),
        "linux-x86_64" => NativeLinuxX86_64Backend::new().build(&ir, &output),
        "windows-x86_64" => NativeWindowsX86_64Backend::new().build(&ir, &output),
        _ => unreachable!("target sudah divalidasi"),
    };

    match result {
        Ok(artifact) => {
            println!("Build native berhasil: {}", artifact.path.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}
