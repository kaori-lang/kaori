#![allow(clippy::missing_safety_doc)]

use crate::{bytecode::instruction::Instruction, error::kaori_error::KaoriError};

use super::value::Value;

pub struct Interpreter {
    instruction_ptr: usize,
    instructions: Vec<Instruction>,
    constant_pool: Vec<Value>,
    registers: Vec<Value>,
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
            registers: Vec::with_capacity(1024),
            function_frames: vec![main_frame],
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let size = self.instructions.len();

        while self.instruction_ptr < size {
            let instruction = unsafe { self.instructions.get_unchecked(self.instruction_ptr) };

            match instruction {
                Instruction::Add { dest, src1, src2 } => {
                    let lhs = self.registers[src1.0 as usize];
                    let rhs = self.registers[src2.0 as usize];

                    unsafe {
                        self.registers[dest.0 as usize] =
                            Value::number(lhs.as_number() + rhs.as_number());
                    }
                }
                Instruction::Subtract { dest, src1, src2 } => todo!(),
                Instruction::Multiply { dest, src1, src2 } => todo!(),
                Instruction::Divide { dest, src1, src2 } => todo!(),
                Instruction::Modulo { dest, src1, src2 } => todo!(),
                Instruction::Equal { dest, src1, src2 } => todo!(),
                Instruction::NotEqual { dest, src1, src2 } => todo!(),
                Instruction::Greater { dest, src1, src2 } => todo!(),
                Instruction::GreaterEqual { dest, src1, src2 } => todo!(),
                Instruction::Less { dest, src1, src2 } => todo!(),
                Instruction::LessEqual { dest, src1, src2 } => todo!(),
                Instruction::And { dest, src1, src2 } => todo!(),
                Instruction::Or { dest, src1, src2 } => todo!(),
                Instruction::Negate { dest, src } => todo!(),
                Instruction::Not { dest, src } => todo!(),
                Instruction::LoadConst { dest, src } => todo!(),
                Instruction::Move { dest, src } => todo!(),
                Instruction::Call => todo!(),
                Instruction::Return { src } => todo!(),
                Instruction::Jump(offset) => todo!(),
                Instruction::JumpFalse(offset) => todo!(),
                Instruction::Print { src } => {
                    let value = self.registers[src.0 as usize];

                    println!("{value:#?}");
                }
            }

            self.instruction_ptr += 1;
        }

        Ok(())
    }

    pub unsafe fn op_add(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_subtract(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_multiply(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_divide(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_modulo(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_and(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_or(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_not_equal(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_equal(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_greater(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_greater_equal(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_less(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_less_equal(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_negate(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_not(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_load_const(&mut self, index: usize) {
        unsafe {}
    }

    #[cold]
    pub unsafe fn op_print(&mut self) {
        unsafe {}
    }

    pub unsafe fn op_jump_false(&mut self, index: usize) {
        unsafe {}
    }

    pub unsafe fn op_call(&mut self, arguments_size: usize, frame_size: usize) {
        unsafe {}
    }

    pub unsafe fn op_return(&mut self) {
        unsafe {}
    }
}
