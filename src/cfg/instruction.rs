use core::fmt;
use std::fmt::{Display, Formatter};

use super::{constant_pool::ConstantIndex, register::Register};

#[derive(Debug, Clone)]
pub enum Instruction {
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Subtract {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Multiply {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Divide {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Modulo {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Power {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Equal {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    NotEqual {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Greater {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    GreaterEqual {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Less {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    LessEqual {
        dest: Register,
        src1: Register,
        src2: Register,
    },

    Negate {
        dest: Register,
        src: Register,
    },
    Not {
        dest: Register,
        src: Register,
    },
    Move {
        dest: Register,
        src: Register,
    },
    LoadConst {
        dest: Register,
        src: ConstantIndex,
    },
    MoveArg {
        dest: Register,
        src: Register,
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
    Call {
        dest: Register,
        func: Register,
    },
    Print {
        src: Register,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Add { dest, src1, src2 } => write!(f, "ADD {} {} {}", dest, src1, src2),
            Subtract { dest, src1, src2 } => write!(f, "SUB {} {} {}", dest, src1, src2),
            Multiply { dest, src1, src2 } => write!(f, "MUL {} {} {}", dest, src1, src2),
            Divide { dest, src1, src2 } => write!(f, "DIV {} {} {}", dest, src1, src2),
            Modulo { dest, src1, src2 } => write!(f, "MOD {} {} {}", dest, src1, src2),
            Power { dest, src1, src2 } => write!(f, "POW {} {} {}", dest, src1, src2),
            Equal { dest, src1, src2 } => write!(f, "EQ {} {} {}", dest, src1, src2),
            NotEqual { dest, src1, src2 } => write!(f, "NEQ {} {} {}", dest, src1, src2),
            Greater { dest, src1, src2 } => write!(f, "GT {} {} {}", dest, src1, src2),
            GreaterEqual { dest, src1, src2 } => write!(f, "GTE {} {} {}", dest, src1, src2),
            Less { dest, src1, src2 } => write!(f, "LT {} {} {}", dest, src1, src2),
            LessEqual { dest, src1, src2 } => write!(f, "LTE {} {} {}", dest, src1, src2),

            Negate { dest, src } => write!(f, "NEG {} {}", dest, src),
            Not { dest, src } => write!(f, "NOT {} {}", dest, src),
            Move { dest, src } => write!(f, "MOV {} {}", dest, src),

            LoadConst { dest, src } => write!(f, "LOADK {} {}", dest, src),

            MoveArg { dest, src } => write!(f, "MOV_ARG {} {}", dest, src),
            CreateDict { dest } => write!(f, "NEWDICT {}", dest),
            SetField { object, key, value } => {
                write!(f, "SETFIELD {} {} {}", object, key, value)
            }
            GetField { dest, object, key } => {
                write!(f, "GETFIELD {} {} {}", dest, object, key)
            }
            Call { dest, func } => write!(f, "CALL {} {}", dest, func),
            Print { src } => write!(f, "PRINT {}", src),
        }
    }
}
