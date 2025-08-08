use std::{fs, time::Instant};

use kaori::{
    backend::{
        codegen::{bytecode_generator::BytecodeGenerator, constant_pool::ConstantPool},
        vm::interpreter::Interpreter,
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
    let mut constant_pool = ConstantPool::default();

    let mut bytecode_generator = BytecodeGenerator::new(&mut instructions, &mut constant_pool);

    bytecode_generator.generate(&mut nodes)?;

    print!("{:#?}", instructions.to_owned());
    let mut interpreter = Interpreter::new(instructions, constant_pool);

    let start = Instant::now();

    interpreter.execute_instructions()?;

    println!("Vm executed in: {:#?}", start.elapsed());

    Ok(())
}
