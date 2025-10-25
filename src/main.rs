#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use kaori::program::run_program;

fn main() {
    let source_to_run = "test_suite/recursive_fib.kr";

    match fs::read_to_string(source_to_run) {
        Ok(source) => {
            if let Err(error) = run_program(source.to_owned()) {
                error.report(&source);
            }
        }
        Err(_) => eprintln!("Error: Could not read the file by the given path."),
    };
}
