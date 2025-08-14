use std::{fs, time::Instant};

use kaori::{
    backend::codegen::{
        bytecode::Bytecode, bytecode_generator::BytecodeGenerator, constant_pool::ConstantPool,
    },
    error::kaori_error::KaoriError,
    frontend::parse_and_analyze::parse_and_analyze,
};

fn main() {
    if let Ok(source) = fs::read_to_string("src/code/main.kaori") {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
        }
    }
}

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let resolved_ast = parse_and_analyze(source)?;

    let mut bytecode = Bytecode::default();

    let mut bytecode_generator = BytecodeGenerator::new(&mut bytecode);

    bytecode_generator.generate(&declarations)?;

    //print!("{:#?}", instructions.to_owned());
    //let mut interpreter = Interpreter::new(instructions, constant_pool);

    let start = Instant::now();

    //interpreter.execute_instructions()?;

    println!("Vm executed in: {:#?}", start.elapsed());

    Ok(())
}
