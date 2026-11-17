#[path = "../integrity.rs"]
mod integrity;

use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::from(integrity::run(std::env::args().skip(1)))
}
