use std::time::Instant;

use crate::{
    bytecode::{
        bytecode::Bytecode,
        bytecode_generator::BytecodeGenerator,
        instruction::{self, Instruction},
    },
    cfg_ir::{cfg_builder::CfgBuilder, cfg_ir::CfgIr},
    error::kaori_error::KaoriError,
    lexer::{lexer::Lexer, token_stream::TokenStream},
    semantic::{hir_decl::HirDecl, resolver::Resolver, type_checker::TypeChecker},
    syntax::{decl::Decl, parser::Parser},
    virtual_machine::kaori_vm::run_vm,
};

fn run_lexical_analysis(source: String) -> Result<TokenStream, KaoriError> {
    let mut lexer = Lexer::new(&source);
    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, lexer.tokens);
    Ok(token_stream)
}

fn run_syntax_analysis(token_stream: TokenStream) -> Result<Vec<Decl>, KaoriError> {
    let mut parser = Parser::new(token_stream);

    let ast = parser.parse()?;

    Ok(ast)
}

fn run_semantic_analysis(ast: &mut [Decl]) -> Result<Vec<HirDecl>, KaoriError> {
    let mut resolver = Resolver::default();

    let hir = resolver.resolve(ast)?;

    let mut type_checker = TypeChecker::default();

    type_checker.type_check(&hir)?;

    Ok(hir)
}

fn build_cfg_ir(hir: &[HirDecl]) -> CfgIr {
    let mut cfg_builder = CfgBuilder::default();

    cfg_builder.build_ir(hir);

    cfg_builder.cfg_ir
}

fn run_lifetime_analyis(cfg_ir: &CfgIr) {
    /*  let mut a = LivenessAnalysis::new(cfg_stream);

    a.analyze_cfgs(); */
}

fn generate_bytecode(cfg_ir: &CfgIr) -> Bytecode {
    let mut generator = BytecodeGenerator::new();

    generator.generate(cfg_ir)
}

pub fn compile_source_code(source: String) -> Result<Bytecode, KaoriError> {
    let token_stream = run_lexical_analysis(source)?;
    let mut ast = run_syntax_analysis(token_stream)?;
    let hir = run_semantic_analysis(&mut ast)?;
    let cfg_ir = build_cfg_ir(&hir);

    //run_lifetime_analyis(&cfg_ir);
    let bytecode = generate_bytecode(&cfg_ir);

    Ok(bytecode)
}

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let bytecode = compile_source_code(source)?;

    for instruction in &bytecode.instructions {
        println!("{instruction}");
    }

    let start = Instant::now();

    //run_vm(bytecode.instructions, bytecode.constants);
    println!("Time elapsed: {:#?}", start.elapsed());

    Ok(())
}
