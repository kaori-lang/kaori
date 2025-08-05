use std::{fs, time::Instant};

use kaori::{
    backend::{codegen::bytecode_generator::BytecodeGenerator, vm::kaori_vm::KaoriVM},
    error::kaori_error::KaoriError,
    frontend::generate_ast::generate_ast,
};

fn main() {
    if let Ok(source) = fs::read_to_string("src/code/main.kaori") {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
        }
    }
}

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let mut nodes = generate_ast(source)?;

    let mut bytecode_generator = BytecodeGenerator::new();

    let bytecode = bytecode_generator.generate(&mut nodes)?;

    let mut vm = KaoriVM::new(bytecode);

    let start = Instant::now();

    vm.execute_instructions();

    println!("Vm executed in: {:#?}", start.elapsed());

    Ok(())
}
