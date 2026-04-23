use std::time::Instant;

use crate::{
    ast::{self, parser::Parser},
    bytecode::{self, emit_bytecode::emit_bytecode},
    error::kaori_error::KaoriError,
    hir::{expr::Expr, resolver::Resolver},
    lexer::{lexer::Lexer, token_stream::TokenStream},
    runtime::{function::from_compiled, gc::Gc, vm::Vm},
    //runtime::{function::from_compiled, gc::Gc, vm::Vm},
};

fn run_lexical_analysis(source: &'_ str) -> Result<TokenStream<'_>, KaoriError> {
    let lexer = Lexer::new(source);

    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);
    Ok(token_stream)
}

fn run_syntax_analysis(token_stream: TokenStream) -> Result<Vec<ast::Expr>, KaoriError> {
    let mut parser = Parser::new(token_stream);

    let functions = parser.parse()?;

    Ok(functions)
}

fn run_semantic_analysis(ast: &mut [ast::Expr]) -> Result<Vec<Expr>, KaoriError> {
    let mut resolver = Resolver::default();

    let functions = resolver.resolve(ast)?;

    Ok(functions)
}

pub fn compile_source_code(source: &str) -> Result<Vec<bytecode::Function>, KaoriError> {
    let token_stream = run_lexical_analysis(source)?;
    let mut functions = run_syntax_analysis(token_stream)?;

    let functions = run_semantic_analysis(&mut functions)?;

    emit_bytecode(&functions)
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
