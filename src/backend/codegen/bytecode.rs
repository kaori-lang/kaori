use crate::backend::vm::value::Value;

use super::{const_value::ConstValue, instruction::Instruction};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
}

impl Bytecode {
    pub fn new(instructions: &[Instruction], constant_pool: &[ConstValue]) -> Self {
        Self {
            instructions: instructions.to_vec(),
            constant_pool: constant_pool.iter().map(|v| v.to_union()).collect(),
        }
    }
}
