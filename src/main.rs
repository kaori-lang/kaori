#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use clap::{Arg, Command};
use kaori::interpreter::compile_and_run;

use std::path::PathBuf;

/* fn main() {
    let matches = Command::new("kaori")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file: PathBuf = matches.get_one::<String>("file").unwrap().into();

    match fs::read_to_string(&file) {
        Ok(source) => {
            if let Err(error) = compile_and_run(&source) {
                error.report(&source);
            }
        }
        Err(_) => eprintln!("Error: Could not read the file by the given path."),
    };
} */

fn main() {
    let source = fs::read_to_string("main.kr").expect("could not read main.kr");

    if let Err(error) = compile_and_run(&source) {
        error.report(&source);
    }
}
