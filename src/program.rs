use std::sync::{LazyLock, Mutex, OnceLock};

use logos::Logos;

use crate::{
    bytecode::{
        Function, emit_bytecode::Compiler, optimize_bytecode::optimize_bytecode, resolve::resolve,
    },
    diagnostics::error::Error,
    runtime::{value::Value, vm::Vm},
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));
pub static CONSTANT_POOL: OnceLock<Vec<Value>> = OnceLock::new();
pub static FUNCTIONS: OnceLock<Vec<Function>> = OnceLock::new();

pub fn compile_source_code(source: &str) -> Result<(), Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let captures = resolve(&ast)?;

    let (mut bytecode, constants) = Compiler::default().compile(&ast, captures);

    optimize_bytecode(&mut bytecode);

    /*  for (index, function) in bytecode.iter().enumerate() {
        println!("FUNCTION {}", index);
        println!("{}", function);
    } */

    FUNCTIONS.set(bytecode).unwrap();
    CONSTANT_POOL.set(constants).unwrap();

    Ok(())
}

pub fn run_program(source: &str) -> Result<(), Error> {
    compile_source_code(source)?;

    let mut vm = Vm::new();

    vm.run()?;

    Ok(())
}
