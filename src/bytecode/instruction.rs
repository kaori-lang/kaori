use crate::cfg_ir::operand::{Operand, Register};

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
    And {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Or {
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
    Const {
        dest: Register,
        src: ConstantIndex,
    },
    Move {
        dest: Register,
        src: Register,
    },
    Call,
    Return {
        src: Register,
    },
    Jump {
        offset: i16,
    },
    JumpFalse {
        src: Register,
        offset: i16,
    },
    Print {
        src: Register,
    },
}

impl Instruction {
    pub fn add(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Add {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn subtract(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Subtract {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn multiply(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Multiply {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn divide(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Divide {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn modulo(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Modulo {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Equal {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn not_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::NotEqual {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn greater(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Greater {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn greater_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::GreaterEqual {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn less(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Less {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn less_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::LessEqual {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn and(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::And {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn or(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Self::Or {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn negate(dest: Operand, src: Operand) -> Self {
        Self::Negate {
            dest: dest.to_register(),
            src: src.to_register(),
        }
    }
    pub fn not(dest: Operand, src: Operand) -> Self {
        Self::Not {
            dest: dest.to_register(),
            src: src.to_register(),
        }
    }
    pub fn const_(dest: Operand, src: ConstantIndex) -> Self {
        Self::Const {
            dest: dest.to_register(),
            src,
        }
    }
    pub fn mov(dest: Operand, src: Operand) -> Self {
        Self::Move {
            dest: dest.to_register(),
            src: src.to_register(),
        }
    }
    pub fn call() -> Self {
        Self::Call
    }
    pub fn return_(src: Operand) -> Self {
        Self::Return {
            src: src.to_register(),
        }
    }
    pub fn jump(offset: i16) -> Self {
        Self::Jump { offset }
    }
    pub fn jump_false(src: Operand, offset: i16) -> Self {
        Self::JumpFalse {
            src: src.to_register(),
            offset,
        }
    }
    pub fn print(src: Operand) -> Self {
        Self::Print {
            src: src.to_register(),
        }
    }
}

use std::fmt;

use super::constant_pool::ConstantIndex;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "Add {}, {}, {}", dest, src1, src2)
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "Subtract {}, {}, {}", dest, src1, src2)
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "Multiply {}, {}, {}", dest, src1, src2)
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "Divide {}, {}, {}", dest, src1, src2)
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "Modulo {}, {}, {}", dest, src1, src2)
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "Equal {}, {}, {}", dest, src1, src2)
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NotEqual {}, {}, {}", dest, src1, src2)
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "Greater {}, {}, {}", dest, src1, src2)
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GreaterEqual {}, {}, {}", dest, src1, src2)
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "Less {}, {}, {}", dest, src1, src2)
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LessEqual {}, {}, {}", dest, src1, src2)
            }
            Self::And { dest, src1, src2 } => {
                write!(f, "And {}, {}, {}", dest, src1, src2)
            }
            Self::Or { dest, src1, src2 } => {
                write!(f, "Or {}, {}, {}", dest, src1, src2)
            }
            Self::Negate { dest, src } => write!(f, "Negate {}, {}", dest, src),
            Self::Not { dest, src } => write!(f, "Not {}, {}", dest, src),
            Self::Const { dest, src } => write!(f, "Const {}, {}", dest, src),
            Self::Move { dest, src } => write!(f, "Move {}, {}", dest, src),
            Self::Call => write!(f, "Call"),
            Self::Return { src } => write!(f, "Return {}", src),
            Self::Jump { offset } => write!(f, "Jump {}", offset),
            Self::JumpFalse { src, offset } => write!(f, "JumpFalse {} {}", src, offset),
            Self::Print { src } => write!(f, "Print {}", src),
        }
    }
}
