use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub parameters: u8,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "SIZE:  {}", self.registers_count)?;
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
