use core::fmt;
use std::fmt::{Display, Formatter};

use super::operand::Operand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CfgOpcode {
    // Binary arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparisons
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Unary operations
    Negate,
    Not,

    // Data movement
    Move,
    MoveArg,

    // Calls & side effects
    Call,
    Print,
}

#[derive(Debug, Clone)]
pub struct CfgInstruction {
    pub op_code: CfgOpcode,
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl CfgInstruction {
    // --- Binary operations ---
    pub fn add(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Add,
            dest,
            src1,
            src2,
        }
    }

    pub fn subtract(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Subtract,
            dest,
            src1,
            src2,
        }
    }

    pub fn multiply(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Multiply,
            dest,
            src1,
            src2,
        }
    }

    pub fn divide(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Divide,
            dest,
            src1,
            src2,
        }
    }

    pub fn modulo(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Modulo,
            dest,
            src1,
            src2,
        }
    }

    // --- Comparisons ---
    pub fn equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Equal,
            dest,
            src1,
            src2,
        }
    }

    pub fn not_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::NotEqual,
            dest,
            src1,
            src2,
        }
    }

    pub fn greater(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Greater,
            dest,
            src1,
            src2,
        }
    }

    pub fn greater_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::GreaterEqual,
            dest,
            src1,
            src2,
        }
    }

    pub fn less(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Less,
            dest,
            src1,
            src2,
        }
    }

    pub fn less_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self {
            op_code: CfgOpcode::LessEqual,
            dest,
            src1,
            src2,
        }
    }

    // --- Unary operations ---
    pub fn negate(dest: Operand, src: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Negate,
            dest,
            src1: src,
            src2: Operand::None,
        }
    }

    pub fn not(dest: Operand, src: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Not,
            dest,
            src1: src,
            src2: Operand::None,
        }
    }

    // --- Moves ---
    pub fn move_(dest: Operand, src: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Move,
            dest,
            src1: src,
            src2: Operand::None,
        }
    }

    pub fn move_arg(dest: Operand, src: Operand) -> Self {
        Self {
            op_code: CfgOpcode::MoveArg,
            dest,
            src1: src,
            src2: Operand::None,
        }
    }

    // --- Calls and effects ---
    pub fn call(dest: Operand, src: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Call,
            dest,
            src1: src,
            src2: Operand::None,
        }
    }

    pub fn print(src: Operand) -> Self {
        Self {
            op_code: CfgOpcode::Print,
            dest: Operand::None,
            src1: src,
            src2: Operand::None,
        }
    }
}

impl Display for CfgInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use CfgOpcode::*;

        match self.op_code {
            Add => write!(f, "{} = {} + {}", self.dest, self.src1, self.src2),
            Subtract => write!(f, "{} = {} - {}", self.dest, self.src1, self.src2),
            Multiply => write!(f, "{} = {} * {}", self.dest, self.src1, self.src2),
            Divide => write!(f, "{} = {} / {}", self.dest, self.src1, self.src2),
            Modulo => write!(f, "{} = {} % {}", self.dest, self.src1, self.src2),

            Equal => write!(f, "{} = {} == {}", self.dest, self.src1, self.src2),
            NotEqual => write!(f, "{} = {} != {}", self.dest, self.src1, self.src2),
            Greater => write!(f, "{} = {} > {}", self.dest, self.src1, self.src2),
            GreaterEqual => write!(f, "{} = {} >= {}", self.dest, self.src1, self.src2),
            Less => write!(f, "{} = {} < {}", self.dest, self.src1, self.src2),
            LessEqual => write!(f, "{} = {} <= {}", self.dest, self.src1, self.src2),

            Negate => write!(f, "{} = -{}", self.dest, self.src1),
            Not => write!(f, "{} = !{}", self.dest, self.src1),
            Move => write!(f, "{} = {}", self.dest, self.src1),
            MoveArg => write!(f, "arg({}) = {}", self.dest, self.src1),
            Call => write!(f, "{} = call {}", self.dest, self.src1),
            Print => write!(f, "print {}", self.src1),
        }
    }
}
