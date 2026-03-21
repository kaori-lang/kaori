#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

pub mod ast;
pub mod bytecode;
pub mod cfg;
pub mod error;
pub mod lexer;
pub mod program;
pub mod semantic;
pub mod virtual_machine;
