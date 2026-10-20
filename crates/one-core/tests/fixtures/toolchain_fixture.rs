use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut exit = 0;
    let mut artifact = None;
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        if let Some(value) = arg.strip_prefix("--exit=") {
            exit = value.parse::<i32>().unwrap_or(2);
        } else if let Some(path) = arg.strip_prefix("--create=") {
            artifact = Some(path.to_owned());
        } else if arg == "-o" {
            println!("-o");
            if let Some(path) = args.get(index + 1) {
                println!("output={path}");
                artifact = Some(path.clone());
                index += 1;
            }
        } else {
            println!("{arg}");
        }
        index += 1;
    }

    if let Some(path) = artifact {
        if let Err(error) = fs::write(path, b"fixture artifact") {
            eprintln!("fixture artifact write failed: {error}");
            return ExitCode::from(3);
        }
    }

    if exit != 0 {
        eprintln!("fixture exit {exit}");
        ExitCode::from(exit as u8)
    } else {
        ExitCode::SUCCESS
    }
}
