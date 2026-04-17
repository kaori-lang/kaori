#![feature(explicit_tail_calls)]
#![feature(f16)]
#![allow(incomplete_features)]

pub mod ast;
pub mod bytecode;
pub mod error;
pub mod hir;
pub mod lexer;
pub mod program;
pub mod runtime;
