use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constant_pool: Vec<Value>,
}

impl Function {
    pub fn new(
        instructions: Vec<Instruction>,
        registers_count: u8,
        constant_pool: Vec<Value>,
    ) -> Self {
        Self {
            instructions,
            registers_count,
            constant_pool,
        }
    }
}
