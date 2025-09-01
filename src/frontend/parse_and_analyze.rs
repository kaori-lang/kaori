use crate::{error::kaori_error::KaoriError, frontend::hir::hir_gen::generate_hir};

use super::{
    scanner::{lexer::Lexer, token_stream::TokenStream},
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

    let hir = generate_hir(&ast);

    let mut resolution_table = ResolutionTable::default();
    let mut resolver = Resolver::new(&mut resolution_table);

    resolver.resolve(&hir)?;

    let mut type_checker = TypeChecker::new(&mut resolution_table);

    type_checker.check(&hir)?;

    Ok(())
}
