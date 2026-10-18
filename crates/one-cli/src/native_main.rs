use std::{env, fs, path::Path, process::ExitCode};

use one_core::{analyze, lower, NativeAndroidAarch64Backend};

fn usage() {
    eprintln!("Penggunaan: one-native build <file.one> [artifact]");
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else { usage(); return ExitCode::from(2); };
    if command != "build" && command != "bangun" { usage(); return ExitCode::from(2); }
    let Some(source_path) = args.next() else { usage(); return ExitCode::from(2); };
    let source = match fs::read_to_string(&source_path) {
        Ok(value) => value,
        Err(error) => { eprintln!("NATIVE-IO: tidak dapat membaca '{}': {error}", source_path); return ExitCode::from(2); }
    };
    if args.next().is_some() && args.next().is_some() { usage(); return ExitCode::from(2); }
    let program = match one_core::parse(&source) {
        Ok(value) => value,
        Err(bundle) => { for diagnostic in bundle.0 { eprintln!("{}: {}", diagnostic.code, diagnostic.message); } return ExitCode::from(1); }
    };
    let typed = match analyze(&program) {
        Ok(value) => value,
        Err(bundle) => { for diagnostic in bundle.0 { eprintln!("{}: {}", diagnostic.code, diagnostic.message); } return ExitCode::from(1); }
    };
    let ir = match lower(&typed) {
        Ok(value) => value,
        Err(bundle) => { for diagnostic in bundle.0 { eprintln!("{}: {}", diagnostic.code, diagnostic.message); } return ExitCode::from(1); }
    };
    let backend = NativeAndroidAarch64Backend::new();
    let output = Path::new(&source_path).parent().unwrap_or_else(|| Path::new(".")).join(
        args.next().unwrap_or_else(|| format!("{}-android-aarch64", Path::new(&source_path).file_stem().and_then(|v| v.to_str()).unwrap_or("program")))
    );
    match backend.build(&ir, &output) {
        Ok(artifact) => { println!("Build native berhasil: {}", artifact.path.display()); ExitCode::SUCCESS }
        Err(error) => { eprintln!("{error}"); ExitCode::from(1) }
    }
}
