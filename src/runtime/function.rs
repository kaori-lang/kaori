use std::fmt::{self, Display, Formatter};

use crate::runtime::{instruction::Instruction, value::Value};
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub frame_size: u8,
    pub arity: u8,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "ARITY: {}", self.arity)?;
        writeln!(f, "FRAME_SIZE: {}", self.frame_size)?;
        writeln!(f, "CONSTANTS: {}", self.constants.len())?;
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
