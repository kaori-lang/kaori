use foldhash::{HashMap, HashMapExt};

use crate::{
    mir::instruction::{ConstIndex, Register},
    runtime::value::Value,
    util::string_interner::StringIndex,
};

use super::instruction::Instruction;
use std::{
    fmt::{self, Display, Formatter},
    ops::Range,
};

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub live_ranges: HashMap<Register, Range<usize>>,
    pub arity: u8,
    pub next_register: usize,
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
            live_ranges: HashMap::new(),
            arity,
            next_register: 0,
        }
    }

    pub fn update_live_range(&mut self, register: Register, index: usize) {
        if let Some(range) = self.live_ranges.get_mut(&register) {
            *range = range.start..(index + 1);
        }
    }

    pub fn remove_live_range(&mut self, register: Register) {
        self.live_ranges.remove(&register);
    }

    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        let mut live = |register: Register| self.update_live_range(register, index);

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
            Instruction::AddK { dest, src1, .. }
            | Instruction::SubtractRK { dest, src1, .. }
            | Instruction::MultiplyK { dest, src1, .. }
            | Instruction::DivideRK { dest, src1, .. }
            | Instruction::ModuloRK { dest, src1, .. }
            | Instruction::EqualK { dest, src1, .. }
            | Instruction::NotEqualK { dest, src1, .. }
            | Instruction::LessK { dest, src1, .. }
            | Instruction::LessEqualK { dest, src1, .. }
            | Instruction::GreaterK { dest, src1, .. }
            | Instruction::GreaterEqualK { dest, src1, .. } => {
                live(dest);
                live(src1);
            }
            Instruction::SubtractKR { dest, src2, .. }
            | Instruction::DivideKR { dest, src2, .. }
            | Instruction::ModuloKR { dest, src2, .. } => {
                live(dest);
                live(src2);
            }
            Instruction::Not { dest, src }
            | Instruction::Negate { dest, src }
            | Instruction::Move { dest, src }
            | Instruction::CaptureValue { dest, src } => {
                live(dest);
                live(src);
            }
            Instruction::LoadK { dest, .. }
            | Instruction::CreateDict { dest }
            | Instruction::CreateClosure { dest, .. } => {
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
            Instruction::Call { dest, src, .. } => {
                live(dest);
                live(src);
            }
            Instruction::Return { src } => {
                live(src);
            }
            Instruction::JumpIfFalse { src, .. } | Instruction::JumpIfTrue { src, .. } => {
                live(src);
            }
            Instruction::JumpIfLess { src1, src2, .. }
            | Instruction::JumpIfLessEqual { src1, src2, .. }
            | Instruction::JumpIfGreater { src1, src2, .. }
            | Instruction::JumpIfGreaterEqual { src1, src2, .. }
            | Instruction::JumpIfEqual { src1, src2, .. }
            | Instruction::JumpIfNotEqual { src1, src2, .. } => {
                live(src1);
                live(src2);
            }
            Instruction::JumpIfLessK { src1, .. }
            | Instruction::JumpIfLessEqualK { src1, .. }
            | Instruction::JumpIfGreaterK { src1, .. }
            | Instruction::JumpIfGreaterEqualK { src1, .. }
            | Instruction::JumpIfEqualK { src1, .. }
            | Instruction::JumpIfNotEqualK { src1, .. } => {
                live(src1);
            }
            Instruction::Jump { .. } | Instruction::Nop => {}
        }

        index
    }

    pub fn allocate_register(&mut self) -> Register {
        let register = self.next_register;
        let start = self.instructions.len();

        self.live_ranges
            .insert(Register(register as i16), start..start + 1);

        self.next_register += 1;

        Register(register as i16)
    }

    fn get_or_insert(&mut self, value: Value) -> u16 {
        if let Some(index) = self.constants.iter().copied().position(|c| c == value) {
            return index as u16;
        }

        let index = self.constants.len();
        self.constants.push(value);

        index as u16
    }

    pub fn push_string(&mut self, value: StringIndex) -> ConstIndex {
        let index = self.get_or_insert(Value::string(value));

        ConstIndex(index)
    }

    pub fn push_number(&mut self, value: f64) -> ConstIndex {
        let index = self.get_or_insert(Value::number(value));

        ConstIndex(index)
    }

    pub fn emit_nil(&mut self) -> Register {
        let dest = self.allocate_register();

        let src = self.push_number(0.0);

        self.emit_instruction(Instruction::LoadK { dest, src });

        dest
    }
}
