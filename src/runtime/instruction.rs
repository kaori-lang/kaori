use std::fmt;

use crate::runtime::operands::{Const, Reg};

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
    LessRK { dest: Reg, src1: Reg, src2: Const },
    LessKR { dest: Reg, src1: Const, src2: Reg },
    LessEqual { dest: Reg, src1: Reg, src2: Reg },
    LessEqualRK { dest: Reg, src1: Reg, src2: Const },
    LessEqualKR { dest: Reg, src1: Const, src2: Reg },
    Not { dest: Reg, src: Reg },
    Negate { dest: Reg, src: Reg },
    Move { dest: Reg, src: Reg },
    LoadConst { dest: Reg, src: Const },
    CreateMap { dest: Reg },
    SetProperty { object: Reg, key: Const, value: Reg },
    GetProperty { dest: Reg, object: Reg, key: Const },
    SetElement { object: Reg, key: Reg, value: Reg },
    CreateClosure { dest: Reg, captures: u8, src: u32 },
    CreateRef { dest: Reg, src: Reg },
    DerefSet { dest: Reg, src: Reg },
    Deref { dest: Reg, src: Reg },
    Call { dest: Reg, src: Reg },
    Return { src: Reg },
    Jump { offset: i32 },
    JumpIfFalse { src: Reg, offset: i32 },
    JumpIfTrue { src: Reg, offset: i32 },
    JumpIfLess { src1: Reg, src2: Reg, offset: i32 },
    JumpIfLessRK { src1: Reg, src2: Const, offset: i32 },
    JumpIfLessKR { src2: Reg, src1: Const, offset: i32 },
    JumpIfLessEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfLessEqualRK { src1: Reg, src2: Const, offset: i32 },
    JumpIfLessEqualKR { src2: Reg, src1: Const, offset: i32 },
    JumpIfEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfEqualK { src1: Reg, src2: Const, offset: i32 },
    JumpIfNotEqual { src1: Reg, src2: Reg, offset: i32 },
    JumpIfNotEqualK { src1: Reg, src2: Const, offset: i32 },

    // UNREACHABLE
    CaptureValue { src: Reg },
}

impl Instruction {
    #[inline(always)]
    pub fn discriminant(&self) -> usize {
        unsafe { *(self as *const Instruction as *const u8) as usize }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "ADD {} {} {}", dest, src1, src2)
            }
            Self::AddK { dest, src1, src2 } => {
                write!(f, "ADD {} {} {}", dest, src1, src2)
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "SUB {} {} {}", dest, src1, src2)
            }
            Self::SubtractRK { dest, src1, src2 } => {
                write!(f, "SUB {} {} {}", dest, src1, src2)
            }
            Self::SubtractKR { dest, src1, src2 } => {
                write!(f, "SUB {} {} {}", dest, src1, src2)
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "MUL {} {} {}", dest, src1, src2)
            }
            Self::MultiplyK { dest, src1, src2 } => {
                write!(f, "MUL {} {} {}", dest, src1, src2)
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "DIV {} {} {}", dest, src1, src2)
            }
            Self::DivideRK { dest, src1, src2 } => {
                write!(f, "DIV {} {} {}", dest, src1, src2)
            }
            Self::DivideKR { dest, src1, src2 } => {
                write!(f, "DIV {} {} {}", dest, src1, src2)
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "MOD {} {} {}", dest, src1, src2)
            }
            Self::ModuloRK { dest, src1, src2 } => {
                write!(f, "MOD {} {} {}", dest, src1, src2)
            }
            Self::ModuloKR { dest, src1, src2 } => {
                write!(f, "MOD {} {} {}", dest, src1, src2)
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "EQ {} {} {}", dest, src1, src2)
            }
            Self::EqualK { dest, src1, src2 } => {
                write!(f, "EQ {} {} {}", dest, src1, src2)
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ {} {} {}", dest, src1, src2)
            }
            Self::NotEqualK { dest, src1, src2 } => {
                write!(f, "NEQ {} {} {}", dest, src1, src2)
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "LT {} {} {}", dest, src1, src2)
            }
            Self::LessRK { dest, src1, src2 } => {
                write!(f, "LT {} {} {}", dest, src1, src2)
            }
            Self::LessKR { dest, src1, src2 } => {
                write!(f, "LT {} {} {}", dest, src1, src2)
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE {} {} {}", dest, src1, src2)
            }
            Self::LessEqualRK { dest, src1, src2 } => {
                write!(f, "LTE {} {} {}", dest, src1, src2)
            }
            Self::LessEqualKR { dest, src1, src2 } => {
                write!(f, "LTE {} {} {}", dest, src1, src2)
            }
            Self::Not { dest, src } => write!(f, "NOT {} {}", dest, src),
            Self::Negate { dest, src } => write!(f, "NEG {} {}", dest, src),
            Self::Move { dest, src } => write!(f, "MOV {} {}", dest, src),
            Self::LoadConst { dest, src } => {
                write!(f, "LOAD_CONST {} {}", dest, src)
            }
            Self::CreateMap { dest } => write!(f, "CREATE_MAP {}", dest),
            Self::SetProperty { object, key, value } => {
                write!(f, "SET_PROPERTY {} {} {}", object, key, value)
            }
            Self::GetProperty { dest, object, key } => {
                write!(f, "GET_PROPERTY {} {} {}", dest, object, key)
            }
            Self::SetElement { object, key, value } => {
                write!(f, "SET_ELEMENT {} {} {}", object, key, value)
            }
            Self::CreateClosure { dest, captures, src } => {
                write!(
                    f,
                    "CREATE_CLOSURE {} FUNCTION: {} CAPTURES: {}",
                    dest, src, captures
                )
            }
            Self::CaptureValue { src } => write!(f, "CAPTURE_VALUE {}", src),
            Self::CreateRef { dest, src } => {
                write!(f, "CREATE_REF {} {}", dest, src)
            }
            Self::DerefSet { dest, src } => {
                write!(f, "DEREF_SET {} {}", dest, src)
            }
            Self::Deref { dest, src } => write!(f, "DEREF {} {}", dest, src),
            Self::Call { dest, src } => write!(f, "CALL {} {}", dest, src),
            Self::Return { src } => write!(f, "RET {}", src),
            Self::Jump { offset } => write!(f, "JMP {}", offset),
            Self::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE {} {}", src, offset)
            }
            Self::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE {} {}", src, offset)
            }
            Self::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessRK { src1, src2, offset } => {
                write!(f, "JMP_IF_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessKR { src1, src2, offset } => {
                write!(f, "JMP_IF_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqualRK { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqualKR { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE {} {} {}", src1, src2, offset)
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
