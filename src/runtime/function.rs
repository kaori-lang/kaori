use std::fmt::{self, Display, Formatter};

use crate::{
    mir,
    runtime::{instruction::Instruction, value::Value},
};

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub frame_size: u8,
    pub arity: u8,
}

impl Function {
    pub fn new(
        instructions: Vec<mir::Instruction>,
        constants: Vec<Value>,
        frame_size: u16,
        arity: u8,
    ) -> Self {
        Self {
            instructions: todo!(),
            constants,
            frame_size: frame_size as u8,
            arity,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "ARITY: {}", self.arity)?;
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
