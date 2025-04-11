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
    let source = r#"2*7.53; 
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

    for i in ast.unwrap().iter() {
        println!("{:?}", i.eval().unwrap());
    }
}
