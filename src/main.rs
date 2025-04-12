#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use yellow_flash::{
    interpreter::interpreter::Interpreter,
    lexer::{Lexer, LexerError},
    parser::{self, Parser, ParserError},
    program_error::ProgramError,
};

pub fn run_program(source: String) -> Result<(), ProgramError> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements)?;

    Ok(())
}

fn main() {
    let source = String::from(
        r#"Number a = 70 / 2;
        
    Number b = a;
    a +  b;
    "#,
    );

    if let Err(error) = run_program(source) {
        println!("{:?}", error);
    }
}
