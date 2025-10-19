#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use kaori::program::run_program;
/*

fn main() -> ExitCode {
    let source_to_run = args().nth(1);

    if source_to_run.is_none() {
        eprintln!("Error: No path was found for the program's source!");
        return ExitCode::FAILURE;
    }

    let source_path = source_to_run.unwrap();

    if let Ok(source) = fs::read_to_string(source_path) {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
            return ExitCode::FAILURE;
        }

        return ExitCode::SUCCESS;
    }

    eprintln!("Error: Could not read the file by the given path.");
    ExitCode::FAILURE
}
 */
fn main() {
    let source_to_run = "test_suite/iterative_fib.kr";

    match fs::read_to_string(source_to_run) {
        Ok(source) => {
            if let Err(error) = run_program(source.to_owned()) {
                error.report(&source);
            }
        }
        Err(_) => eprintln!("Error: Could not read the file by the given path."),
    };
}
