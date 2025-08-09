use crate::error::kaori_error::KaoriError;

use super::{
    scanner::{lexer::Lexer, token_stream::TokenStream},
    semantic::{resolver::Resolver, type_checker::TypeChecker},
    syntax::{declaration::Decl, parser::Parser},
};

pub fn parse_and_analyze(source: String) -> Result<Vec<Decl>, KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let mut declarations = parser.parse()?;

    let mut resolver = Resolver::new();

    resolver.resolve(&mut declarations)?;

    let mut type_checker = TypeChecker::new();

    type_checker.check(&mut declarations)?;

    Ok(declarations)
}
