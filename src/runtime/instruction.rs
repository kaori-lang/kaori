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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

impl fmt::Display for ConstIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "k{}", self.0)
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
            Self::LessK { dest, src1, src2 } => {
                write!(f, "LT {} {} {}", dest, src1, src2)
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE {} {} {}", dest, src1, src2)
            }
            Self::LessEqualK { dest, src1, src2 } => {
                write!(f, "LTE {} {} {}", dest, src1, src2)
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "GT {} {} {}", dest, src1, src2)
            }
            Self::GreaterK { dest, src1, src2 } => {
                write!(f, "GT {} {} {}", dest, src1, src2)
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE {} {} {}", dest, src1, src2)
            }
            Self::GreaterEqualK { dest, src1, src2 } => {
                write!(f, "GTE {} {} {}", dest, src1, src2)
            }
            Self::Not { dest, src } => {
                write!(f, "NOT {} {}", dest, src)
            }
            Self::Negate { dest, src } => {
                write!(f, "NEG {} {}", dest, src)
            }
            Self::Move { dest, src } => {
                write!(f, "MOV {} {}", dest, src)
            }
            Self::MoveArg { dest, src } => {
                write!(f, "MOV_ARG {} {}", dest, src)
            }
            Self::LoadK { dest, src } => {
                write!(f, "LOADK {} {}", dest, src)
            }
            Self::CreateDict { dest } => {
                write!(f, "DICT {}", dest)
            }
            Self::SetField { object, key, value } => {
                write!(f, "SET {} {} {}", object, key, value)
            }
            Self::GetField { dest, object, key } => {
                write!(f, "GET {} {} {}", dest, object, key)
            }
            Self::Call { dest, src, arity } => {
                write!(f, "CALL {} {} ARITY({})", dest, src, arity)
            }
            Self::Return { src } => {
                write!(f, "RET {}", src)
            }
            Self::Jump { offset } => {
                write!(f, "JMP {}", offset)
            }
            Self::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE {} {}", src, offset)
            }
            Self::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE {} {}", src, offset)
            }
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
            Self::CreateClosure { dest, src } => {
                write!(f, "CREATE_CLOSURE {} FUNCTIONS[{}]", dest, src)
            }
            Self::CaptureValue { dest, src } => {
                write!(f, "CAPTURE_VALUE {} {}", dest, src)
            }
            Self::Nop => {
                write!(f, "NOP")
            }
        }
    }
}
