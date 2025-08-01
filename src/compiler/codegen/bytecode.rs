#![allow(clippy::new_without_default)]
use super::{instruction::Instruction, value::Value};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
        }
    }

    pub fn index(&self) -> usize {
        self.instructions.len()
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let mut index = 0;

        self.emit(Instruction::LoadConst(index));
    }

    pub fn create_placeholder(&mut self) -> usize {
        let index = self.instructions.len();

        self.instructions.push(Instruction::Nothing);

        index
    }

    pub fn update_placeholder(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }
}
