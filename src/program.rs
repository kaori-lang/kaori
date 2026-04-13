use std::time::Instant;

use crate::{
    ast::{self, parser::Parser},
    bytecode::emit_bytecode::emit_bytecode,
    cfg::{
        self, Function, build_functions_graph::build_functions_graph,
        jump_threading::run_jump_threading_optimization,
    },
    error::kaori_error::KaoriError,
    hir::{decl::Decl, resolver::Resolver},
    lexer::{lexer::Lexer, token_stream::TokenStream},
    runtime::{function::from_compiled, gc::Gc, vm::Vm},
};

fn run_lexical_analysis(source: &'_ str) -> Result<TokenStream<'_>, KaoriError> {
    let lexer = Lexer::new(source);

    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);
    Ok(token_stream)
}

fn run_syntax_analysis(token_stream: TokenStream) -> Result<Vec<ast::Decl>, KaoriError> {
    let mut parser = Parser::new(token_stream);

    let ast = parser.parse()?;

    Ok(ast)
}

fn run_semantic_analysis(ast: &mut [ast::Decl]) -> Result<Vec<Decl>, KaoriError> {
    let mut resolver = Resolver::default();

    let declarations = resolver.resolve(ast)?;

    Ok(declarations)
}

fn run_cfg_analysis(functions: &mut [cfg::function::Function]) -> Result<(), KaoriError> {
    run_jump_threading_optimization(functions);

    Ok(())
}

pub fn compile_source_code(source: &str) -> Result<Vec<Function>, KaoriError> {
    let token_stream = run_lexical_analysis(source)?;
    let mut ast = run_syntax_analysis(token_stream)?;

    let declarations = run_semantic_analysis(&mut ast)?;
    let mut functions = build_functions_graph(&declarations)?;

    run_cfg_analysis(&mut functions)?;

    /*  for function in functions.iter() {
           println!("{}", function);
       }
    */

    Ok(functions)
}

pub fn run_program(source: &str) -> Result<(), KaoriError> {
    let functions = compile_source_code(source)?;

    let bytecode = emit_bytecode(functions);

    let mut gc = Gc::default();
    let functions = from_compiled(bytecode, &mut gc);

    /*   for function in functions.iter() {
        println!("{}", function);
    } */

    let start = Instant::now();

    let mut vm = Vm::new(gc);
    let entry = &functions[0];
    vm.run(entry)?;

    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
