use crate::backend::vm::value::Value;

use super::{const_value::ConstValue, instruction::Instruction};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>, constant_pool: Vec<ConstValue>) -> Self {
        Self {
            instructions,
            constant_pool: constant_pool.iter().map(|v| v.to_union()).collect(),
        }
    }
}
