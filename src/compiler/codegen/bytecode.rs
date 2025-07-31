#![allow(clippy::new_without_default)]
use super::{instruction::Instruction, value::Value};

#[derive(Debug)]
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

        while index < self.constant_pool.len() {
            match (self.constant_pool.get(index), &value) {
                (Some(Value::Str(left)), Value::Str(right)) => {
                    if left == right {
                        break;
                    }
                }
                (Some(Value::Bool(left)), Value::Bool(right)) => {
                    if left == right {
                        break;
                    }
                }
                (Some(Value::Number(left)), Value::Number(right)) => {
                    if left == right {
                        break;
                    }
                }
                _ => {}
            };

            index += 1;
        }

        if index == self.constant_pool.len() {
            self.constant_pool.push(value);
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
