use crate::{runtime::value::Value, util::string_interner::StringIndex};

use super::instruction::Instruction;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub registers: u8,
    pub arity: u8,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Function {
    pub fn emit_nil(&mut self) -> u8 {
        let dest = self.allocate_register();

        let src = self.push_number(0.0);

        self.instructions.push(Instruction::LoadK {
            dest,
            src: src as u16,
        });

        dest
    }

    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        index
    }

    pub fn allocate_register(&mut self) -> u8 {
        let register = self.registers;

        self.registers += 1;

        register
    }

    fn get_or_insert(&mut self, value: Value) -> usize {
        if let Some(index) = self.constants.iter().copied().position(|c| c == value) {
            return index;
        }

        let index = self.constants.len();
        self.constants.push(value);

        index
    }

    pub fn push_string(&mut self, value: StringIndex) -> usize {
        self.get_or_insert(Value::string(value))
    }

    pub fn push_number(&mut self, value: f64) -> usize {
        self.get_or_insert(Value::number(value))
    }

    pub fn push_unit(&mut self) -> usize {
        self.get_or_insert(Value::number(0.0))
    }
}
