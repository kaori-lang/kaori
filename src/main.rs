#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::fs;

use kaori::{
    compiler::{
        lexer::{lexer::Lexer, token_stream::TokenStream},
        syntax::parser::Parser,
    },
    error::compilation_error::CompilationError,
};
use regex::{Captures, Regex};

pub fn run_program(source: String) -> Result<(), CompilationError> {
    let mut lexer = Lexer::new(source.clone());

    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source.clone(), tokens);

    let mut parser = Parser::new(token_stream);

    let declarations = parser.declarations()?;
    println!("{:#?}", declarations);
    Ok(())
}

fn main() {
    if let Ok(source) = fs::read_to_string("src/code/main.kaori") {
        match run_program(source) {
            Err(error) => println!("{:#?}", error),
            Ok(()) => (),
        };
    }
}
