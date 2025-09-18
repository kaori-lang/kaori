use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        cfgir::{basic_block_stream::BasicBlockStream, cfg_builder::CfgBuilder},
        lexer::{lexer::Lexer, token_stream::TokenStream},
        semantic::{hir_decl::HirDecl, resolver::Resolver, type_checker::TypeChecker},
        syntax::{decl::Decl, parser::Parser},
    },
};

fn run_lexical_analysis(source: String) -> Result<TokenStream, KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);
    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);
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

fn build_cfg_ir(hir: &[HirDecl]) -> BasicBlockStream {
    let mut basic_block_stream = BasicBlockStream::default();

    let mut cfg_builder = CfgBuilder::new(&mut basic_block_stream);

    cfg_builder.build_ir(hir);

    basic_block_stream
}

pub fn compile_source_code(source: String) -> Result<(), KaoriError> {
    let token_stream = run_lexical_analysis(source)?;
    let mut ast = run_syntax_analysis(token_stream)?;
    let hir = run_semantic_analysis(&mut ast)?;
    let cfg_ir = build_cfg_ir(&hir);

    Ok(())
}
