use core::fmt;
use std::fmt::{Display, Formatter};

use super::operand::Operand;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Subtract {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Multiply {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Divide {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Modulo {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Power {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Equal {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    NotEqual {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Greater {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    GreaterEqual {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Less {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    LessEqual {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },

    Negate {
        dest: Operand,
        src: Operand,
    },
    Not {
        dest: Operand,
        src: Operand,
    },
    Move {
        dest: Operand,
        src: Operand,
    },
    MoveArg {
        dest: Operand,
        src: Operand,
    },
    CreateDict {
        dest: Operand,
    },
    SetField {
        object: Operand,
        key: Operand,
        value: Operand,
    },
    GetField {
        dest: Operand,
        object: Operand,
        key: Operand,
    },
    Call {
        dest: Operand,
        func: Operand,
    },
    Print {
        src: Operand,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Add { dest, src1, src2 } => {
                write!(f, "ADD {} {} {}", dest, src1, src2)
            }
            Subtract { dest, src1, src2 } => {
                write!(f, "SUB {} {} {}", dest, src1, src2)
            }
            Multiply { dest, src1, src2 } => {
                write!(f, "MUL {} {} {}", dest, src1, src2)
            }
            Divide { dest, src1, src2 } => {
                write!(f, "DIV {} {} {}", dest, src1, src2)
            }
            Modulo { dest, src1, src2 } => {
                write!(f, "MOD {} {} {}", dest, src1, src2)
            }
            Power { dest, src1, src2 } => {
                write!(f, "POW {} {} {}", dest, src1, src2)
            }
            Equal { dest, src1, src2 } => {
                write!(f, "EQ {} {} {}", dest, src1, src2)
            }
            NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ {} {} {}", dest, src1, src2)
            }
            Greater { dest, src1, src2 } => {
                write!(f, "GT {} {} {}", dest, src1, src2)
            }
            GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE {} {} {}", dest, src1, src2)
            }
            Less { dest, src1, src2 } => {
                write!(f, "LT {} {} {}", dest, src1, src2)
            }
            LessEqual { dest, src1, src2 } => {
                write!(f, "LTE {} {} {}", dest, src1, src2)
            }

            Negate { dest, src } => {
                write!(f, "NEG {} {}", dest, src)
            }
            Not { dest, src } => {
                write!(f, "NOT {} {}", dest, src)
            }

            Move { dest, src } => {
                write!(f, "MOV {} {}", dest, src)
            }

            MoveArg { dest, src } => {
                write!(f, "MOV_ARG {} {}", dest, src)
            }

            CreateDict { dest } => {
                write!(f, "NEWDICT {}", dest)
            }

            SetField { object, key, value } => {
                write!(f, "SETFIELD {} {} {}", object, key, value)
            }

            GetField { dest, object, key } => {
                write!(f, "GETFIELD {} {} {}", dest, object, key)
            }

            Call { dest, func } => {
                write!(f, "CALL {} {}", dest, func)
            }

            Print { src } => {
                write!(f, "PRINT {}", src)
            }
        }
    }
}
