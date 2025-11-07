use std::time::Instant;

use crate::{
    bytecode::{bytecode::Bytecode, emit_bytecode::emit_bytecode},
    cfg_ir::{
        build_cfgs::build_cfgs, cfg_function::CfgFunction,
        jump_threading::run_jump_threading_optimization,
    },
    error::kaori_error::KaoriError,
    lexer::{lexer::Lexer, token_stream::TokenStream},
    semantic::{hir_ir::HirIr, resolver::Resolver, type_checker::TypeChecker},
    syntax::{decl::Decl, parser::Parser},
    virtual_machine::vm::run_vm,
};

fn run_lexical_analysis(source: &'_ str) -> Result<TokenStream<'_>, KaoriError> {
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);
    Ok(token_stream)
}

fn run_syntax_analysis(token_stream: TokenStream) -> Result<Vec<Decl>, KaoriError> {
    let mut parser = Parser::new(token_stream);

    let ast = parser.parse()?;

    Ok(ast)
}

fn run_semantic_analysis(ast: &mut [Decl]) -> Result<HirIr, KaoriError> {
    let mut resolver = Resolver::default();

    let declarations = resolver.resolve(ast)?;

    let type_checker = TypeChecker::default();

    let types = type_checker.type_check(&declarations)?;

    let hir = HirIr::new(declarations, types);

    Ok(hir)
}

fn run_optimizations(cfgs: &mut [CfgFunction]) {
    run_jump_threading_optimization(cfgs);
}

pub fn compile_source_code(source: &str) -> Result<Bytecode, KaoriError> {
    let token_stream = run_lexical_analysis(source)?;
    let mut ast = run_syntax_analysis(token_stream)?;
    let hir = run_semantic_analysis(&mut ast)?;
    let mut cfgs = build_cfgs(&hir.declarations)?;

    run_optimizations(&mut cfgs);

    let bytecode = emit_bytecode(cfgs);

    //println!("{bytecode}");

    Ok(bytecode)
}

pub fn run_program(source: &str) -> Result<(), KaoriError> {
    // Start timer

    // Compile and run
    let bytecode = compile_source_code(source)?;

    let start = Instant::now();

    run_vm(bytecode.bytes, bytecode.functions);

    // Measure elapsed time
    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
