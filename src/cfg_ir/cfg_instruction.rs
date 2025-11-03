use core::fmt;
use std::fmt::{Display, Formatter};

use super::operand::Operand;

#[derive(Debug, Clone)]
pub enum CfgInstruction {
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
    Call {
        dest: Operand,
        src: Operand,
    },
    Print {
        src: Operand,
    },
}

impl CfgInstruction {
    pub fn add(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Add { dest, src1, src2 }
    }

    pub fn subtract(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Subtract { dest, src1, src2 }
    }

    pub fn multiply(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Multiply { dest, src1, src2 }
    }

    pub fn divide(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Divide { dest, src1, src2 }
    }

    pub fn modulo(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Modulo { dest, src1, src2 }
    }

    pub fn equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Equal { dest, src1, src2 }
    }

    pub fn not_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::NotEqual { dest, src1, src2 }
    }

    pub fn greater(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Greater { dest, src1, src2 }
    }

    pub fn greater_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::GreaterEqual { dest, src1, src2 }
    }

    pub fn less(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Less { dest, src1, src2 }
    }

    pub fn less_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::LessEqual { dest, src1, src2 }
    }

    pub fn negate(dest: Operand, src: Operand) -> Self {
        Self::Negate { dest, src }
    }

    pub fn not(dest: Operand, src: Operand) -> Self {
        Self::Not { dest, src }
    }

    pub fn move_(dest: Operand, src: Operand) -> Self {
        Self::Move { dest, src }
    }

    pub fn move_arg(dest: Operand, src: Operand) -> Self {
        Self::MoveArg { dest, src }
    }

    pub fn call(dest: Operand, src: Operand) -> Self {
        Self::Call { dest, src }
    }

    pub fn print(src: Operand) -> Self {
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
            MoveArg { dest, src } => write!(f, "{} = arg({})", dest, src),
            Call { dest, src } => {
                write!(f, "{} = call {}", dest, src)
            }
            Print { src } => write!(f, "print {}", src),
        }
    }
}
