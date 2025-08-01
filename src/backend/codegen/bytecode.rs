#![allow(clippy::new_without_default)]
use super::{instruction::Instruction, value::Value};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
        }
    }
}
