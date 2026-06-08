use std::collections::HashMap;
use std::hint::black_box;
#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use clap::{Arg, Command};
use kaori::compiler::{Compiler, compile_and_run};

use std::path::PathBuf;

fn main() {
    let matches = Command::new("kaori")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file: PathBuf = matches.get_one::<String>("file").unwrap().into();

    match compile_and_run(file.to_str().unwrap()) {
        Ok(value) => {}
        Err(error) => error.report(),
    }
}

/* fn main() {
    if let Err(error) = compile_and_run("main.kr") {
        error.report()
    }
}
 */
