use std::sync::{LazyLock, Mutex, OnceLock};

use logos::Logos;

use crate::{
    bytecode::{
        Function, emit_bytecode::Compiler, optimize_bytecode::optimize_bytecode, resolve::resolve,
    },
    diagnostics::error::Error,
    runtime::{
        value::Value,
        vm::{VmState, run_vm},
    },
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub fn compile_source_code(source: &str) -> Result<(Vec<Value>, Vec<Function>), Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let captures = resolve(&ast)?;

    let (mut functions, constants) = Compiler::default().compile(&ast, captures);

    optimize_bytecode(&mut functions);

    /*     for (index, function) in bytecode.iter().enumerate() {
        println!("FUNCTION {}", index);
        println!("{}", function);
    } */

    Ok((constants, functions))
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let (constants, functions) = compile_source_code(source)?;

    run_vm(functions, constants)?;

    Ok(())
}
