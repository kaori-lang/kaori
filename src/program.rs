use std::sync::{LazyLock, Mutex};

use logos::Logos;

use crate::{
    diagnostics::error::Error,
    mir::{self, resolve::resolve},
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub fn compile_source_code(source: &str) -> Result<Vec<mir::Function>, Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let resolved_ast = resolve(ast)?;
    let mut functions = resolved_ast.lower();

    for function in functions.iter_mut() {
        function.run_optimization_passes();
    }

    for function in functions.iter() {
        println!("{}", function);
    }

    Ok(functions)
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let functions = compile_source_code(source)?;

    //run_vm(functions)?;

    Ok(())
}
