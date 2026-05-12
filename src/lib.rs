#![feature(explicit_tail_calls)]
#![feature(f16)]
#![allow(incomplete_features)]
#![feature(likely_unlikely)]
#![feature(rust_preserve_none_cc)]

pub mod bytecode;
pub mod diagnostics;
pub mod syntax;

pub mod program;
pub mod runtime;

pub mod std;
pub mod util;
