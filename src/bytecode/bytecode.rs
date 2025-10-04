use super::{instruction::Instruction, value::Value};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Value>) -> Self {
        Self {
            instructions,
            constants,
        }
    }
}
