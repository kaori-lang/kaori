use crate::error::kaori_error::KaoriError;

use super::{
    resolver::resolver::Resolver,
    scanner::{lexer::Lexer, token_stream::TokenStream},
    syntax::{ast_node::AstNode, parser::Parser},
    type_checker::type_checker::TypeChecker,
};

pub fn parse_and_analyze(source: String) -> Result<Vec<AstNode>, KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let nodes = parser.parse()?;

    let mut resolver = Resolver::new();

    let resolved_nodes = resolver.resolve(&nodes)?;

    let mut type_checker = TypeChecker::new();

    type_checker.check(&resolved_nodes)?;

    Ok(nodes)
}
