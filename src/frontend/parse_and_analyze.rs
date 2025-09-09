use crate::error::kaori_error::KaoriError;

use super::{
    hir::hir_gen::HirGen,
    lexer::{lexer::Lexer, token_stream::TokenStream},
    semantic::{resolution_table::ResolutionTable, resolver::Resolver, type_checker::TypeChecker},
    syntax::parser::Parser,
};

pub fn parse_and_analyze(source: String) -> Result<(), KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let ast = parser.parse()?;

    let mut resolution_table = ResolutionTable::default();
    let hir = HirGen::new(&mut resolution_table);

    hir.generate_hir(&ast);

    let type_checker = TypeChecker::new(&mut resolution_table);

    //type_checker.check(&hir)?;

    Ok(())
}
