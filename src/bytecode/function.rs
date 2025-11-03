use super::{instruction::Instruction, value::Value};

pub struct Function {
    pub ip: *const Instruction,
    pub frame_size: usize,
    pub constants: Vec<Value>,
}
