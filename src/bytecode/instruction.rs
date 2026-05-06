use crate::bytecode::immediate::Imm;
use std::fmt;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Instruction {
    Add { dest: u8, src1: u8, src2: u8 },
    AddI { dest: u8, src1: u8, src2: Imm },
    Subtract { dest: u8, src1: u8, src2: u8 },
    SubtractRI { dest: u8, src1: u8, src2: Imm },
    SubtractIR { dest: u8, src1: Imm, src2: u8 },
    Multiply { dest: u8, src1: u8, src2: u8 },
    MultiplyI { dest: u8, src1: u8, src2: Imm },
    Divide { dest: u8, src1: u8, src2: u8 },
    DivideRI { dest: u8, src1: u8, src2: Imm },
    DivideIR { dest: u8, src1: Imm, src2: u8 },
    Modulo { dest: u8, src1: u8, src2: u8 },
    ModuloRI { dest: u8, src1: u8, src2: Imm },
    ModuloIR { dest: u8, src1: Imm, src2: u8 },
    Equal { dest: u8, src1: u8, src2: u8 },
    EqualI { dest: u8, src1: u8, src2: Imm },
    NotEqual { dest: u8, src1: u8, src2: u8 },
    NotEqualI { dest: u8, src1: u8, src2: Imm },
    Less { dest: u8, src1: u8, src2: u8 },
    LessI { dest: u8, src1: u8, src2: Imm },
    LessEqual { dest: u8, src1: u8, src2: u8 },
    LessEqualI { dest: u8, src1: u8, src2: Imm },
    Greater { dest: u8, src1: u8, src2: u8 },
    GreaterI { dest: u8, src1: u8, src2: Imm },
    GreaterEqual { dest: u8, src1: u8, src2: u8 },
    GreaterEqualI { dest: u8, src1: u8, src2: Imm },
    Not { dest: u8, src: u8 },
    Negate { dest: u8, src: u8 },
    Move { dest: u8, src: u8 },
    MoveArg { dest: u8, src: u8 },
    LoadK { dest: u8, src: u32 },
    LoadImm { dest: u8, src: Imm },
    CreateDict { dest: u8 },
    SetField { object: u8, key: u8, value: u8 },
    SetFieldI { object: u8, key: u8, src: Imm },
    GetField { dest: u8, object: u8, key: u8 },
    CreateClosure { dest: u8, captures: u8, src: u32 },
    CaptureValue { src: u8 },
    Call { dest: u8, src: u8 },
    Return { src: u8 },
    Jump { offset: i32 },
    JumpIfFalse { src: u8, offset: i32 },
    JumpIfTrue { src: u8, offset: i32 },
    JumpIfLess { src1: u8, src2: u8, offset: i32 },
    JumpIfLessI { src1: u8, src2: Imm, offset: i32 },
    JumpIfLessEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfLessEqualI { src1: u8, src2: Imm, offset: i32 },
    JumpIfGreater { src1: u8, src2: u8, offset: i32 },
    JumpIfGreaterI { src1: u8, src2: Imm, offset: i32 },
    JumpIfGreaterEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfGreaterEqualI { src1: u8, src2: Imm, offset: i32 },
    JumpIfEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfEqualI { src1: u8, src2: Imm, offset: i32 },
    JumpIfNotEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfNotEqualI { src1: u8, src2: Imm, offset: i32 },
    Nop,
}

impl Instruction {
    #[inline(always)]
    pub const fn discriminant(&self) -> usize {
        unsafe { *(self as *const Instruction as *const u8) as usize }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Self::AddI { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} {}", dest, src1, src2)
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Self::SubtractRI { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} {}", dest, src1, src2)
            }
            Self::SubtractIR { dest, src1, src2 } => {
                write!(f, "SUB r{} {} r{}", dest, src1, src2)
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Self::MultiplyI { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} {}", dest, src1, src2)
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Self::DivideRI { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} {}", dest, src1, src2)
            }
            Self::DivideIR { dest, src1, src2 } => {
                write!(f, "DIV r{} {} r{}", dest, src1, src2)
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Self::ModuloRI { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} {}", dest, src1, src2)
            }
            Self::ModuloIR { dest, src1, src2 } => {
                write!(f, "MOD r{} {} r{}", dest, src1, src2)
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Self::EqualI { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} {}", dest, src1, src2)
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Self::NotEqualI { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} {}", dest, src1, src2)
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Self::LessI { dest, src1, src2 } => {
                write!(f, "LT r{} r{} {}", dest, src1, src2)
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Self::LessEqualI { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} {}", dest, src1, src2)
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Self::GreaterI { dest, src1, src2 } => {
                write!(f, "GT r{} r{} {}", dest, src1, src2)
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Self::GreaterEqualI { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} {}", dest, src1, src2)
            }
            Self::Not { dest, src } => {
                write!(f, "NOT r{} r{}", dest, src)
            }
            Self::Negate { dest, src } => {
                write!(f, "NEG r{} r{}", dest, src)
            }
            Self::Move { dest, src } => {
                write!(f, "MOV r{} r{}", dest, src)
            }
            Self::MoveArg { dest, src } => {
                write!(f, "MOV_ARG r{} r{}", dest, src)
            }
            Self::LoadK { dest, src } => {
                write!(f, "LOADK r{} k{}", dest, src)
            }
            Self::LoadImm { dest, src } => {
                write!(f, "LOAD_IMM r{} {}", dest, src)
            }
            Self::CreateDict { dest } => {
                write!(f, "DICT r{}", dest)
            }
            Self::SetField { object, key, value } => {
                write!(f, "SET r{} r{} r{}", object, key, value)
            }
            Self::SetFieldI { object, key, src } => {
                write!(f, "SET r{} r{} {}", object, key, src)
            }
            Self::GetField { dest, object, key } => {
                write!(f, "GET r{} r{} r{}", dest, object, key)
            }
            Self::Call { dest, src } => {
                write!(f, "CALL r{} r{}", dest, src)
            }
            Self::Return { src } => {
                write!(f, "RET r{}", src)
            }
            Self::Jump { offset } => write!(f, "JMP {}", offset),

            Self::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE r{} {}", src, offset)
            }
            Self::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{} {}", src, offset)
            }
            Self::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfLessI { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterI { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} {} {}", src1, src2, offset)
            }
            Self::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} {} {}", src1, src2, offset)
            }
            Self::CreateClosure {
                dest,
                src,
                captures,
            } => {
                write!(
                    f,
                    "CREATE_CLOSURE r{} Function({}) CAPTURES: {}",
                    dest, src, captures
                )
            }
            Self::CaptureValue { src } => {
                write!(f, "CAPTURE_VALUE r{}", src)
            }
            Self::Nop => {
                write!(f, "NOP")
            }
        }
    }
}
