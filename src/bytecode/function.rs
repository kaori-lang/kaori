use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub ip: *const Instruction,
    pub frame_size: usize,
    pub constants: Vec<Value>,
}

impl Function {
    pub fn new(ip: *const Instruction, frame_size: usize, constants: Vec<Value>) -> Self {
        Self {
            ip,
            frame_size,
            constants,
        }
    }
}
