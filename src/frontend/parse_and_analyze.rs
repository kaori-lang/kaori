use crate::error::kaori_error::KaoriError;

use super::{
    scanner::{lexer::Lexer, token_stream::TokenStream},
    semantic::resolver::Resolver,
    syntax::{ast_node::AstNode, parser::Parser},
};

pub fn parse_and_analyze(source: String) -> Result<Vec<AstNode>, KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let mut nodes = parser.parse()?;

    let mut resolver = Resolver::new();

    resolver.resolve(&mut nodes)?;

    /*     let mut type_checker = TypeChecker::new();

    type_checker.check(&mut nodes)?; */

    Ok(nodes)
}
