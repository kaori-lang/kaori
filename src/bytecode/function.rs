use crate::{runtime::value::Value, util::string_interner::StringIndex};

use super::instruction::Instruction;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Default)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub registers: u8,
    pub live_ranges: HashMap<u8, (usize, usize)>,
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

    pub fn update_live_range(&mut self, register: u8, index: usize) {
        self.live_ranges
            .entry(register)
            .and_modify(|r| r.1 = index)
            .or_insert((index, index));
    }

    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        let mut live = |register: u8| self.update_live_range(register, index);

        match instruction {
            Instruction::Add { dest, src1, src2 }
            | Instruction::Subtract { dest, src1, src2 }
            | Instruction::Multiply { dest, src1, src2 }
            | Instruction::Divide { dest, src1, src2 }
            | Instruction::Modulo { dest, src1, src2 }
            | Instruction::Equal { dest, src1, src2 }
            | Instruction::NotEqual { dest, src1, src2 }
            | Instruction::Less { dest, src1, src2 }
            | Instruction::LessEqual { dest, src1, src2 }
            | Instruction::Greater { dest, src1, src2 }
            | Instruction::GreaterEqual { dest, src1, src2 } => {
                live(dest);
                live(src1);
                live(src2);
            }
            Instruction::SubtractRK {
                dest,
                src1,
                src2: _,
            }
            | Instruction::DivideRK {
                dest,
                src1,
                src2: _,
            }
            | Instruction::ModuloRK {
                dest,
                src1,
                src2: _,
            } => {
                live(dest);
                live(src1);
            }
            Instruction::DivideKR {
                dest,
                src1: _,
                src2,
            }
            | Instruction::ModuloKR {
                dest,
                src1: _,
                src2,
            } => {
                live(dest);
                live(src2);
            }
            Instruction::Not { dest, src }
            | Instruction::Negate { dest, src }
            | Instruction::Move { dest, src }
            | Instruction::MoveArg { dest, src }
            | Instruction::CaptureValue { dest, src } => {
                live(dest);
                live(src);
            }
            Instruction::CreateDict { dest } | Instruction::CreateClosure { dest, src: _ } => {
                live(dest);
            }
            Instruction::SetField { object, key, value } => {
                live(object);
                live(key);
                live(value);
            }
            Instruction::GetField { dest, object, key } => {
                live(dest);
                live(object);
                live(key);
            }
            Instruction::Call {
                dest,
                src,
                arity: _,
            } => {
                live(dest);
                live(src);
            }
            Instruction::Return { src } => {
                live(src);
            }
            Instruction::JumpIfFalse { src, offset: _ }
            | Instruction::JumpIfTrue { src, offset: _ } => {
                live(src);
            }
            _ => {}
        }

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
