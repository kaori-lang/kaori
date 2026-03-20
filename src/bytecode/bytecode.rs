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

use std::fmt;

impl fmt::Display for Bytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (pc, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", pc, instr)?;
        }

        Ok(())
    }
}
