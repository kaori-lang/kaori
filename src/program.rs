use std::time::Instant;

use logos::Logos;

use crate::{
    bytecode::{Function, emit_bytecode::compile, optimize_bytecode::optimize_bytecode},
    diagnostics::diagnostics::Error,
    syntax::{parser::Parser, token::Token},
};

pub fn compile_source_code(source: &str) -> Result<Vec<Function>, Error> {
    let tokens = Token::lexer(source).spanned();

    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut bytecode = compile(&ast)?;

    optimize_bytecode(&mut bytecode);

    Ok(bytecode)
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let bytecode = compile_source_code(source)?;

    for function in bytecode.iter() {
        println!("{}", function);
    }

    /*  let mut gc = Gc::default();
    let functions = from_compiled(bytecode, &mut gc);

    let start = Instant::now();

    let mut vm = Vm::new(gc);
    let entry = &functions[0];
    vm.run(entry)?;

    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0); */

    Ok(())
}
