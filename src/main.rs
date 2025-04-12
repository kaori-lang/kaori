#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use yellow_flash::{
    interpreter::interpreter::Interpreter,
    lexer::Lexer,
    parser::{self, Parser},
};

fn main() {
    let source = r#"2*7; 
    2 + 4;
    5+9;
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let Ok(t) = tokens else {
        panic!("yes panic");
    };

    let mut parser = Parser::new(t);
    let ast = parser.parse();
    if let Ok(ast) = ast {
        let interpreter = Interpreter::new(ast);
        println!("{:?}", interpreter.interpret());
    }
}
