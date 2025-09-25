#![allow(clippy::missing_safety_doc)]

use crate::{backend::codegen::instruction::Instruction, error::kaori_error::KaoriError};

use super::{register::Register, value::Value};

pub struct Interpreter {
    instruction_ptr: usize,
    instructions: Vec<Instruction>,
    constant_pool: Vec<Value>,
    values: Vec<Value>,
    register: Register,
    function_frames: Vec<FunctionFrame>,
}

pub struct FunctionFrame {
    pub base_ptr: usize,
    pub return_address: usize,
}

impl FunctionFrame {
    fn new(base_ptr: usize, return_address: usize) -> Self {
        Self {
            base_ptr,
            return_address,
        }
    }
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constant_pool: Vec<Value>) -> Self {
        let main_frame = FunctionFrame::new(0, instructions.len());

        Self {
            instruction_ptr: 0,
            instructions,
            constant_pool,
            values: Vec::with_capacity(64),
            register: Register::default(),
            function_frames: vec![main_frame],
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let size = self.instructions.len();

        while self.instruction_ptr < size {
            let instruction = unsafe { self.instructions.get_unchecked(self.instruction_ptr) };

            match *instruction {
                Instruction::LoadConst(index) => unsafe { self.op_load_const(index as usize) },
                Instruction::StoreLocal(offset) => unsafe { self.op_store_local(offset as usize) },
                Instruction::LoadLocal(offset) => unsafe { self.op_load_local(offset as usize) },
                Instruction::Jump(index) => self.instruction_ptr = index as usize - 1,
                Instruction::JumpIfFalse(index) => unsafe { self.op_jump_if_false(index as usize) },
                Instruction::Pop => unsafe { self.op_pop() },
                Instruction::Add => unsafe { self.op_add() },
                Instruction::Subtract => unsafe { self.op_subtract() },
                Instruction::Multiply => unsafe { self.op_multiply() },
                Instruction::Divide => unsafe { self.op_divide() },
                Instruction::Modulo => unsafe { self.op_modulo() },
                Instruction::And => unsafe { self.op_and() },
                Instruction::Or => unsafe { self.op_or() },
                Instruction::NotEqual => unsafe { self.op_not_equal() },
                Instruction::Equal => unsafe { self.op_equal() },
                Instruction::Greater => unsafe { self.op_greater() },
                Instruction::GreaterEqual => unsafe { self.op_greater_equal() },
                Instruction::Less => unsafe { self.op_less() },
                Instruction::LessEqual => unsafe { self.op_less_equal() },
                Instruction::Not => unsafe { self.op_not() },
                Instruction::Negate => unsafe { self.op_negate() },
                Instruction::Print => unsafe { self.op_print() },
                Instruction::Call {
                    arguments_size,
                    frame_size,
                } => unsafe { self.op_call(arguments_size as usize, frame_size as usize) },
                Instruction::Return => unsafe { self.op_return() },
                _ => todo!(),
            };

            self.instruction_ptr += 1;
        }

        Ok(())
    }

    pub unsafe fn op_add(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::number(left.as_number() + right.as_number()));
        }
    }

    pub unsafe fn op_subtract(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::number(left.as_number() - right.as_number()));
        }
    }

    pub unsafe fn op_multiply(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::number(left.as_number() * right.as_number()));
        }
    }

    pub unsafe fn op_divide(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::number(left.as_number() / right.as_number()));
        }
    }

    pub unsafe fn op_modulo(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::number(left.as_number() % right.as_number()));
        }
    }

    pub unsafe fn op_and(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_bool() && right.as_bool()));
        }
    }

    pub unsafe fn op_or(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_bool() || right.as_bool()));
        }
    }

    pub unsafe fn op_not_equal(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_number() != right.as_number()));
        }
    }

    pub unsafe fn op_equal(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_number() == right.as_number()));
        }
    }

    pub unsafe fn op_greater(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_number() > right.as_number()));
        }
    }

    pub unsafe fn op_greater_equal(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_number() >= right.as_number()));
        }
    }

    pub unsafe fn op_less(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_number() < right.as_number()));
        }
    }

    pub unsafe fn op_less_equal(&mut self) {
        unsafe {
            let right = self.values.pop().unwrap_unchecked();
            let left = self.values.pop().unwrap_unchecked();
            self.values
                .push(Value::boolean(left.as_number() <= right.as_number()));
        }
    }

    pub unsafe fn op_negate(&mut self) {
        unsafe {
            let value = self.values.pop().unwrap_unchecked();
            self.values.push(Value::number(-value.as_number()));
        }
    }

    pub unsafe fn op_not(&mut self) {
        unsafe {
            let value = self.values.pop().unwrap_unchecked();
            self.values.push(Value::boolean(!value.as_bool()));
        }
    }

    pub unsafe fn op_load_const(&mut self, index: usize) {
        unsafe {
            let value = self.constant_pool.get_unchecked(index);
            self.values.push(*value);
        }
    }

    pub unsafe fn op_store_local(&mut self, offset: usize) {
        unsafe {
            let base_ptr = self.function_frames.last().unwrap_unchecked().base_ptr;

            let value = self.values.last().unwrap_unchecked();

            self.register.store_local(*value, base_ptr + offset);
        }
    }

    pub unsafe fn op_load_local(&mut self, offset: usize) {
        unsafe {
            let base_ptr = self.function_frames.last().unwrap_unchecked().base_ptr;
            let value = self.register.load_local(base_ptr + offset);

            self.values.push(*value);
        }
    }
    pub unsafe fn op_pop(&mut self) {
        unsafe { self.values.pop().unwrap_unchecked() };
    }

    #[cold]
    pub unsafe fn op_print(&mut self) {
        unsafe {
            let value = self.values.pop().unwrap_unchecked();

            println!("{value:?}");
        }
    }

    pub unsafe fn op_jump_if_false(&mut self, index: usize) {
        unsafe {
            let value = self.values.pop().unwrap_unchecked();

            if !value.as_bool() {
                self.instruction_ptr = index - 1;
            }
        }
    }

    pub unsafe fn op_call(&mut self, arguments_size: usize, frame_size: usize) {
        unsafe {
            let value = self.values.pop().unwrap_unchecked();

            let instruction_ptr = value.as_instruction_ptr();
            let return_address = self.instruction_ptr;

            let current_frame = self.function_frames.last().unwrap_unchecked();
            let base_ptr = current_frame.base_ptr + frame_size;

            let frame = FunctionFrame::new(base_ptr, return_address);

            for offset in (0..arguments_size).rev() {
                let value = self.values.pop().unwrap_unchecked();
                self.register.store_local(value, base_ptr + offset);
            }

            self.function_frames.push(frame);

            self.instruction_ptr = instruction_ptr - 1;
        }
    }

    pub unsafe fn op_return(&mut self) {
        unsafe {
            let return_address = self
                .function_frames
                .last()
                .unwrap_unchecked()
                .return_address;

            self.instruction_ptr = return_address;

            self.function_frames.pop();
        }
    }
}
