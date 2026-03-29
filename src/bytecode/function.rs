use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub start: *const Instruction,
    pub end: *const Instruction,
    pub registers_count: u8,
    pub constant_pool: Vec<Value>,
}

impl Function {
    pub fn new(
        start: *const Instruction,
        end: *const Instruction,
        registers_count: u8,
        constant_pool: Vec<Value>,
    ) -> Self {
        Self {
            start,
            end,
            registers_count,
            constant_pool,
        }
    }
}
