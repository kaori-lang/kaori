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

fn fmt_operand(op: &Operand) -> String {
    match op {
        Operand::Variable(i) => format!("r{}", i),
        Operand::Constant(i) => format!("k{}", i),
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Add { dest, src1, src2 } => {
                write!(
                    f,
                    "ADD {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Subtract { dest, src1, src2 } => {
                write!(
                    f,
                    "SUB {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Multiply { dest, src1, src2 } => {
                write!(
                    f,
                    "MUL {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Divide { dest, src1, src2 } => {
                write!(
                    f,
                    "DIV {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Modulo { dest, src1, src2 } => {
                write!(
                    f,
                    "MOD {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Power { dest, src1, src2 } => {
                write!(
                    f,
                    "POW {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Equal { dest, src1, src2 } => {
                write!(
                    f,
                    "EQ {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            NotEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "NEQ {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Greater { dest, src1, src2 } => {
                write!(
                    f,
                    "GT {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            GreaterEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "GTE {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            Less { dest, src1, src2 } => {
                write!(
                    f,
                    "LT {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }
            LessEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "LTE {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(src1),
                    fmt_operand(src2)
                )
            }

            Negate { dest, src } => {
                write!(f, "NEG {} {}", fmt_operand(dest), fmt_operand(src))
            }
            Not { dest, src } => {
                write!(f, "NOT {} {}", fmt_operand(dest), fmt_operand(src))
            }

            Move { dest, src } => {
                write!(f, "MOV {} {}", fmt_operand(dest), fmt_operand(src))
            }

            MoveArg { dest, src } => {
                write!(f, "MOV_ARG {} {}", fmt_operand(dest), fmt_operand(src))
            }

            CreateDict { dest } => {
                write!(f, "NEWDICT {}", fmt_operand(dest))
            }

            SetField { object, key, value } => {
                write!(
                    f,
                    "SETFIELD {} {} {}",
                    fmt_operand(object),
                    fmt_operand(key),
                    fmt_operand(value)
                )
            }

            GetField { dest, object, key } => {
                write!(
                    f,
                    "GETFIELD {} {} {}",
                    fmt_operand(dest),
                    fmt_operand(object),
                    fmt_operand(key)
                )
            }

            Call { dest, func } => {
                write!(f, "CALL {} {}", fmt_operand(dest), fmt_operand(func))
            }

            Print { src } => {
                write!(f, "PRINT {}", fmt_operand(src))
            }
        }
    }
}
