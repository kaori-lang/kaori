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
            call_stack: CallStack::new(return_address),
            instructions,
            constant_pool,
            registers: Registers::new(),
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        for instruction in &self.instructions {
            println!("{instruction}");
        }

        let mut instruction_index = self.instructions.len();

        let size = self.instructions.len();

        while instruction_index < size {
            let instruction = &self.instructions[instruction_index];
            //println!("{instruction}");
            match *instruction {
                Instruction::Add { dest, src1, src2 } => {
                    let lhs = self.registers.get_value(src1);
                    let rhs = self.registers.get_value(src2);

                    let value = match (lhs, rhs) {
                        (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                        _ => unreachable!("Add must be run with numbers"),
                    };

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
                Instruction::Less { dest, src1, src2 } => {
                    let lhs = self.registers.get_value(src1);
                    let rhs = self.registers.get_value(src2);

                    let value = match (lhs, rhs) {
                        (Value::Number(left), Value::Number(right)) => Value::Bool(left < right),
                        _ => unreachable!("Less must be run with numbers"),
                    };

                    self.registers.set_value(dest, value);
                }
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
                Instruction::Return { src } => {
                    println!("{:#?}", self.registers.get_value(src));
                    instruction_index = self.instructions.len()
                }
                Instruction::Jump { offset } => {
                    instruction_index = (instruction_index as i16 + offset) as usize;

                    continue;
                }
                Instruction::JumpIfFalse { src, offset } => {
                    if let Value::Bool(value) = self.registers.get_value(src) {
                        if !value {
                            instruction_index = (instruction_index as i16 + offset) as usize;
                            continue;
                        }
                    } else {
                        unreachable!("JumpFalse must run with booleans")
                    }
                }
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
