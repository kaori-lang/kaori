use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub start: *const Instruction,
    pub end: *const Instruction,
    pub frame_size: u8,
    pub constant_pool: Vec<Value>,
}

impl Function {
    pub fn new(
        start: *const Instruction,
        end: *const Instruction,
        frame_size: u8,
        constant_pool: Vec<Value>,
    ) -> Self {
        Self {
            start,
            end,
            frame_size,
            constant_pool,
        }
    }
}
