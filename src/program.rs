use std::{
    sync::{LazyLock, Mutex, OnceLock},
    time::Instant,
};

use logos::Logos;

use crate::{
    bytecode::{Function, emit_bytecode::Compiler, optimize_bytecode::optimize_bytecode},
    diagnostics::error::Error,
    runtime::{value::Value, vm::run_vm},
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));
pub static CONSTANT_POOL: OnceLock<Box<[Value]>> = OnceLock::new();
pub static FUNCTIONS: OnceLock<Box<[Function]>> = OnceLock::new();

pub fn compile_source_code(source: &str) -> Result<(Vec<Function>, Vec<Value>), Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let compiler = Compiler::default();
    let (mut bytecode, constants) = compiler.compile(&ast);

    optimize_bytecode(&mut bytecode);

    for function in bytecode.iter() {
        println!("{}", function);
    }

    Ok((bytecode, constants))
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let (bytecode, constants) = compile_source_code(source)?;

    FUNCTIONS.set(bytecode.into_boxed_slice()).unwrap();
    CONSTANT_POOL.set(constants.into_boxed_slice()).unwrap();

    let start = Instant::now();

    run_vm()?;

    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
