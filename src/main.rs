#![allow(dead_code)] // Suppresses warnings about unused structs, enums, functions, and traits
#![allow(unused_variables)] // Suppresses warnings about unused variables
#![allow(unused_mut)] // Suppresses warnings about unused `mut` (mutable) variables
#![allow(unused_imports)] // Suppresses warnings about unused `use` statements
#![allow(unused_assignments)] // Suppresses warnings about variables being assigned but never used

use yellow_flash::{lexer::Lexer, parser::Parser};

fn main() {
    let lex: Lexer = Lexer::new("2. 5 / *   + 5");
    let tokens = lex.get_tokens();
    let parser = Parser::new(tokens);

    println!("{:#?}", parser.show_tokens());
    println!("{:#?}", parser.show_tokens());
}
