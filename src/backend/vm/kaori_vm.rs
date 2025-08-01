use crate::backend::codegen::{bytecode::Bytecode, instruction::Instruction};

use super::{callstack::Callstack, value::Value, value_stack::ValueStack};

pub struct KaoriVM {
    bytecode: Bytecode,
    callstack: Callstack,
    instruction_ptr: usize,
}

impl KaoriVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            callstack: Callstack::default(),
            instruction_ptr: 0,
        }
    }

    pub fn execute_instructions(&mut self) {
        let mut value_stack: ValueStack = ValueStack::default();

        while self.instruction_ptr < self.bytecode.instructions.len() {
            match self.bytecode.instructions[self.instruction_ptr] {
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
                Instruction::EnterScope => self.callstack.enter_scope(),
                Instruction::ExitScope => self.callstack.exit_scope(),
                Instruction::Jump(ptr) => {
                    self.instruction_ptr = ptr;
                    continue;
                }
                Instruction::JumpIfFalse(ptr) => {
                    let value = value_stack.pop();

                    if !value.as_bool() {
                        self.instruction_ptr = ptr;
                        continue;
                    }
                }
                _ => (),
            }

            self.instruction_ptr += 1;
        }
    }
}
