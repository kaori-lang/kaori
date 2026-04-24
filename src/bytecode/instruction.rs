use crate::bytecode::immediate::Imm;
use std::fmt;

#[derive(Clone, Copy)]
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
    LoadK { dest: u8, src: u8 },
    LoadImm { dest: u8, src: Imm },
    CreateDict { dest: u8 },
    SetField { object: u8, key: u8, value: u8 },
    SetFieldI { object: u8, key: u8, src: Imm },
    GetField { dest: u8, object: u8, key: u8 },
    Call { dest: u8, src: u8 },
    CallK { dest: u8, src: u8 },
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
    Print { src: u8 },
    EnterUncheckedBlock,
    ExitUncheckedBlock,
    Nop,
}

impl Instruction {
    pub const fn discriminant(&self) -> usize {
        unsafe { *(self as *const Instruction as *const u8) as usize }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::EnterUncheckedBlock => write!(f, "ENTER_UNCHECKED_BLOCK"),
            Instruction::ExitUncheckedBlock => write!(f, "EXIT_UNCHECKED_BLOCK"),
            Instruction::Add { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::AddI { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} {}", dest, src1, src2)
            }
            Instruction::Subtract { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Instruction::SubtractRI { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} {}", dest, src1, src2)
            }
            Instruction::SubtractIR { dest, src1, src2 } => {
                write!(f, "SUB r{} {} r{}", dest, src1, src2)
            }
            Instruction::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Instruction::MultiplyI { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} {}", dest, src1, src2)
            }
            Instruction::Divide { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Instruction::DivideRI { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} {}", dest, src1, src2)
            }
            Instruction::DivideIR { dest, src1, src2 } => {
                write!(f, "DIV r{} {} r{}", dest, src1, src2)
            }
            Instruction::Modulo { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::ModuloRI { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} {}", dest, src1, src2)
            }
            Instruction::ModuloIR { dest, src1, src2 } => {
                write!(f, "MOD r{} {} r{}", dest, src1, src2)
            }
            Instruction::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::EqualI { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} {}", dest, src1, src2)
            }
            Instruction::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::NotEqualI { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} {}", dest, src1, src2)
            }
            Instruction::Less { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessI { dest, src1, src2 } => {
                write!(f, "LT r{} r{} {}", dest, src1, src2)
            }
            Instruction::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessEqualI { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} {}", dest, src1, src2)
            }
            Instruction::Greater { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterI { dest, src1, src2 } => {
                write!(f, "GT r{} r{} {}", dest, src1, src2)
            }
            Instruction::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterEqualI { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} {}", dest, src1, src2)
            }
            Instruction::Not { dest, src } => {
                write!(f, "NOT r{} r{}", dest, src)
            }
            Instruction::Negate { dest, src } => {
                write!(f, "NEG r{} r{}", dest, src)
            }
            Instruction::Move { dest, src } => {
                write!(f, "MOV r{} r{}", dest, src)
            }
            Instruction::MoveArg { dest, src } => {
                write!(f, "MOV_ARG r{} r{}", dest, src)
            }
            Instruction::LoadK { dest, src } => {
                write!(f, "LOADK r{} k{}", dest, src)
            }
            Instruction::LoadImm { dest, src } => {
                write!(f, "LOAD_IMM r{} {}", dest, src)
            }
            Instruction::CreateDict { dest } => {
                write!(f, "DICT r{}", dest)
            }
            Instruction::SetField { object, key, value } => {
                write!(f, "SET r{} r{} r{}", object, key, value)
            }
            Instruction::SetFieldI { object, key, src } => {
                write!(f, "SET r{} r{} {}", object, key, src)
            }
            Instruction::GetField { dest, object, key } => {
                write!(f, "GET r{} r{} r{}", dest, object, key)
            }
            Instruction::Call { dest, src } => {
                write!(f, "CALL r{} r{}", dest, src)
            }
            Instruction::CallK { dest, src } => {
                write!(f, "CALL r{} k{}", dest, src)
            }
            Instruction::Return { src } => {
                write!(f, "RET r{}", src)
            }
            Instruction::Jump { offset } => write!(f, "JMP {}", offset),

            Instruction::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE r{} {}", src, offset)
            }
            Instruction::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{} {}", src, offset)
            }
            Instruction::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfLessI { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} {} {}", src1, src2, offset)
            }
            Instruction::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfLessEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} {} {}", src1, src2, offset)
            }
            Instruction::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfGreaterI { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} {} {}", src1, src2, offset)
            }
            Instruction::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfGreaterEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} {} {}", src1, src2, offset)
            }
            Instruction::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} {} {}", src1, src2, offset)
            }
            Instruction::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfNotEqualI { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} {} {}", src1, src2, offset)
            }
            Instruction::Print { src } => {
                write!(f, "PRINT r{}", src)
            }
            Instruction::Nop => {
                write!(f, "NOP")
            }
        }
    }
}
