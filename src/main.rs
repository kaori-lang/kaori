use std::env::args;
use std::process::ExitCode;

// TODO: remove this line suppression after using the import.
#[allow(unused_imports)]
use std::{fs, time::Instant};

use kaori::{ error::kaori_error::KaoriError, frontend::parse_and_analyze::parse_and_analyze};

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

// TODO: remove the lint suppressions after using unused variables.
#[allow(unused_variables)]
pub fn run_program(source: String) -> Result<(), KaoriError> {
    #[allow(clippy::let_unit_value)]
    let resolved_declarations = parse_and_analyze(source)?;

    /*  let mut bytecode = Bytecode::default();

    let mut bytecode_generator = BytecodeGenerator::new(&mut bytecode);

    bytecode_generator.generate(&resolved_declarations)?;

    let instructions = bytecode.instructions;
    let constant_pool = bytecode.constant_pool.constants;

    let mut interpreter = Interpreter::new(instructions, constant_pool);

    let start = Instant::now();

    interpreter.execute_instructions()?;

    println!("Vm executed in: {:#?}", start.elapsed()); */

    Ok(())
}
