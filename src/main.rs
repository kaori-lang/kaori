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
    let statements = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&statements)?;

    Ok(())
}

fn main() {
    let mut source = String::from(
        r#"
        String a = "hello world";
        print(a);
        Number b = 1;
        Number c = 2;

        print(b + c);
        b = c = 2.5;
        print(b + c);

        {   
            print("entrando no block scope!");
            Number d = 11.5;
            print(d);
            print(b);
            print(c);
        }

        print(d);
        "#,
    );

    match run_program(source) {
        Err(error) => println!("{:}", error),
        Ok(_) => (),
    }
}
