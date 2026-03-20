use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub ip: *const Instruction,
    pub frame_size: u8,
    pub constant_pool: Vec<Value>,
}

impl Function {
    pub fn new(ip: *const Instruction, frame_size: u8, constant_pool: Vec<Value>) -> Self {
        Self {
            ip,
            frame_size,
            constant_pool,
        }
    }
}
