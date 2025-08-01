use std::fs;

use kaori::{
    backend::{codegen::bytecode_generator::BytecodeGenerator, vm::kaori_vm::KaoriVM},
    error::kaori_error::KaoriError,
};

fn main() {
    if let Ok(source) = fs::read_to_string("src/code/main.kaori") {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
        }
    }
}

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let mut bytecode_generator = BytecodeGenerator::new();

    let bytecode = bytecode_generator.generate(&mut nodes)?;

    let mut vm = KaoriVM::new(bytecode);

    vm.execute_instructions();

    Ok(())
}
