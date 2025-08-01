use crate::backend::codegen::{
    bytecode::{self, Bytecode},
    instruction::Instruction,
};

use super::{value::Value, value_stack::ValueStack};

pub struct KaoriVM {
    bytecode: Bytecode,
}

impl KaoriVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode }
    }

    pub fn execute_instructions(&self) {
        let mut index = 0;
        let mut stack: ValueStack = ValueStack::default();

        while index < self.bytecode.instructions.len() {
            match self.bytecode.instructions[index] {
                Instruction::Plus => {
                    let right = stack.pop();
                    let left = stack.pop();

                    stack.push(Value::number(left.as_number() + right.as_number()));
                }
                Instruction::Minus => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::number(left.as_number() - right.as_number()));
                }
                Instruction::Multiply => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::number(left.as_number() * right.as_number()));
                }
                Instruction::Divide => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::number(left.as_number() / right.as_number()));
                }
                Instruction::Modulo => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::number(left.as_number() % right.as_number()));
                }
                Instruction::And => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_bool() && right.as_bool()));
                }
                Instruction::Or => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_bool() || right.as_bool()));
                }
                Instruction::NotEqual => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_number() != right.as_number()));
                }
                Instruction::Equal => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_number() == right.as_number()));
                }
                Instruction::Greater => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_number() > right.as_number()));
                }
                Instruction::GreaterEqual => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_number() >= right.as_number()));
                }
                Instruction::Less => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_number() < right.as_number()));
                }
                Instruction::LessEqual => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(Value::boolean(left.as_number() <= right.as_number()));
                }
                Instruction::Not => {
                    let value = stack.pop();
                    stack.push(Value::boolean(!value.as_bool()));
                }
                Instruction::Negate => {
                    let value = stack.pop();
                    stack.push(Value::number(-value.as_number()));
                }
                Instruction::Print => {
                    let value = stack.pop();
                    println!("{:?}", value.as_number());
                }
                Instruction::LoadConst(i) => {
                    let value = self.bytecode.constant_pool[i];
                    stack.push(value);
                }
                _ => (),
            }

            index += 1;
        }
    }
}
