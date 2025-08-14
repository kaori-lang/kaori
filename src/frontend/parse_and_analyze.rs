use crate::error::kaori_error::KaoriError;

use super::{
    scanner::{lexer::Lexer, token_stream::TokenStream},
    semantic::{resolved_decl::ResolvedDecl, resolver::Resolver, type_checker::TypeChecker},
    syntax::parser::Parser,
};

pub fn parse_and_analyze(source: String) -> Result<Vec<ResolvedDecl>, KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let mut declarations = parser.parse()?;

    let mut resolver = Resolver::new();

    let resolved_declarations = resolver.resolve(&mut declarations)?;

    let type_checker = TypeChecker::new();

    type_checker.check(&resolved_declarations)?;

    Ok(resolved_declarations)
}
