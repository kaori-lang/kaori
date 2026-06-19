use crate::{
    codegen::operand::Constant,
    runtime::{operands::Const, value::Value},
    util::string_interner::Symbol,
};

use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub frame_size: usize,
    pub arity: usize,
}

impl Function {
    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    fn get_or_insert(&mut self, value: Value) -> Const {
        if let Some(index) =
            self.constants.iter().copied().position(|c| c == value)
        {
            return Const::from(index);
        }

        let index = self.constants.len();
        self.constants.push(value);

        Const::from(index)
    }

    pub fn store_constant(&mut self, constant: Constant) -> Const {
        match constant {
            Constant::Boolean(value) => self.get_or_insert(Value::bool(value)),
            Constant::Nil => self.store_nil_const(),
            Constant::Number(value) => self.get_or_insert(Value::number(value)),
            Constant::String(value) => self.get_or_insert(Value::string(value)),
        }
    }

    pub fn store_string_const(&mut self, value: Symbol) -> Const {
        self.get_or_insert(Value::string(value))
    }

    pub fn store_number_const(&mut self, value: f64) -> Const {
        self.get_or_insert(Value::number(value))
    }

    pub fn store_nil_const(&mut self) -> Const {
        self.get_or_insert(Value::nil())
    }

    pub fn store_native_function_const(&mut self, index: usize) -> Const {
        self.get_or_insert(Value::native_function(index as u32))
    }
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
