use std::fmt::{self, Display, Formatter};

use super::{constants::Constant, instruction::Instruction};

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constants: Vec<Constant>,
}

impl Function {
    pub fn new(
        instructions: Vec<Instruction>,
        registers_count: u8,
        constants: Vec<Constant>,
    ) -> Self {
        Self {
            instructions,
            registers_count,
            constants,
        }
    }
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
