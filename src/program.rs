use std::sync::{LazyLock, Mutex};

use logos::Logos;

use crate::{
    bytecode::{Function, emit_bytecode::CompilerContext, resolve::resolve},
    diagnostics::error::Error,
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub fn compile_source_code(source: &str) -> Result<Vec<Function>, Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let captures = resolve(&ast)?;

    let compiler = CompilerContext::new(ast, captures);

    let mut functions = compiler.compile();

    for function in functions.iter_mut() {
        function.run_optimization_passes();
    }

    for (index, function) in functions.iter().enumerate() {
        println!("FUNCTION {}", index);
        println!("{}", function);
        println!("{:?}", function.live_ranges);
    }

    Ok(functions)
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let functions = compile_source_code(source)?;

    //run_vm(functions)?;

    Ok(())
}
