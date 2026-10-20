use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let mut exit = 0;
    for arg in env::args().skip(1) {
        if let Some(value) = arg.strip_prefix("--exit=") {
            exit = value.parse::<i32>().unwrap_or(2);
        } else if let Some(path) = arg.strip_prefix("--create=") {
            if let Err(error) = fs::write(path, b"fixture artifact") {
                eprintln!("fixture artifact write failed: {error}");
                return ExitCode::from(3);
            }
        } else {
            println!("{arg}");
        }
    }
    if exit != 0 {
        eprintln!("fixture exit {exit}");
        ExitCode::from(exit as u8)
    } else {
        ExitCode::SUCCESS
    }
}
