#![allow(dead_code)] // Suppresses warnings about unused structs, enums, functions, and traits
#![allow(unused_variables)] // Suppresses warnings about unused variables
#![allow(unused_mut)] // Suppresses warnings about unused `mut` (mutable) variables
#![allow(unused_imports)] // Suppresses warnings about unused `use` statements
#![allow(unused_assignments)] // Suppresses warnings about variables being assigned but never used

use yellow_flash::{ast::ast_node::ASTNode, lexer::Lexer, parser::Parser};

fn main() {
    let source = String::from("25.964*9==5<=1==10");
    let lex: Lexer = Lexer::new(source);
    let tokens = lex.get_tokens();
    let mut parser = Parser::new(tokens);

    let ast = parser.get_ast();

    println!("{:#?}", ast);
}
