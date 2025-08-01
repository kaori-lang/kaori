use crate::backend::codegen::{bytecode::Bytecode, instruction::Instruction};

use super::{value::Value, value_stack::ValueStack};

pub struct KaoriVM {
    bytecode: Bytecode,
}

impl KaoriVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode }
    }

    pub fn execute_instructions(&self) {
        let index = 0;
        let mut stack: ValueStack = ValueStack::default();

        while index < self.bytecode.instructions.len() {
            match self.bytecode.instructions[index] {
                Instruction::Plus => {
                    let right = stack.pop();
                    let left = stack.pop();

                    stack.push(Value::number(left.as_number() + right.as_number()));
                }
                _ => (),
            }
        }
    }
}
