#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use yellow_flash::{lexer::Lexer, parser::Parser, yf_error::YFError};

pub fn run_program(source: String) -> Result<(), YFError> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    println!("{:#?}", statements);
    //let mut interpreter = Interpreter::new();
    //interpreter.interpret(statements)?;

    Ok(())
}

fn main() {
    let source = String::from(
        r#"5 + 2o
        5 + 5;
        2 +/3;
        5;
    "#,
    );

    println!("{:?}", run_program(source));
}
