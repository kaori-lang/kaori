use crate::frontend::codegen::{bytecode::Bytecode, instruction::Instruction, value::Value};

pub struct KaoriVM {
    bytecode: Bytecode,
}

pub struct ValueStack {
    index: usize,
    values: [Value; 1000],
}

impl ValueStack {
    pub fn new() -> Self {
        Self {
            index: 0,
            values: [Value::default(); 1000],
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values[self.index] = value;

        self.index += 1;
    }

    pub fn pop(&mut self) -> Value {
        let value = self.values[self.index];

        self.index -= 1;

        value
    }
}

impl KaoriVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode }
    }

    pub fn execute_instructions(&self) {
        let index = 0;
        let mut stack: Vec<Value> = Vec::new();

        while index < self.bytecode.instructions.len() {
            match self.bytecode.instructions[index] {
                Instruction::Plus => {
                    let right = stack.pop();
                    let left = stack.pop();
                }
            }
        }
    }
}
