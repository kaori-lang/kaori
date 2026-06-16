use crate::runtime::value::Value;

use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub frame_size: usize,
    pub arity: usize,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "FRAME_SIZE: {}", self.frame_size)?;
        writeln!(f, "ARITY: {}", self.arity)?;

        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
