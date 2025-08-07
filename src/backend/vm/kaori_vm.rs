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
        let size = self.bytecode.instructions.len();

        while self.instruction_ptr < size {
            match self.bytecode.instructions[self.instruction_ptr] {
                Instruction::LoadConst(offset) => {
                    let value = self.bytecode.constant_pool[offset as usize].to_owned();

                    value_stack.push(value);
                }
                Instruction::StoreGlobal(offset) => {
                    let value = value_stack.pop();

                    self.callstack
                        .store_global(value.to_owned(), offset as usize);
                }
                Instruction::LoadGlobal(offset) => {
                    let value = self.callstack.load_global(offset as usize);

                    value_stack.push(value.to_owned());
                }
                Instruction::Plus => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() + right.as_number() };
                    value_stack.push(Value::number(result));
                }
                Instruction::Minus => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() - right.as_number() };
                    value_stack.push(Value::number(result));
                }
                Instruction::Multiply => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() * right.as_number() };
                    value_stack.push(Value::number(result));
                }
                Instruction::Divide => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() / right.as_number() };
                    value_stack.push(Value::number(result));
                }
                Instruction::Modulo => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() % right.as_number() };
                    value_stack.push(Value::number(result));
                }
                Instruction::And => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_bool() && right.as_bool() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::Or => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_bool() || right.as_bool() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::NotEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() != right.as_number() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::Equal => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() == right.as_number() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::Greater => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() > right.as_number() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::GreaterEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() >= right.as_number() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::Less => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() < right.as_number() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::LessEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    let result = unsafe { left.as_number() <= right.as_number() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::Not => {
                    let value = value_stack.pop();

                    let result = unsafe { !value.as_bool() };
                    value_stack.push(Value::boolean(result));
                }
                Instruction::Negate => {
                    let value = value_stack.pop();

                    let result = unsafe { -value.as_number() };
                    value_stack.push(Value::number(result));
                }

                Instruction::Declare => {
                    let value = value_stack.pop();

                    self.callstack.declare(value.to_owned());
                }

                Instruction::EnterScope => self.callstack.enter_scope(),
                Instruction::ExitScope => self.callstack.exit_scope(),
                Instruction::Jump(offset) => {
                    self.instruction_ptr = (self.instruction_ptr as i16 + offset - 1) as usize
                }

                Instruction::JumpIfFalse(offset) => {
                    let value = value_stack.pop();

                    let jump = offset - 1;

                    if unsafe { !value.as_bool() } {
                        self.instruction_ptr = (self.instruction_ptr as i16 + jump) as usize;
                    }
                }
                Instruction::Print => {
                    let value = value_stack.pop();

                    let number = unsafe { value.as_number() };
                    println!("{:?}", number);
                }
                _ => (),
            }

            self.instruction_ptr += 1;
        }
    }
}
