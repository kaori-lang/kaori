#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use yellow_flash::{
    lexer::Lexer,
    parser::{self, Parser},
};

fn main() {
    let source = r#"Number a = 10.91;
    print("hello world"); 
    Number b = 125*12.52+5;
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let Ok(t) = tokens else {
        panic!("yes panic");
    };

    //println!("{:#?}", &t);
    let mut parser = Parser::new(t);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
