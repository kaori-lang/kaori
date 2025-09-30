#![allow(clippy::missing_safety_doc)]

use crate::{
    bytecode::{constant_pool::ConstantPool, instruction::Instruction, value::Value},
    error::kaori_error::KaoriError,
};

use super::registers::Registers;

pub struct Interpreter {
    instruction_index: usize,
    instructions: Vec<Instruction>,
    constant_pool: ConstantPool,
    registers: Registers,
    function_frames: Vec<FunctionFrame>,
}

type InstructionIndex = usize;
type RegisterIndex = usize;
pub struct FunctionFrame {
    pub base_address: RegisterIndex,
    pub return_address: InstructionIndex,
}

impl FunctionFrame {
    fn new(base_address: RegisterIndex, return_address: InstructionIndex) -> Self {
        Self {
            base_address,
            return_address,
        }
    }
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constant_pool: ConstantPool) -> Self {
        let main_frame = FunctionFrame::new(0, instructions.len());

        Self {
            instruction_index: 0,
            instructions,
            constant_pool,
            registers: Registers::new(),
            function_frames: vec![main_frame],
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let size = self.instructions.len();

        while self.instruction_index < size {
            let instruction = self.instructions.get(self.instruction_index).unwrap();

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
                Instruction::LoadConst { dest, src } => {
                    let value = self.constant_pool.get_value(src);

                    self.registers.set_value(dest, value);
                }
                Instruction::Move { dest, src } => {
                    let value = self.registers.get_value(src);

                    self.registers.set_value(dest, value);
                }
                Instruction::Call => todo!(),
                Instruction::Return { src } => {
                    let frame = self.function_frames.pop().unwrap();

                    self.instruction_index = frame.return_address;
                }
                Instruction::Jump { offset } => todo!(),
                Instruction::JumpFalse { src, offset } => todo!(),
                Instruction::Print { src } => {
                    let value = self.registers.get_value(src);

                    println!("{value:#?}");
                }
            }

            self.instruction_index += 1;
        }

        Ok(())
    }
}
