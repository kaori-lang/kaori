use std::hint::unreachable_unchecked;

use crate::{
    backend::codegen::{constant_pool::ConstantPool, instruction::Instruction},
    error::kaori_error::KaoriError,
};

use super::{callstack::Callstack, value::Value};

pub struct Interpreter {
    callstack: Callstack,
    instruction_ptr: usize,
    instructions: Vec<Instruction>,
    constant_pool: ConstantPool,
    values: Vec<Value>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constant_pool: ConstantPool) -> Self {
        Self {
            callstack: Callstack::default(),
            instruction_ptr: 0,
            instructions,
            constant_pool,
            values: Vec::with_capacity(64),
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let size = self.instructions.len();

        while self.instruction_ptr < size {
            let instruction = unsafe { self.instructions.get_unchecked(self.instruction_ptr) };

            match *instruction {
                Instruction::LoadConst(index) => {
                    let value = self.constant_pool.get_constant(index as usize);

                    self.values.push(*value);
                }
                Instruction::StoreLocal(offset) => {
                    let value = unsafe { self.values.last().unwrap_unchecked() };

                    self.callstack.store_local(*value, offset as usize);
                }
                Instruction::LoadLocal(offset) => {
                    let value = self.callstack.load_local(offset as usize);

                    self.values.push(*value);
                }
                Instruction::Pop => {
                    unsafe { self.values.pop().unwrap_unchecked() };
                }
                Instruction::Declare => {
                    let value = unsafe { self.values.pop().unwrap_unchecked() };

                    self.callstack.declare(value);
                }
                Instruction::Plus => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::number(unsafe {
                        left.as_number() + right.as_number()
                    }));
                }
                Instruction::Minus => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::number(unsafe {
                        left.as_number() - right.as_number()
                    }));
                }
                Instruction::Multiply => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::number(unsafe {
                        left.as_number() * right.as_number()
                    }));
                }
                Instruction::Divide => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::number(unsafe {
                        left.as_number() / right.as_number()
                    }));
                }
                Instruction::Modulo => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::number(unsafe {
                        left.as_number() % right.as_number()
                    }));
                }
                Instruction::And => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values
                        .push(Value::boolean(unsafe { left.as_bool() && right.as_bool() }));
                }
                Instruction::Or => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values
                        .push(Value::boolean(unsafe { left.as_bool() || right.as_bool() }));
                }
                Instruction::NotEqual => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::boolean(unsafe {
                        left.as_number() != right.as_number()
                    }));
                }
                Instruction::Equal => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::boolean(unsafe {
                        left.as_number() == right.as_number()
                    }));
                }
                Instruction::Greater => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::boolean(unsafe {
                        left.as_number() > right.as_number()
                    }));
                }
                Instruction::GreaterEqual => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::boolean(unsafe {
                        left.as_number() >= right.as_number()
                    }));
                }
                Instruction::Less => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::boolean(unsafe {
                        left.as_number() < right.as_number()
                    }));
                }
                Instruction::LessEqual => {
                    let right = unsafe { self.values.pop().unwrap_unchecked() };
                    let left = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values.push(Value::boolean(unsafe {
                        left.as_number() <= right.as_number()
                    }));
                }
                Instruction::Not => {
                    let value = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values
                        .push(Value::boolean(unsafe { !value.as_bool() }));
                }
                Instruction::Negate => {
                    let value = unsafe { self.values.pop().unwrap_unchecked() };

                    self.values
                        .push(Value::number(unsafe { -value.as_number() }));
                }
                Instruction::EnterScope => self.callstack.enter_scope(),
                Instruction::ExitScope => self.callstack.exit_scope(),
                Instruction::Jump(offset) => {
                    self.instruction_ptr = (self.instruction_ptr as i16 + offset - 1) as usize
                }
                Instruction::JumpIfFalse(offset) => {
                    let value = unsafe { self.values.pop().unwrap_unchecked() };

                    if unsafe { !value.as_bool() } {
                        self.instruction_ptr = (self.instruction_ptr as i16 + offset - 1) as usize;
                    }
                }
                Instruction::Print => {
                    let value = unsafe { self.values.pop().unwrap_unchecked() };

                    println!("{value:?}");
                }

                _ => unsafe { unreachable_unchecked() },
            };

            self.instruction_ptr += 1;
        }

        Ok(())
    }
}
