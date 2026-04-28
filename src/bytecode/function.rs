use std::fmt::{self, Display, Formatter};

use crate::bytecode::function_scope::Constant;

use super::instruction::Instruction;

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constants: Vec<Constant>,
    pub parameters: u8,
    pub captures: u8,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "CONSTANTS: {:?}", self.constants)?;
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
