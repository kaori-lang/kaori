use std::time::Instant;

use crate::{
    ast::parser::Parser,
    bytecode::{Function, emit_bytecode::compile, optimize_bytecode::optimize_bytecode},
    error::kaori_error::KaoriError,
    lexer::{lexer::Lexer, token_stream::TokenStream},
    runtime::{function::from_compiled, gc::Gc, vm::Vm},
};

pub fn compile_source_code(source: &str) -> Result<Vec<Function>, KaoriError> {
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let token_stream = TokenStream::new(source, tokens);
    let mut parser = Parser::new(token_stream);
    let ast = parser.parse()?;

    let mut bytecode = compile(&ast)?;

    optimize_bytecode(&mut bytecode);

    Ok(bytecode)
}

pub fn run_program(source: &str) -> Result<(), KaoriError> {
    let bytecode = compile_source_code(source)?;

    for function in bytecode.iter() {
        println!("{}", function);
    }

    let mut gc = Gc::default();
    let functions = from_compiled(bytecode, &mut gc);

    let start = Instant::now();

    let mut vm = Vm::new(gc);
    let entry = &functions[0];
    vm.run(entry)?;

    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
