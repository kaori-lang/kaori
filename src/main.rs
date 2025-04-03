#![allow(dead_code)]
#![allow(unused_variables)]
mod lexer;
mod parser;
use lexer::Lexer;

fn main() {
    let lex: Lexer = Lexer::new("2. 5 / *   + 5");

    println!("{:#?}", lex.get_tokens());
    println!("{:#?}", lex.get_tokens());
}
