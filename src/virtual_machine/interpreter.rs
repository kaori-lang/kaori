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
    constants: Vec<Value>,
    registers: [Value; 4096],
    instruction_index: usize,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Value>) -> Self {
        let return_address = instructions.len();

        Self {
            call_stack: CallStack::new(return_address),
            instructions,
            constants,
            registers: [Value::default(); 4096],
            instruction_index: 0,
        }
    }

    #[inline(always)]
    pub fn get_value(&self, register: Register) -> &Value {
        if register.0 < 0 {
            &self.constants[(-register.0) as usize]
        } else {
            &self.registers[register.0 as usize]
        }
    }

    #[inline(always)]
    pub fn set_value(&mut self, register: Register, value: Value) {
        self.registers[register.0 as usize] = value;
    }

    #[inline(always)]
    fn instr_move(&mut self, dest: Register, src: Register) {
        let value = self.get_value(src);
        self.set_value(dest, *value);
    }

    #[inline(always)]
    fn instr_add(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::number(lhs.as_number() + rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_subtract(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::number(lhs.as_number() - rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_multiply(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::number(lhs.as_number() * rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_divide(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::number(lhs.as_number() / rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_modulo(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::number(lhs.as_number() % rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_equal(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::boolean(lhs.as_number() == rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_not_equal(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::boolean(lhs.as_number() != rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_greater(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::boolean(lhs.as_number() > rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_greater_equal(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::boolean(lhs.as_number() >= rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_less(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::boolean(lhs.as_number() < rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_less_equal(&mut self, dest: Register, src1: Register, src2: Register) {
        let lhs = *self.get_value(src1);
        let rhs = *self.get_value(src2);
        let value = unsafe { Value::boolean(lhs.as_number() <= rhs.as_number()) };
        self.set_value(dest, value);
    }

    #[inline(always)]
    fn instr_jump(&mut self, offset: i16) {
        self.instruction_index = (self.instruction_index as i16 + offset) as usize;
    }

    #[inline(always)]
    fn instr_conditional_jump(&mut self, src: Register, true_offset: i16, false_offset: i16) {
        let value = self.get_value(src);

        unsafe {
            if value.as_boolean() {
                self.instruction_index = (self.instruction_index as i16 + true_offset) as usize;
            } else {
                self.instruction_index = (self.instruction_index as i16 + false_offset) as usize;
            }
        }
    }

    #[inline(always)]
    fn dispatch_instruction(&mut self) {
        match self.instructions[self.instruction_index] {
            Instruction::Move { dest, src } => self.instr_move(dest, src),
            Instruction::Add { dest, src1, src2 } => self.instr_add(dest, src1, src2),
            Instruction::Subtract { dest, src1, src2 } => self.instr_subtract(dest, src1, src2),
            Instruction::Multiply { dest, src1, src2 } => self.instr_multiply(dest, src1, src2),
            Instruction::Divide { dest, src1, src2 } => self.instr_divide(dest, src1, src2),
            Instruction::Modulo { dest, src1, src2 } => self.instr_modulo(dest, src1, src2),
            Instruction::Equal { dest, src1, src2 } => self.instr_equal(dest, src1, src2),
            Instruction::NotEqual { dest, src1, src2 } => self.instr_not_equal(dest, src1, src2),
            Instruction::Greater { dest, src1, src2 } => self.instr_greater(dest, src1, src2),
            Instruction::GreaterEqual { dest, src1, src2 } => {
                self.instr_greater_equal(dest, src1, src2)
            }
            Instruction::Less { dest, src1, src2 } => self.instr_less(dest, src1, src2),
            Instruction::LessEqual { dest, src1, src2 } => self.instr_less_equal(dest, src1, src2),
            Instruction::Negate { dest, src } => todo!(),
            Instruction::Not { dest, src } => todo!(),
            Instruction::Call => todo!(),
            Instruction::Return { src } => self.instruction_index = self.instructions.len(),
            Instruction::Jump { offset } => {
                self.instr_jump(offset);
            }
            Instruction::ConditionalJump {
                src,
                true_offset,
                false_offset,
            } => {
                self.instr_conditional_jump(src, true_offset, false_offset);
            }
            Instruction::Print { src } => {
                let value = self.get_value(src);

                unsafe {
                    println!("{:#?}", value.as_number());
                }
            }
        }
    }

    pub fn execute_instructions(&mut self) -> Result<(), KaoriError> {
        let size = self.instructions.len();

        while self.instruction_index < size {
            match self.instructions[self.instruction_index] {
                Instruction::Move { dest, src } => self.instr_move(dest, src),
                Instruction::Add { dest, src1, src2 } => self.instr_add(dest, src1, src2),
                Instruction::Subtract { dest, src1, src2 } => self.instr_subtract(dest, src1, src2),
                Instruction::Multiply { dest, src1, src2 } => self.instr_multiply(dest, src1, src2),
                Instruction::Divide { dest, src1, src2 } => self.instr_divide(dest, src1, src2),
                Instruction::Modulo { dest, src1, src2 } => self.instr_modulo(dest, src1, src2),
                Instruction::Equal { dest, src1, src2 } => self.instr_equal(dest, src1, src2),
                Instruction::NotEqual { dest, src1, src2 } => {
                    self.instr_not_equal(dest, src1, src2)
                }
                Instruction::Greater { dest, src1, src2 } => self.instr_greater(dest, src1, src2),
                Instruction::GreaterEqual { dest, src1, src2 } => {
                    self.instr_greater_equal(dest, src1, src2)
                }
                Instruction::Less { dest, src1, src2 } => self.instr_less(dest, src1, src2),
                Instruction::LessEqual { dest, src1, src2 } => {
                    self.instr_less_equal(dest, src1, src2)
                }
                Instruction::Negate { dest, src } => todo!(),
                Instruction::Not { dest, src } => todo!(),
                Instruction::Call => todo!(),
                Instruction::Return { src } => self.instruction_index = self.instructions.len(),
                Instruction::Jump { offset } => {
                    self.instr_jump(offset);
                    continue;
                }
                Instruction::ConditionalJump {
                    src,
                    true_offset,
                    false_offset,
                } => {
                    self.instr_conditional_jump(src, true_offset, false_offset);
                    continue;
                }
                Instruction::Print { src } => {
                    let value = self.get_value(src);

                    unsafe {
                        println!("{:#?}", value.as_number());
                    }
                }
            }

            self.instruction_index += 1;
        }

        Ok(())
    }
}
