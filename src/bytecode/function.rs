use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub ip: *const Instruction,
    pub frame_size: u8,
    pub constants: Vec<Value>,
}

impl Function {
    pub fn new(ip: *const Instruction, frame_size: u8, constants: Vec<Value>) -> Self {
        Self {
            ip,
            frame_size,
            constants,
        }
    }
}
