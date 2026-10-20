use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut exit = 0;
    let mut artifact = None;

    for (index, arg) in args.iter().enumerate() {
        if let Some(value) = arg.strip_prefix("--exit=") {
            exit = value.parse::<i32>().unwrap_or(2);
        } else if let Some(path) = arg.strip_prefix("--create=") {
            artifact = Some(path.to_owned());
        } else if arg == "-o" {
            artifact = args.get(index + 1).cloned();
        } else {
            println!("{arg}");
        }
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
