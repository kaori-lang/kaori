use super::{function::Function, instruction::Instruction};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub functions: Vec<Function>,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>, functions: Vec<Function>) -> Self {
        Self {
            instructions,
            functions,
        }
    }
}
