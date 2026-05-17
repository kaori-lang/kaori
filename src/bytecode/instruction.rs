use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Register(pub u8);

#[derive(Clone, Copy, Debug)]
pub struct ConstIndex(pub u16);

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Instruction {
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    AddK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    Subtract {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    SubtractRK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    SubtractKR {
        dest: Register,
        src1: ConstIndex,
        src2: Register,
    },
    Multiply {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    MultiplyK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    Divide {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    DivideRK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    DivideKR {
        dest: Register,
        src1: ConstIndex,
        src2: Register,
    },
    Modulo {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    ModuloRK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    ModuloKR {
        dest: Register,
        src1: ConstIndex,
        src2: Register,
    },
    Equal {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    EqualK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    NotEqual {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    NotEqualK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    Less {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    LessK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    LessEqual {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    LessEqualK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    Greater {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    GreaterK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    GreaterEqual {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    GreaterEqualK {
        dest: Register,
        src1: Register,
        src2: ConstIndex,
    },
    Not {
        dest: Register,
        src: Register,
    },
    Negate {
        dest: Register,
        src: Register,
    },
    Move {
        dest: Register,
        src: Register,
    },
    MoveArg {
        dest: Register,
        src: Register,
    },
    LoadK {
        dest: Register,
        src: ConstIndex,
    },
    CreateDict {
        dest: Register,
    },
    SetField {
        object: Register,
        key: Register,
        value: Register,
    },
    GetField {
        dest: Register,
        object: Register,
        key: Register,
    },
    CreateClosure {
        dest: Register,
        src: u32,
    },
    CaptureValue {
        dest: Register,
        src: Register,
    },
    Call {
        dest: Register,
        src: Register,
        arity: u8,
    },
    Return {
        src: Register,
    },
    Jump {
        offset: i32,
    },
    JumpIfFalse {
        src: Register,
        offset: i32,
    },
    JumpIfTrue {
        src: Register,
        offset: i32,
    },
    JumpIfLess {
        src1: Register,
        src2: Register,
        offset: i32,
    },
    JumpIfLessK {
        src1: Register,
        src2: ConstIndex,
        offset: i32,
    },
    JumpIfLessEqual {
        src1: Register,
        src2: Register,
        offset: i32,
    },
    JumpIfLessEqualK {
        src1: Register,
        src2: ConstIndex,
        offset: i32,
    },
    JumpIfGreater {
        src1: Register,
        src2: Register,
        offset: i32,
    },
    JumpIfGreaterK {
        src1: Register,
        src2: ConstIndex,
        offset: i32,
    },
    JumpIfGreaterEqual {
        src1: Register,
        src2: Register,
        offset: i32,
    },
    JumpIfGreaterEqualK {
        src1: Register,
        src2: ConstIndex,
        offset: i32,
    },
    JumpIfEqual {
        src1: Register,
        src2: Register,
        offset: i32,
    },
    JumpIfEqualK {
        src1: Register,
        src2: ConstIndex,
        offset: i32,
    },
    JumpIfNotEqual {
        src1: Register,
        src2: Register,
        offset: i32,
    },
    JumpIfNotEqualK {
        src1: Register,
        src2: ConstIndex,
        offset: i32,
    },
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
                write!(f, "ADD r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::AddK { dest, src1, src2 } => {
                write!(f, "ADD r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "SUB r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::SubtractRK { dest, src1, src2 } => {
                write!(f, "SUB r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::SubtractKR { dest, src1, src2 } => {
                write!(f, "SUB r{:?} k{:?} r{:?}", dest, src1, src2)
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::MultiplyK { dest, src1, src2 } => {
                write!(f, "MUL r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "DIV r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::DivideRK { dest, src1, src2 } => {
                write!(f, "DIV r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::DivideKR { dest, src1, src2 } => {
                write!(f, "DIV r{:?} k{:?} r{:?}", dest, src1, src2)
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "MOD r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::ModuloRK { dest, src1, src2 } => {
                write!(f, "MOD r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::ModuloKR { dest, src1, src2 } => {
                write!(f, "MOD r{:?} k{:?} r{:?}", dest, src1, src2)
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::EqualK { dest, src1, src2 } => {
                write!(f, "EQ r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::NotEqualK { dest, src1, src2 } => {
                write!(f, "NEQ r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "LT r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::LessK { dest, src1, src2 } => {
                write!(f, "LT r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::LessEqualK { dest, src1, src2 } => {
                write!(f, "LTE r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "GT r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::GreaterK { dest, src1, src2 } => {
                write!(f, "GT r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{:?} r{:?} r{:?}", dest, src1, src2)
            }
            Self::GreaterEqualK { dest, src1, src2 } => {
                write!(f, "GTE r{:?} r{:?} k{:?}", dest, src1, src2)
            }
            Self::Not { dest, src } => {
                write!(f, "NOT r{:?} r{:?}", dest, src)
            }
            Self::Negate { dest, src } => {
                write!(f, "NEG r{:?} r{:?}", dest, src)
            }
            Self::Move { dest, src } => {
                write!(f, "MOV r{:?} r{:?}", dest, src)
            }
            Self::MoveArg { dest, src } => {
                write!(f, "MOV_ARG r{:?} r{:?}", dest, src)
            }
            Self::LoadK { dest, src } => {
                write!(f, "LOADK r{:?} k{:?}", dest, src)
            }
            Self::CreateDict { dest } => {
                write!(f, "DICT r{:?}", dest)
            }
            Self::SetField { object, key, value } => {
                write!(f, "SET r{:?} r{:?} r{:?}", object, key, value)
            }
            Self::GetField { dest, object, key } => {
                write!(f, "GET r{:?} r{:?} r{:?}", dest, object, key)
            }
            Self::Call { dest, src, arity } => {
                write!(f, "CALL r{:?} r{:?} ARITY({})", dest, src, arity)
            }
            Self::Return { src } => {
                write!(f, "RET r{:?}", src)
            }
            Self::Jump { offset } => {
                write!(f, "JMP {}", offset)
            }
            Self::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE r{:?} {}", src, offset)
            }
            Self::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{:?} {}", src, offset)
            }
            Self::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{:?} r{:?} {}", src1, src2, offset)
            }
            Self::JumpIfLessK { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{:?} k{:?} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{:?} r{:?} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{:?} k{:?} {}", src1, src2, offset)
            }
            Self::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{:?} r{:?} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterK { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{:?} k{:?} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{:?} r{:?} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{:?} k{:?} {}", src1, src2, offset)
            }
            Self::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{:?} r{:?} {}", src1, src2, offset)
            }
            Self::JumpIfEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{:?} k{:?} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{:?} r{:?} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqualK { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{:?} k{:?} {}", src1, src2, offset)
            }
            Self::CreateClosure { dest, src } => {
                write!(f, "CREATE_CLOSURE r{:?} FUNCTIONS[{}]", dest, src)
            }
            Self::CaptureValue { dest, src } => {
                write!(f, "CAPTURE_VALUE r{:?} r{:?}", dest, src)
            }
            Self::Nop => {
                write!(f, "NOP")
            }
        }
    }
}
