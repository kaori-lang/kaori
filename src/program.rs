use std::time::Instant;

use crate::{
    bytecode::{bytecode::Bytecode, bytecode_generator::BytecodeGenerator},
    cfg_ir::{cfg_builder::CfgBuilder, cfg_ir::CfgIr},
    error::kaori_error::KaoriError,
    lexer::{lexer::Lexer, token_stream::TokenStream},
    semantic::{hir_ir::HirIr, resolver::Resolver, type_checker::TypeChecker},
    syntax::{decl::Decl, parser::Parser},
    virtual_machine::kaori_vm::run_kaori_vm,
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

fn run_semantic_analysis(ast: &mut [Decl]) -> Result<HirIr, KaoriError> {
    let mut resolver = Resolver::default();

    let declarations = resolver.resolve(ast)?;

    let type_checker = TypeChecker::default();

    let types_table = type_checker.type_check(&declarations)?;

    let hir = HirIr::new(declarations, types_table);

    Ok(hir)
}

fn build_cfg_ir(hir: HirIr) -> CfgIr {
    let types_table = hir.types_table;
    let declarations = hir.declarations;

    let cfg_builder = CfgBuilder::new(types_table);

    cfg_builder.build_ir(&declarations)
}
fn generate_bytecode(cfg_ir: &CfgIr) -> Bytecode {
    let mut generator = BytecodeGenerator::default();

    generator.generate(cfg_ir)
}

pub fn compile_source_code(source: String) -> Result<Bytecode, KaoriError> {
    let token_stream = run_lexical_analysis(source)?;
    let mut ast = run_syntax_analysis(token_stream)?;
    let hir = run_semantic_analysis(&mut ast)?;
    let cfg_ir = build_cfg_ir(hir);

    let bytecode = generate_bytecode(&cfg_ir);

    Ok(bytecode)
}

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let bytecode = compile_source_code(source)?;

    /*    unsafe {
        run_vm(&bytecode.instructions, &bytecode.constants);
    }

    let elapsed = start.elapsed();
    println!("main vm took: {elapsed:?}"); */

    let start = Instant::now();
    run_kaori_vm(bytecode.instructions, bytecode.constants);

    let elapsed = start.elapsed();
    println!("took: {elapsed:?}");

    Ok(())
}
