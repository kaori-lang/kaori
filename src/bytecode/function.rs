use crate::{
    bytecode::instruction::Const, runtime::value::Value, util::string_interner::StringIndex,
};

use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub arity: u8,
    pub frame_size: u8,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "ARITY: {}", self.arity)?;

        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }

        writeln!(f)?;
        Ok(())
    }
}

impl Function {
    pub fn new(arity: u8) -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            arity,
            frame_size: 0,
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

    pub fn push_string(&mut self, value: StringIndex) -> Const {
        let index = self.get_or_insert(Value::string(value));

        Const(index)
    }

    pub fn push_number(&mut self, value: f64) -> Const {
        let index = self.get_or_insert(Value::number(value));

        Const(index)
    }
}
