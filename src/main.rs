#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::fs;

use regex::{Captures, Regex};
use yellow_flash::{
    interpreter::interpreter::Interpreter, lexer::lexer::Lexer, parser::parser::Parser,
    yf_error::YFError,
};

pub fn run_program(source: String) -> Result<(), YFError> {
    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.execute()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&statements)?;

    Ok(())
}

fn main() {
    let source = r#"
        
        float a = 5;
              

        float b = 7
        ;

        bool c = a >= b;
        print("{a:f} {b:f} {c:f}");
        
        "#;

    match run_program(source.to_string()) {
        Err(error) => println!("{}", error),
        _ => (),
    };

    /*   match fs::read_to_string("src/source.yf") {
        Ok(source) => match run_program(source) {
            Err(error) => println!("{}", error),
            _ => (),
        },
        Err(error) => println!("{}", error),
    } */
}
