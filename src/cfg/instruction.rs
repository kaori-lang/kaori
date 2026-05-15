use std::fmt;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Instruction {
    Add { dest: u8, src1: u8, src2: u8 },
    AddK { dest: u8, src1: u8, src2: u16 },
    Subtract { dest: u8, src1: u8, src2: u8 },
    SubtractRK { dest: u8, src1: u8, src2: u16 },
    SubtractKR { dest: u8, src1: u16, src2: u8 },
    Multiply { dest: u8, src1: u8, src2: u8 },
    MultiplyK { dest: u8, src1: u8, src2: u16 },
    Divide { dest: u8, src1: u8, src2: u8 },
    DivideRK { dest: u8, src1: u8, src2: u16 },
    DivideKR { dest: u8, src1: u16, src2: u8 },
    Modulo { dest: u8, src1: u8, src2: u8 },
    ModuloRK { dest: u8, src1: u8, src2: u16 },
    ModuloKR { dest: u8, src1: u16, src2: u8 },
    Equal { dest: u8, src1: u8, src2: u8 },
    EqualK { dest: u8, src1: u8, src2: u16 },
    NotEqual { dest: u8, src1: u8, src2: u8 },
    NotEqualK { dest: u8, src1: u8, src2: u16 },
    Less { dest: u8, src1: u8, src2: u8 },
    LessK { dest: u8, src1: u8, src2: u16 },
    LessEqual { dest: u8, src1: u8, src2: u8 },
    LessEqualK { dest: u8, src1: u8, src2: u16 },
    Greater { dest: u8, src1: u8, src2: u8 },
    GreaterK { dest: u8, src1: u8, src2: u16 },
    GreaterEqual { dest: u8, src1: u8, src2: u8 },
    GreaterEqualK { dest: u8, src1: u8, src2: u16 },
    Not { dest: u8, src: u8 },
    Negate { dest: u8, src: u8 },
    Move { dest: u8, src: u8 },
    MoveArg { dest: u8, src: u8 },
    LoadK { dest: u8, src: u16 },
    CreateDict { dest: u8 },
    SetField { object: u8, key: u8, value: u8 },
    GetField { dest: u8, object: u8, key: u8 },
    CreateClosure { dest: u8, src: u32 },
    CaptureValue { dest: u8, src: u8 },
    Call { dest: u8, src: u8, arity: u8 },
    Return { src: u8 },
    Jump { offset: i32 },
    JumpIfFalse { src: u8, offset: i32 },
    JumpIfTrue { src: u8, offset: i32 },
    JumpIfLess { src1: u8, src2: u8, offset: i32 },
    JumpIfLessK { src1: u8, src2: u16, offset: i32 },
    JumpIfLessEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfLessEqualK { src1: u8, src2: u16, offset: i32 },
    JumpIfGreater { src1: u8, src2: u8, offset: i32 },
    JumpIfGreaterK { src1: u8, src2: u16, offset: i32 },
    JumpIfGreaterEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfGreaterEqualK { src1: u8, src2: u16, offset: i32 },
    JumpIfEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfEqualK { src1: u8, src2: u16, offset: i32 },
    JumpIfNotEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfNotEqualK { src1: u8, src2: u16, offset: i32 },
    Nop,
}
impl Instruction {
    pub fn discriminant(&self) -> usize {
        unsafe { *(self as *const Instruction as *const u8) as usize }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Self::AddK { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} k{}", dest, src1, src2)
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Self::SubtractRK { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} k{}", dest, src1, src2)
            }
            Self::SubtractKR { dest, src1, src2 } => {
                write!(f, "SUB r{} k{} r{}", dest, src1, src2)
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Self::MultiplyK { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} k{}", dest, src1, src2)
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Self::DivideRK { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} k{}", dest, src1, src2)
            }
            Self::DivideKR { dest, src1, src2 } => {
                write!(f, "DIV r{} k{} r{}", dest, src1, src2)
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Self::ModuloRK { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} k{}", dest, src1, src2)
            }
            Self::ModuloKR { dest, src1, src2 } => {
                write!(f, "MOD r{} k{} r{}", dest, src1, src2)
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Self::EqualK { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} k{}", dest, src1, src2)
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Self::NotEqualK { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} k{}", dest, src1, src2)
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Self::LessK { dest, src1, src2 } => {
                write!(f, "LT r{} r{} k{}", dest, src1, src2)
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Self::LessEqualK { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} k{}", dest, src1, src2)
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Self::GreaterK { dest, src1, src2 } => {
                write!(f, "GT r{} r{} k{}", dest, src1, src2)
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Self::GreaterEqualK { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} k{}", dest, src1, src2)
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
            Self::CreateDict { dest } => {
                write!(f, "DICT r{}", dest)
            }
            Self::SetField { object, key, value } => {
                write!(f, "SET r{} r{} r{}", object, key, value)
            }
            Self::GetField { dest, object, key } => {
                write!(f, "GET r{} r{} r{}", dest, object, key)
            }
            Self::Call { dest, src, arity } => {
                write!(f, "CALL r{} r{} ARITY({})", dest, src, arity)
            }
            Self::Return { src } => {
                write!(f, "RET r{}", src)
            }
            Self::Jump { offset } => {
                write!(f, "JMP {}", offset)
            }
            Self::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE r{} {}", src, offset)
            }
            Self::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{} {}", src, offset)
            }
            Self::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfLessK { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} k{} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} k{} {}", src1, src2, offset)
            }
            Self::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterK { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} k{} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} k{} {}", src1, src2, offset)
            }
            Self::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} k{} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} r{} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} k{} {}", src1, src2, offset)
            }
            Self::CreateClosure { dest, src } => {
                write!(f, "CREATE_CLOSURE r{} FUNCTIONS[{}]", dest, src)
            }
            Self::CaptureValue { dest, src } => {
                write!(f, "CAPTURE_VALUE r{} r{}", dest, src)
            }
            Self::Nop => {
                write!(f, "NOP")
            }
        }
    }
}
