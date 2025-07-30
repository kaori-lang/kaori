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

    pub fn emit(&mut self, instruction: Instruction) {
        self.bytecode.push(instruction);
    }

    pub fn emit_jump_if_false(&mut self, index: usize) {
        self.bytecode[index] = Instruction::JumpIfFalse(self.bytecode.len());
    }

    pub fn emit_jump(&mut self, index: usize) {
        self.bytecode[index] = Instruction::Jump(self.bytecode.len());
    }
}
