#![allow(clippy::new_without_default)]
use super::instruction::Instruction;

pub struct Bytecode {
    pub bytecode: Vec<Instruction>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    pub fn place_holder(&mut self) -> usize {
        let index = self.bytecode.len();

        self.bytecode.push(Instruction::Nothing);

        index
    }

    pub fn index(&self) -> usize {
        self.bytecode.len()
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.bytecode.push(instruction);
    }

    pub fn update_placeholder(&mut self, index: usize, instruction: Instruction) {
        self.bytecode[index] = instruction;
    }
}
