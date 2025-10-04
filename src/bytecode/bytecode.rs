use super::{instruction::Instruction, value::Value};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>, constant_pool: Vec<Value>) -> Self {
        Self {
            instructions,
            constant_pool,
        }
    }
}
