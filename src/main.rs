#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

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
    let mut source = String::from(
        r#"
        Number a = 1;
        {
    }
            Number b = 3;
            print(b);
            print(a);
        
 
        print(b);
        "#,
    );

    match run_program(source) {
        Err(error) => println!("{:}", error),
        Ok(_) => (),
    }
}
