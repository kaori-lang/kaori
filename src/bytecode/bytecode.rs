use crate::vm::heap::Heap;

use super::{function::Function, instruction::Instruction};

use std::fmt;
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub functions: Vec<Function>,
    pub heap: Heap,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>, functions: Vec<Function>, heap: Heap) -> Self {
        Self {
            instructions,
            functions,
            heap,
        }
    }
}

impl fmt::Display for Bytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_ptr = self.instructions.as_ptr();

        for (index, function) in self.functions.iter().enumerate() {
            writeln!(f, "FUNCTION {}:", index)?;

            let start_idx = unsafe { function.start.offset_from(base_ptr) as usize };
            let end_idx = unsafe { function.end.offset_from(base_ptr) as usize };

            for instruction in self.instructions[start_idx..end_idx].iter() {
                writeln!(f, "   {}", instruction)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
