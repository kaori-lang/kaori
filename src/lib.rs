#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]
#![feature(likely_unlikely)]
#![feature(rust_preserve_none_cc)]

pub mod codegen;
pub mod diagnostics;
pub mod syntax;

pub mod compiler;
pub mod runtime;

pub mod std;
pub mod util;
