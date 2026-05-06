#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use clap::{Arg, Command};
use kaori::program::run_program;
use std::path::PathBuf;

/* fn main() -> ExitCode {
    let source_to_run = args().nth(1);

    if source_to_run.is_none() {
        eprintln!("Error: No path was found for the program's source!");
        return ExitCode::FAILURE;
    }

    let source_path = source_to_run.unwrap();

    if let Ok(source) = fs::read_to_string(source_path) {
        if let Err(err) = run_program(&source) {
            err.report(&source);
            return ExitCode::FAILURE;
        }

        return ExitCode::SUCCESS;
    }

    eprintln!("Error: Could not read the file by the given path.");
    ExitCode::FAILURE
} */

fn main() {
    let matches = Command::new("kaori")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file: PathBuf = matches.get_one::<String>("file").unwrap().into();

    match fs::read_to_string(&file) {
        Ok(source) => {
            if let Err(error) = run_program(&source) {
                error.report(&source);
            }
        }
        Err(_) => eprintln!("Error: Could not read the file by the given path."),
    };
}
