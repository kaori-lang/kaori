use std::fmt;

use crate::bytecode::environment::Register;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Const(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reg(pub u8);

impl From<Register> for Reg {
    fn from(register: Register) -> Self {
        let register = match register {
            Register::Local(register) => register,
            Register::Temp(register) => register,
        };

        Reg(register as u8)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Instruction {
    Add { dest: Reg, src1: Reg, src2: Reg },
    AddK { dest: Reg, src1: Reg, src2: Const },
    Subtract { dest: Reg, src1: Reg, src2: Reg },
    SubtractRK { dest: Reg, src1: Reg, src2: Const },
    SubtractKR { dest: Reg, src1: Const, src2: Reg },
    Multiply { dest: Reg, src1: Reg, src2: Reg },
    MultiplyK { dest: Reg, src1: Reg, src2: Const },
    Divide { dest: Reg, src1: Reg, src2: Reg },
    DivideRK { dest: Reg, src1: Reg, src2: Const },
    DivideKR { dest: Reg, src1: Const, src2: Reg },
    Modulo { dest: Reg, src1: Reg, src2: Reg },
    ModuloRK { dest: Reg, src1: Reg, src2: Const },
    ModuloKR { dest: Reg, src1: Const, src2: Reg },
    Equal { dest: Reg, src1: Reg, src2: Reg },
    EqualK { dest: Reg, src1: Reg, src2: Const },
    NotEqual { dest: Reg, src1: Reg, src2: Reg },
    NotEqualK { dest: Reg, src1: Reg, src2: Const },
    Less { dest: Reg, src1: Reg, src2: Reg },
    LessK { dest: Reg, src1: Reg, src2: Const },
    LessEqual { dest: Reg, src1: Reg, src2: Reg },
    LessEqualK { dest: Reg, src1: Reg, src2: Const },
    Greater { dest: Reg, src1: Reg, src2: Reg },
    GreaterK { dest: Reg, src1: Reg, src2: Const },
    GreaterEqual { dest: Reg, src1: Reg, src2: Reg },
    GreaterEqualK { dest: Reg, src1: Reg, src2: Const },
    Not { dest: Reg, src: Reg },
    Negate { dest: Reg, src: Reg },
    Move { dest: Reg, src: Reg },
    LoadConst { dest: Reg, src: Const },
    CreateMap { dest: Reg },
    SetField { object: Reg, key: Reg, value: Reg },
    GetField { dest: Reg, object: Reg, key: Reg },
    CreateClosure { dest: Reg, captures: u8 },
    CaptureValue { src: Reg },
    CreateCell { dest: Reg, src: Reg },
    SetCell { dest: Reg, src: Reg },
    GetCell { dest: Reg, src: Reg },
    Call { dest: Reg, src: Reg },
    Return { src: Reg },
    Jump { offset: i32 },
    JumpIfFalse { src: Reg, offset: i32 },
    JumpIfTrue { src: Reg, offset: i32 },
    JumpIfLess { src1: Reg, src2: Reg, offset: i32 },
    JumpIfLessK { src1: Reg, src2: Const, offset: i32 },
    JumpIfLessEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfLessEqualK { src1: Reg, src2: Const, offset: i32 },
    JumpIfGreater { src1: Reg, src2: Reg, offset: i32 },
    JumpIfGreaterK { src1: Reg, src2: Const, offset: i32 },
    JumpIfGreaterEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfGreaterEqualK { src1: Reg, src2: Const, offset: i32 },
    JumpIfEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfEqualK { src1: Reg, src2: Const, offset: i32 },
    JumpIfNotEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfNotEqualK { src1: Reg, src2: Const, offset: i32 },
}

impl Instruction {
    pub fn discriminant(&self) -> usize {
        unsafe { *(self as *const Instruction as *const u8) as usize }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}", self.0)
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => write!(f, "ADD {} {} {}", dest, src1, src2),
            Self::AddK { dest, src1, src2 } => write!(f, "ADD {} {} {}", dest, src1, src2),
            Self::Subtract { dest, src1, src2 } => write!(f, "SUB {} {} {}", dest, src1, src2),
            Self::SubtractRK { dest, src1, src2 } => write!(f, "SUB {} {} {}", dest, src1, src2),
            Self::SubtractKR { dest, src1, src2 } => write!(f, "SUB {} {} {}", dest, src1, src2),
            Self::Multiply { dest, src1, src2 } => write!(f, "MUL {} {} {}", dest, src1, src2),
            Self::MultiplyK { dest, src1, src2 } => write!(f, "MUL {} {} {}", dest, src1, src2),
            Self::Divide { dest, src1, src2 } => write!(f, "DIV {} {} {}", dest, src1, src2),
            Self::DivideRK { dest, src1, src2 } => write!(f, "DIV {} {} {}", dest, src1, src2),
            Self::DivideKR { dest, src1, src2 } => write!(f, "DIV {} {} {}", dest, src1, src2),
            Self::Modulo { dest, src1, src2 } => write!(f, "MOD {} {} {}", dest, src1, src2),
            Self::ModuloRK { dest, src1, src2 } => write!(f, "MOD {} {} {}", dest, src1, src2),
            Self::ModuloKR { dest, src1, src2 } => write!(f, "MOD {} {} {}", dest, src1, src2),
            Self::Equal { dest, src1, src2 } => write!(f, "EQ {} {} {}", dest, src1, src2),
            Self::EqualK { dest, src1, src2 } => write!(f, "EQ {} {} {}", dest, src1, src2),
            Self::NotEqual { dest, src1, src2 } => write!(f, "NEQ {} {} {}", dest, src1, src2),
            Self::NotEqualK { dest, src1, src2 } => write!(f, "NEQ {} {} {}", dest, src1, src2),
            Self::Less { dest, src1, src2 } => write!(f, "LT {} {} {}", dest, src1, src2),
            Self::LessK { dest, src1, src2 } => write!(f, "LT {} {} {}", dest, src1, src2),
            Self::LessEqual { dest, src1, src2 } => write!(f, "LTE {} {} {}", dest, src1, src2),
            Self::LessEqualK { dest, src1, src2 } => write!(f, "LTE {} {} {}", dest, src1, src2),
            Self::Greater { dest, src1, src2 } => write!(f, "GT {} {} {}", dest, src1, src2),
            Self::GreaterK { dest, src1, src2 } => write!(f, "GT {} {} {}", dest, src1, src2),
            Self::GreaterEqual { dest, src1, src2 } => write!(f, "GTE {} {} {}", dest, src1, src2),
            Self::GreaterEqualK { dest, src1, src2 } => write!(f, "GTE {} {} {}", dest, src1, src2),
            Self::Not { dest, src } => write!(f, "NOT {} {}", dest, src),
            Self::Negate { dest, src } => write!(f, "NEG {} {}", dest, src),
            Self::Move { dest, src } => write!(f, "MOV {} {}", dest, src),
            Self::LoadConst { dest, src } => write!(f, "LOAD_CONST {} {}", dest, src),
            Self::CreateMap { dest } => write!(f, "CREATE_MAP {}", dest),
            Self::SetField { object, key, value } => {
                write!(f, "SET_FIELD {} {} {}", object, key, value)
            }
            Self::GetField { dest, object, key } => {
                write!(f, "GET_FIELD {} {} {}", dest, object, key)
            }
            Self::CreateClosure { dest, captures } => {
                write!(f, "CREATE_CLOSURE {} CAPTURES: {}", dest, captures)
            }
            Self::CaptureValue { src } => write!(f, "CAPTURE_VALUE  {}", src),
            Self::CreateCell { dest, src } => write!(f, "CREATE_CELL {} {}", dest, src),
            Self::SetCell { dest, src } => write!(f, "SET_CELL {} {}", dest, src),
            Self::GetCell { dest, src } => write!(f, "GET_CELL {} {}", dest, src),
            Self::Call { dest, src } => write!(f, "CALL {} {}", dest, src),
            Self::Return { src } => write!(f, "RET {}", src),
            Self::Jump { offset } => write!(f, "JMP {}", offset),
            Self::JumpIfTrue { src, offset } => write!(f, "JMP_IF_TRUE {} {}", src, offset),
            Self::JumpIfFalse { src, offset } => write!(f, "JMP_IF_FALSE {} {}", src, offset),
            Self::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessK { src1, src2, offset } => {
                write!(f, "JMP_IF_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterK { src1, src2, offset } => {
                write!(f, "JMP_IF_GT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ {} {} {}", src1, src2, offset)
            }
            Self::JumpIfEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ {} {} {}", src1, src2, offset)
            }
        }
    }
}
