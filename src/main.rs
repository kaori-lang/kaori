use std::{fs, time::Instant};

use kaori::{
    backend::{
        codegen::{bytecode::Bytecode, bytecode_generator::BytecodeGenerator},
        vm::kaori_vm::KaoriVM,
    },
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

    let mut instructions = Vec::new();
    let mut constant_pool = Vec::new();

    let mut bytecode_generator = BytecodeGenerator::new(&mut instructions, &mut constant_pool);

    bytecode_generator.generate(&mut nodes)?;

    let bytecode = Bytecode::new(instructions, constant_pool);

    let mut vm = KaoriVM::new(bytecode);

    let start = Instant::now();

    vm.execute_instructions();

    println!("Vm executed in: {:#?}", start.elapsed());

    Ok(())
}
