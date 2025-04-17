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
        float limit = 100;
        float dividend = 2;

        while (dividend <= limit) {
            bool is_prime = true;
            float divisor = 2;

            while (divisor < dividend && is_prime) {
                if (dividend % divisor == 0) {
                    is_prime = false;
                }

                divisor = divisor + 1;
            }

            if (is_prime) {
                log("{dividend} is prime");
            }

            dividend = dividend + 1;
        }
      
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
