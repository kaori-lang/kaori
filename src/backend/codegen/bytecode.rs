#![allow(clippy::new_without_default)]
use super::{
    instruction::Instruction,
    value::{ConstValue, Value},
};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub temp_const_pool: Vec<ConstValue>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            temp_const_pool: Vec::new(),
        }
    }

    pub fn get_const_pool(&self) -> Vec<Value> {
        self.temp_const_pool.iter().map(|c| c.to_value()).collect()
    }

    pub fn index(&self) -> usize {
        self.instructions.len()
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn emit_constant(&mut self, other: ConstValue) {
        let mut index = 0;

        while index < self.temp_const_pool.len() {
            let current = &self.temp_const_pool[index];

            if current.equal(&other) {
                break;
            }

            index += 1;
        }

        if index == self.temp_const_pool.len() {
            self.temp_const_pool.push(other);
        }

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
