#![allow(clippy::new_without_default)]

use std::fs;

use kaori::{
    compiler::{
        lexer::{lexer::Lexer, token_stream::TokenStream},
        semantic::{resolver::Resolver, type_checker::TypeChecker, visitor::Visitor},
        syntax::parser::Parser,
    },
    error::kaori_error::KaoriError,
};

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let mut lexer = Lexer::new(source.clone());

    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source.clone(), tokens);

    let mut parser = Parser::new(token_stream);

    let mut declarations = parser.declarations()?;

    let mut resolver = Resolver::new();
    resolver.run(&mut declarations)?;

    let mut type_checker = TypeChecker::new();

    type_checker.run(&mut declarations)?;
    Ok(())
}

fn main() {
    if let Ok(source) = fs::read_to_string("src/code/main.kaori") {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
        }
    }
}
