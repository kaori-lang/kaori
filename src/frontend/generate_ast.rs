use crate::error::kaori_error::KaoriError;

use super::{
    scanner::{lexer::Lexer, token_stream::TokenStream},
    semantic::{resolver::Resolver, type_checker::TypeChecker},
    syntax::{ast_node::ASTNode, parser::Parser},
};

pub fn generate_ast(source: String) -> Result<Vec<ASTNode>, KaoriError> {
    let mut lexer = Lexer::new(source.clone());

    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source.clone(), tokens);

    let mut parser = Parser::new(token_stream);

    let mut nodes = parser.parse()?;

    let mut resolver = Resolver::new();

    resolver.resolve(&mut nodes)?;

    let mut type_checker = TypeChecker::new();

    type_checker.check(&mut nodes)?;

    Ok(nodes)
}
