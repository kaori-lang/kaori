use crate::{
    runtime::{instruction::Const, value::Value},
    util::string_interner::Symbol,
};

use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub frame_size: usize,
    pub arity: usize,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "FRAME_SIZE: {}", self.frame_size)?;
        writeln!(f, "ARITY: {}", self.arity)?;

        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }

        writeln!(f)?;
        Ok(())
    }
}

impl Function {
    pub fn new(arity: usize) -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            frame_size: 0,
            arity,
        }
    }

    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    fn get_or_insert(&mut self, value: Value) -> u16 {
        if let Some(index) = self.constants.iter().copied().position(|c| c == value) {
            return index as u16;
        }

        let index = self.constants.len();
        self.constants.push(value);

        index as u16
    }

    pub fn store_string_const(&mut self, value: Symbol) -> Const {
        let index = self.get_or_insert(Value::string(value));

        Const(index)
    }

    pub fn store_number_const(&mut self, value: f64) -> Const {
        let index = self.get_or_insert(Value::number(value));

        Const(index)
    }

    pub fn store_nil_const(&mut self) -> Const {
        let index = self.get_or_insert(Value::nil());

        Const(index)
    }

    pub fn store_boolean_const(&mut self, value: bool) -> Const {
        let index = self.get_or_insert(Value::bool(value));

        Const(index)
    }

    pub fn store_function_const(&mut self, value: usize) -> Const {
        let index = self.get_or_insert(Value::function(value));

        Const(index)
    }
}
