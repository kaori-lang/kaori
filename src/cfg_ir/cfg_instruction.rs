use core::fmt;
use std::fmt::{Display, Formatter};

use super::{basic_block::BlockId, variable::Variable};

#[derive(Debug, Clone)]
pub enum CfgInstruction {
    Add {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Subtract {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Multiply {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Divide {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Modulo {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Equal {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    NotEqual {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Greater {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    GreaterEqual {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Less {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    LessEqual {
        dest: Variable,
        src1: Variable,
        src2: Variable,
    },
    Negate {
        dest: Variable,
        src: Variable,
    },
    Not {
        dest: Variable,
        src: Variable,
    },
    Move {
        dest: Variable,
        src: Variable,
    },
    Call {
        dest: Variable,
        src: Variable,
        caller_size: u16,
    },
    Print {
        src: Variable,
    },
}

impl CfgInstruction {
    pub fn add(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Add { dest, src1, src2 }
    }

    pub fn subtract(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Subtract { dest, src1, src2 }
    }

    pub fn multiply(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Multiply { dest, src1, src2 }
    }

    pub fn divide(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Divide { dest, src1, src2 }
    }

    pub fn modulo(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Modulo { dest, src1, src2 }
    }

    pub fn equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Equal { dest, src1, src2 }
    }

    pub fn not_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::NotEqual { dest, src1, src2 }
    }

    pub fn greater(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Greater { dest, src1, src2 }
    }

    pub fn greater_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::GreaterEqual { dest, src1, src2 }
    }

    pub fn less(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Less { dest, src1, src2 }
    }

    pub fn less_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::LessEqual { dest, src1, src2 }
    }

    pub fn negate(dest: Variable, src: Variable) -> Self {
        Self::Negate { dest, src }
    }

    pub fn not(dest: Variable, src: Variable) -> Self {
        Self::Not { dest, src }
    }

    pub fn move_(dest: Variable, src: Variable) -> Self {
        Self::Move { dest, src }
    }

    pub fn call(dest: Variable, src: Variable, caller_size: u16) -> Self {
        Self::Call {
            dest,
            src,
            caller_size,
        }
    }

    pub fn print(src: Variable) -> Self {
        Self::Print { src }
    }
}

impl Display for CfgInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use CfgInstruction::*;

        match self {
            Add { dest, src1, src2 } => write!(f, "{} = {} + {}", dest, src1, src2),
            Subtract { dest, src1, src2 } => write!(f, "{} = {} - {}", dest, src1, src2),
            Multiply { dest, src1, src2 } => write!(f, "{} = {} * {}", dest, src1, src2),
            Divide { dest, src1, src2 } => write!(f, "{} = {} / {}", dest, src1, src2),
            Modulo { dest, src1, src2 } => write!(f, "{} = {} % {}", dest, src1, src2),

            Equal { dest, src1, src2 } => write!(f, "{} = {} == {}", dest, src1, src2),
            NotEqual { dest, src1, src2 } => write!(f, "{} = {} != {}", dest, src1, src2),
            Greater { dest, src1, src2 } => write!(f, "{} = {} > {}", dest, src1, src2),
            GreaterEqual { dest, src1, src2 } => write!(f, "{} = {} >= {}", dest, src1, src2),
            Less { dest, src1, src2 } => write!(f, "{} = {} < {}", dest, src1, src2),
            LessEqual { dest, src1, src2 } => write!(f, "{} = {} <= {}", dest, src1, src2),

            Negate { dest, src } => write!(f, "{} = -{}", dest, src),
            Not { dest, src } => write!(f, "{} = !{}", dest, src),
            Move { dest, src } => write!(f, "{} = {}", dest, src),
            Call {
                dest,
                src,
                caller_size,
            } => {
                write!(f, "{} = call {} ({})", dest, src, caller_size)
            }
            Print { src } => write!(f, "print {}", src),
            _ => todo!(),
        }
    }
}
