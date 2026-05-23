use std::sync::{LazyLock, Mutex};

use logos::Logos;

use crate::{
    bytecode::{function::Function, lower_ast::lower_ast},
    diagnostics::error::Error,
    runtime::vm::run_vm,
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub fn compile_source_code(source: &str) -> Result<Vec<Function>, Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let functions = lower_ast(ast)?;

    for function in functions.iter() {
        println!("{}", function);
    }

    Ok(functions)
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let functions = compile_source_code(source)?;

    run_vm(functions)?;

    Ok(())
}
