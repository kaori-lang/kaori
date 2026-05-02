use std::fmt::{self, Display, Formatter};

use crate::bytecode::function_scope::Constant;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct Function<T> {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constants: Vec<T>,
    pub parameters: u8,
    pub captures: u8,
}

impl Display for Function<Constant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "CONSTANTS: {:?}", self.constants)?;
        writeln!(f, "SIZE: {:?}", self.registers_count)?;
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
