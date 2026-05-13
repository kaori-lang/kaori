use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub arity: u8,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
