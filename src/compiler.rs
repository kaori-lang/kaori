use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        lexer::{lexer::Lexer, token_stream::TokenStream},
        semantic::{resolver::Resolver, type_checker::TypeChecker},
        syntax::parser::Parser,
    },
};

pub fn compiler(source: String) -> Result<(), KaoriError> {
    let mut tokens = Vec::new();

    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let mut ast = parser.parse()?;

    let mut resolver = Resolver::default();

    let hir = resolver.resolve(&mut ast)?;

    let mut type_checker = TypeChecker::default();

    type_checker.check(&hir)?;

    Ok(())
}
