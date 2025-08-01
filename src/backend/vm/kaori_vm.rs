use crate::backend::codegen::{bytecode::Bytecode, instruction::Instruction};

use super::{callstack::Callstack, value::Value, value_stack::ValueStack};

pub struct KaoriVM {
    bytecode: Bytecode,
    callstack: Callstack,
}

impl KaoriVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            callstack: Callstack::default(),
        }
    }

    pub fn execute_instructions(&mut self) {
        let mut index = 0;
        let mut value_stack: ValueStack = ValueStack::default();

        while index < self.bytecode.instructions.len() {
            match self.bytecode.instructions[index] {
                Instruction::Plus => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() + right.as_number()));
                }
                Instruction::Minus => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() - right.as_number()));
                }
                Instruction::Multiply => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() * right.as_number()));
                }
                Instruction::Divide => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() / right.as_number()));
                }
                Instruction::Modulo => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() % right.as_number()));
                }
                Instruction::And => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_bool() && right.as_bool()));
                }
                Instruction::Or => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_bool() || right.as_bool()));
                }
                Instruction::NotEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() != right.as_number()));
                }
                Instruction::Equal => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() == right.as_number()));
                }
                Instruction::Greater => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() > right.as_number()));
                }
                Instruction::GreaterEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() >= right.as_number()));
                }
                Instruction::Less => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() < right.as_number()));
                }
                Instruction::LessEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() <= right.as_number()));
                }
                Instruction::Not => {
                    let value = value_stack.pop();

                    value_stack.push(Value::boolean(!value.as_bool()));
                }
                Instruction::Negate => {
                    let value = value_stack.pop();

                    value_stack.push(Value::number(-value.as_number()));
                }
                Instruction::Print => {
                    let value = value_stack.pop();

                    println!("{:?}", value.as_number());
                }
                Instruction::LoadConst(i) => {
                    let value = self.bytecode.constant_pool[i];

                    value_stack.push(value);
                }
                Instruction::Declare => {
                    let value = value_stack.pop();

                    self.callstack.declare(value);
                }
                Instruction::StoreGlobal(offset) => {
                    let value = value_stack.pop();

                    self.callstack.store_global(value, offset);
                }
                Instruction::LoadGlobal(offset) => {
                    let value = self.callstack.load_global(offset);

                    value_stack.push(value);
                }
                _ => (),
            }

            index += 1;
        }
    }
}
