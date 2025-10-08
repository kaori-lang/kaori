use std::hint::unreachable_unchecked;

use crate::{
    bytecode::{instruction::Instruction, value::Value},
    cfg_ir::operand::Register,
};

use super::call_stack::CallStack;

pub struct KaoriVM {
    call_stack: CallStack,
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    registers: Vec<Value>,
    instruction_index: usize,
}

impl KaoriVM {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Value>) -> Self {
        let return_address = instructions.len();

        Self {
            call_stack: CallStack::new(return_address),
            instructions,
            constants,
            registers: vec![Value::default(); 1024],
            instruction_index: 0,
        }
    }

    pub fn run(&mut self) {}

    pub fn get_value(&self, register: i16) -> &Value {
        if register < 0 {
            &self.constants[-register as usize]
        } else {
            &self.registers[register as usize]
        }
    }

    pub fn set_value(&mut self, register: i16, value: Value) {
        self.registers[register.0 as usize] = value;
    }

    #[inline(never)]
    fn instruction_move(&mut self, dest: Register, src: Register) {
        let value = *self.get_value(src);
        self.set_value(dest, value);

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_add(&mut self, instruction_index: usize) {
        let Instruction::Add { dest, src1, src2 } = self.instructions[instruction_index] else {
            unsafe {
                unreachable_unchecked();
            }
        };

        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::number(lhs.as_number() + rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_subtract(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::number(lhs.as_number() - rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_multiply(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::number(lhs.as_number() * rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_divide(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::number(lhs.as_number() / rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_modulo(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::number(lhs.as_number() % rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_equal(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::boolean(lhs.as_number() == rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_not_equal(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::boolean(lhs.as_number() != rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_greater(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::boolean(lhs.as_number() > rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_greater_equal(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::boolean(lhs.as_number() >= rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_less(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::boolean(lhs.as_number() < rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_less_equal(&mut self, instruction: &Instruction) {
        let lhs = self.get_value(src1);
        let rhs = self.get_value(src2);
        self.set_value(dest, Value::boolean(lhs.as_number() <= rhs.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_not(&mut self, dest: Register, src: Register) {
        let value = self.get_value(src);
        self.set_value(dest, Value::boolean(!value.as_boolean()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_negate(&mut self, dest: Register, src: Register) {
        let value = self.get_value(src);
        self.set_value(dest, Value::number(-value.as_number()));

        self.instruction_index += 1;
    }

    #[inline(never)]
    fn instruction_jump(&mut self, offset: i16) {
        self.instruction_index = (self.instruction_index as i16 + offset) as usize;
    }

    #[inline(never)]
    fn instruction_conditional_jump(&mut self, src: Register, true_offset: i16, false_offset: i16) {
        let value = self.get_value(src);

        if value.as_boolean() {
            self.instruction_index = (self.instruction_index as i16 + true_offset) as usize;
        } else {
            self.instruction_index = (self.instruction_index as i16 + false_offset) as usize;
        }
    }

    #[inline(never)]
    fn instruction_print(&mut self, src: Register) {
        let value = self.get_value(src);

        println!("", value.a);
        self.instruction_index += 1;
    }
}
