use std::sync::{LazyLock, Mutex};

use logos::Logos;

use crate::{
    diagnostics::error::Error,
    mir::lower_ast::lower_ast,
    runtime::vm::run_vm,
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

use crate::runtime::function::Function as runtime_function;

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub fn compile_source_code(source: &str) -> Result<Vec<runtime_function>, Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let functions = lower_ast(ast)?;

    for function in functions.iter() {
        println!("{}", function);
    }
    /* let functions = functions
    .into_iter()
    .map(|function| function.run_optimization_passes())
    .collect::<Vec<runtime_function>>(); */

    todo!()
    //Ok(functions)
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let functions = compile_source_code(source)?;

    //run_vm(functions)?;

    Ok(())
}
