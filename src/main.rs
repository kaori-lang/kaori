#![allow(dead_code)] // Suppresses warnings about unused structs, enums, functions, and traits
#![allow(unused_variables)] // Suppresses warnings about unused variables
#![allow(unused_mut)] // Suppresses warnings about unused `mut` (mutable) variables
#![allow(unused_imports)] // Suppresses warnings about unused `use` statements
#![allow(unused_assignments)] // Suppresses warnings about variables being assigned but never used

use std::{iter::Peekable, str::Chars, vec};

use yellow_flash::{lexer::Lexer, parser::Parser};

fn main() {
    let source = "6>=6.1";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    if let Ok(t) = tokens {
        let mut parser = Parser::new(t);
        let ast = parser.get_ast();

        if let Ok(tree) = ast {
            println!("{:#?}", tree.eval());
        } else {
            println!("{:#?}", ast);
        }
    } else {
        println!("{:?}", tokens);
    }
}
