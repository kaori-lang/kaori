#![allow(clippy::missing_safety_doc)]

use std::hint::unreachable_unchecked;

use crate::{backend::codegen::instruction::Instruction, error::kaori_error::KaoriError};

use super::{register::Register, value::Value};

pub struct Interpreter {
    instruction_ptr: usize,
    instructions: Vec<Instruction>,
    constant_pool: Vec<Value>,
    values: Vec<Value>,
    function_frames: Vec<FunctionFrame>,
}

pub struct FunctionFrame {
    pub return_address: usize,
    pub locals: Register,
}

impl Default for FunctionFrame {
    fn default() -> Self {
        Self {
            return_address: usize::MAX,
            locals: Register::default(),
        }
    }
}

impl FunctionFrame {
    fn new(return_address: usize) -> Self {
        Self {
            return_address,
            locals: Register::default(),
        }
    }
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constant_pool: Vec<Value>) -> Self {
        Self {
            instruction_ptr: 0,
            instructions,
            constant_pool,
            values: Vec::with_capacity(64),
            function_frames: vec![FunctionFrame::default()],
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
                Instruction::Call(arguments) => unsafe { self.op_call(arguments as usize) },
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
            let current_frame = self.function_frames.last_mut().unwrap_unchecked();

            let value = self.values.last().unwrap_unchecked();

            current_frame.locals.store_local(*value, offset);
        }
    }

    pub unsafe fn op_load_local(&mut self, offset: usize) {
        unsafe {
            let current_frame = self.function_frames.last().unwrap_unchecked();

            let value = current_frame.locals.load_local(offset);
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

    pub unsafe fn op_call(&mut self, arguments: usize) {
        unsafe {
            let value = self.values.pop().unwrap_unchecked();

            let function_ref = value.as_function_ref();
            let return_address = self.instruction_ptr;

            let mut frame = FunctionFrame::new(return_address);

            for offset in (0..arguments).rev() {
                let value = self.values.pop().unwrap_unchecked();
                frame.locals.store_local(value, offset);
            }

            self.function_frames.push(frame);

            self.instruction_ptr = function_ref - 1;
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
