use crate::backend::codegen::{bytecode::Bytecode, instruction::Opcode};

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
            let instruction = unsafe {
                self.bytecode
                    .instructions
                    .get_unchecked(self.instruction_ptr)
            };

            match instruction.opcode {
                Opcode::Plus => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() + right.as_number()));
                }
                Opcode::Minus => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() - right.as_number()));
                }
                Opcode::Multiply => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() * right.as_number()));
                }
                Opcode::Divide => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() / right.as_number()));
                }
                Opcode::Modulo => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::number(left.as_number() % right.as_number()));
                }
                Opcode::And => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_bool() && right.as_bool()));
                }
                Opcode::Or => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_bool() || right.as_bool()));
                }
                Opcode::NotEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() != right.as_number()));
                }
                Opcode::Equal => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() == right.as_number()));
                }
                Opcode::Greater => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() > right.as_number()));
                }
                Opcode::GreaterEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() >= right.as_number()));
                }
                Opcode::Less => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() < right.as_number()));
                }
                Opcode::LessEqual => {
                    let right = value_stack.pop();
                    let left = value_stack.pop();

                    value_stack.push(Value::boolean(left.as_number() <= right.as_number()));
                }
                Opcode::Not => {
                    let value = value_stack.pop();

                    value_stack.push(Value::boolean(!value.as_bool()));
                }
                Opcode::Negate => {
                    let value = value_stack.pop();

                    value_stack.push(Value::number(-value.as_number()));
                }
                Opcode::Print => {
                    let value = value_stack.pop();

                    println!("{:?}", value.as_number());
                }
                Opcode::LoadConst => {
                    let value = self.bytecode.constant_pool[instruction.operand];

                    value_stack.push(value);
                }
                Opcode::Declare => {
                    let value = value_stack.pop();

                    self.callstack.declare(value);
                }
                Opcode::StoreGlobal => {
                    let value = value_stack.pop();

                    self.callstack.store_global(value, instruction.operand);
                }
                Opcode::LoadGlobal => {
                    let value = self.callstack.load_global(instruction.operand);

                    value_stack.push(value);
                }
                Opcode::EnterScope => self.callstack.enter_scope(),
                Opcode::ExitScope => self.callstack.exit_scope(),
                Opcode::Jump => {
                    self.instruction_ptr = instruction.operand;
                    continue;
                }
                Opcode::JumpIfFalse => {
                    let value = value_stack.pop();

                    if !value.as_bool() {
                        self.instruction_ptr = instruction.operand;
                        continue;
                    }
                }
                _ => (),
            }

            self.instruction_ptr += 1;
        }
    }
}
