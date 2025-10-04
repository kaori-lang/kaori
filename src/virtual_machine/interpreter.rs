#![allow(clippy::missing_safety_doc)]

use crate::{
    bytecode::{instruction::Instruction, value::Value},
    cfg_ir::operand::Register,
    error::kaori_error::KaoriError,
};

use super::call_stack::CallStack;

pub struct Interpreter {
    call_stack: CallStack,
    instructions: Vec<Instruction>,
    registers: Vec<Value>,
    register_offset: i16,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, mut constants: Vec<Value>) -> Self {
        let return_address = instructions.len();
        let register_offset = constants.len() as i16;

        constants.resize(1024, Value::default());

        Self {
            call_stack: CallStack::new(return_address),
            instructions,
            registers: constants,
            register_offset,
        }
    }

    pub fn get_value(&mut self, register: Register) -> Value {
        self.registers[(register.0 + self.register_offset) as usize]
    }

    pub fn set_value(&mut self, register: Register, value: Value) {
        self.registers[(register.0 + self.register_offset) as usize] = value;
    }

    pub fn op_add(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);

        let value = match (lhs, rhs) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            _ => unreachable!("Add must be run with numbers"),
        };

        self.set_value(dest, value);
    }

    pub fn op_less(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);

        let value = match (lhs, rhs) {
            (Value::Number(left), Value::Number(right)) => Value::Bool(left < right),
            _ => unreachable!("Less must be run with numbers"),
        };

        self.set_value(dest, value);
    }

    pub fn op_move(&mut self, dest: Register, src: Register) {
        let value = self.get_value(src);

        self.set_value(dest, value);
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let mut instruction_index = 0;

        let size = self.instructions.len();

        while instruction_index < size {
            let instruction = &self.instructions[instruction_index];

            match *instruction {
                Instruction::Move { dest, src } => {
                    self.op_move(dest, src);
                }
                Instruction::Add { dest, src1, src2 } => {
                    self.op_add(dest, src1, src2);
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
                    self.op_less(dest, src1, src2);
                }
                Instruction::LessEqual { dest, src1, src2 } => todo!(),
                Instruction::And { dest, src1, src2 } => todo!(),
                Instruction::Or { dest, src1, src2 } => todo!(),
                Instruction::Negate { dest, src } => todo!(),
                Instruction::Not { dest, src } => todo!(),
                Instruction::Call => todo!(),
                Instruction::Return { src } => instruction_index = self.instructions.len(),
                Instruction::Jump { offset } => {
                    instruction_index = (instruction_index as i16 + offset) as usize;

                    continue;
                }
                Instruction::JumpIfFalse { src, offset } => {
                    if let Value::Bool(value) = self.get_value(src) {
                        if !value {
                            instruction_index = (instruction_index as i16 + offset) as usize;
                            continue;
                        }
                    } else {
                        unreachable!("JumpFalse must run with booleans")
                    }
                }
                Instruction::Print { src } => {
                    let value = self.get_value(src);

                    println!("{value:#?}");
                }
            }

            instruction_index += 1;
        }

        Ok(())
    }
}
