use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(pub i16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstIndex(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
    Const(ConstIndex),
}

impl From<Register> for Operand {
    fn from(r: Register) -> Self {
        Operand::Register(r)
    }
}

impl From<ConstIndex> for Operand {
    fn from(k: ConstIndex) -> Self {
        Operand::Const(k)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Add {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Subtract {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Multiply {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Divide {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Modulo {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Equal {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    NotEqual {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Less {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    LessEqual {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Greater {
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    GreaterEqual {
        dest: Register,
        src1: Operand,
        src2: Operand,
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
    LoadConst {
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
    CreateCell {
        dest: Register,
        src: Register,
    },
    SetCell {
        dest: Register,
        src: Register,
    },
    GetCell {
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
    JumpIfEqual {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfNotEqual {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfLess {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfNotLess {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfLessEqual {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfNotLessEqual {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfGreater {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfNotGreater {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfGreaterEqual {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    JumpIfNotGreaterEqual {
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    Nop,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            write!(f, "arg({})", -(self.0 + 1))
        } else {
            write!(f, "R{}", self.0)
        }
    }
}

impl fmt::Display for ConstIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Const(c) => write!(f, "{}", c),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => write!(f, "ADD {} {} {}", dest, src1, src2),
            Self::Subtract { dest, src1, src2 } => write!(f, "SUB {} {} {}", dest, src1, src2),
            Self::Multiply { dest, src1, src2 } => write!(f, "MUL {} {} {}", dest, src1, src2),
            Self::Divide { dest, src1, src2 } => write!(f, "DIV {} {} {}", dest, src1, src2),
            Self::Modulo { dest, src1, src2 } => write!(f, "MOD {} {} {}", dest, src1, src2),
            Self::Equal { dest, src1, src2 } => write!(f, "EQ {} {} {}", dest, src1, src2),
            Self::NotEqual { dest, src1, src2 } => write!(f, "NEQ {} {} {}", dest, src1, src2),
            Self::Less { dest, src1, src2 } => write!(f, "LT {} {} {}", dest, src1, src2),
            Self::LessEqual { dest, src1, src2 } => write!(f, "LTE {} {} {}", dest, src1, src2),
            Self::Greater { dest, src1, src2 } => write!(f, "GT {} {} {}", dest, src1, src2),
            Self::GreaterEqual { dest, src1, src2 } => write!(f, "GTE {} {} {}", dest, src1, src2),
            Self::Not { dest, src } => write!(f, "NOT {} {}", dest, src),
            Self::Negate { dest, src } => write!(f, "NEG {} {}", dest, src),
            Self::Move { dest, src } => write!(f, "MOV {} {}", dest, src),
            Self::LoadConst { dest, src } => write!(f, "LOAD_CONST {} {}", dest, src),
            Self::CreateDict { dest } => write!(f, "DICT {}", dest),
            Self::SetField { object, key, value } => write!(f, "SET {} {} {}", object, key, value),
            Self::GetField { dest, object, key } => write!(f, "GET {} {} {}", dest, object, key),
            Self::CreateClosure { dest, src } => {
                write!(f, "CREATE_CLOSURE {} FUNCTIONS[{}]", dest, src)
            }
            Self::CaptureValue { dest, src } => write!(f, "CAPTURE_VALUE {} {}", dest, src),
            Self::CreateCell { dest, src } => write!(f, "CREATE_CELL {} {}", dest, src),
            Self::SetCell { dest, src } => write!(f, "SET_CELL {} {}", dest, src),
            Self::GetCell { dest, src } => write!(f, "GET_CELL {} {}", dest, src),
            Self::Call { dest, src, arity } => write!(f, "CALL {} {} ARITY({})", dest, src, arity),
            Self::Return { src } => write!(f, "RET {}", src),
            Self::Jump { offset } => write!(f, "JMP {}", offset),
            Self::JumpIfTrue { src, offset } => write!(f, "JMP_IF_TRUE {} {}", src, offset),
            Self::JumpIfFalse { src, offset } => write!(f, "JMP_IF_FALSE {} {}", src, offset),
            Self::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotLess { src1, src2, offset } => {
                write!(f, "JMP_IF_NOT_LT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NOT_LTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_NOT_GT {} {} {}", src1, src2, offset)
            }
            Self::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE {} {} {}", src1, src2, offset)
            }
            Self::JumpIfNotGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NOT_GTE {} {} {}", src1, src2, offset)
            }
            Self::Nop => write!(f, "NOP"),
        }
    }
}
