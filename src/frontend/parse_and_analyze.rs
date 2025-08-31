use crate::error::kaori_error::KaoriError;

use super::{
    hir::hir_gen::HirGen,
    scanner::{lexer::Lexer, token_stream::TokenStream},
    semantic::{resolver::Resolver, type_checker::TypeChecker},
    syntax::parser::Parser,
};

pub fn parse_and_analyze(source: String) -> Result<(), KaoriError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source, &mut tokens);

    lexer.tokenize()?;

    let token_stream = TokenStream::new(source, tokens);

    let mut parser = Parser::new(token_stream);

    let ast = parser.parse()?;

    let hir = HirGen::generate(&ast);
    let mut resolver = Resolver::new();

    let resolved_declarations = resolver.resolve(&mut declarations)?;

    let mut type_checker = TypeChecker::new();

    type_checker.check(&resolved_declarations)?;

    Ok(())
}
