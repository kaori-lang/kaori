#![allow(clippy::missing_safety_doc)]

use crate::{
    bytecode::{constant_pool::ConstantPool, instruction::Instruction, value::Value},
    error::kaori_error::KaoriError,
};

use super::{call_stack::CallStack, registers::Registers};

pub struct Interpreter {
    call_stack: CallStack,
    instructions: Vec<Instruction>,
    constant_pool: ConstantPool,
    registers: Registers,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constant_pool: ConstantPool) -> Self {
        let return_address = instructions.len();

        Self {
            instructions,
            constant_pool,
            registers: Registers::new(),
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let mut instruction_index = 0;

        let size = self.instructions.len();

        while instruction_index < size {
            let instruction = self.instructions.get(instruction_index).unwrap();

            match *instruction {
                Instruction::Add { dest, src1, src2 } => {
                    let lhs = self.registers.get_value(src1);
                    let rhs = self.registers.get_value(src2);

                    let value = unsafe { Value::number(lhs.as_number() + rhs.as_number()) };

                    self.registers.set_value(dest, value);
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
                Instruction::Const { dest, src } => {
                    let value = self.constant_pool.get_value(src);

                    self.registers.set_value(dest, value);
                }
                Instruction::Move { dest, src } => {
                    let value = self.registers.get_value(src);

                    self.registers.set_value(dest, value);
                }
                Instruction::Call => todo!(),
                Instruction::Return { src } => {}
                Instruction::Jump { offset } => todo!(),
                Instruction::JumpFalse { src, offset } => todo!(),
                Instruction::Print { src } => {
                    let value = self.registers.get_value(src);

                    println!("{value:#?}");
                }
            }

            instruction_index += 1;
        }

        Ok(())
    }
}
