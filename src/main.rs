#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use clap::{Arg, Command};
use kaori::compiler::compile_and_run;
use kaori::runtime::value::Value;

use std::path::PathBuf;
/*
fn main() {
    let matches = Command::new("kaori")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file: PathBuf = matches.get_one::<String>("file").unwrap().into();

    if let Err(error) = compile_and_run(file.to_str().unwrap()) {
        error.report()
    }
}
 */
fn main() {
    if let Err(error) = compile_and_run("main.kr") {
        error.report()
    }
}
