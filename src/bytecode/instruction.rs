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
    LoadConst {
        dest: Register,
        src: u16,
    },
    Move {
        dest: Register,
        src: Register,
    },
    Call,
    Return {
        src: Register,
    },
    Jump(i16),
    JumpFalse(i16),
    Print,
}

impl Instruction {
    pub fn add(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Add {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn subtract(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Subtract {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn multiply(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Multiply {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn divide(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Divide {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn modulo(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Modulo {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Equal {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn not_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::NotEqual {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn greater(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Greater {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn greater_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::GreaterEqual {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn less(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Less {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn less_equal(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::LessEqual {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn and(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::And {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn or(dest: Operand, src1: Operand, src2: Operand) -> Self {
        Instruction::Or {
            dest: dest.to_register(),
            src1: src1.to_register(),
            src2: src2.to_register(),
        }
    }
    pub fn negate(dest: Operand, src: Operand) -> Self {
        Instruction::Negate {
            dest: dest.to_register(),
            src: src.to_register(),
        }
    }
    pub fn not(dest: Operand, src: Operand) -> Self {
        Instruction::Not {
            dest: dest.to_register(),
            src: src.to_register(),
        }
    }
    pub fn load_const(dest: Operand, src: u16) -> Self {
        Instruction::LoadConst {
            dest: dest.to_register(),
            src,
        }
    }
    pub fn mov(dest: Operand, src: Operand) -> Self {
        Instruction::Move {
            dest: dest.to_register(),
            src: src.to_register(),
        }
    }
    pub fn call() -> Self {
        Instruction::Call
    }
    pub fn return_(src: Operand) -> Self {
        Instruction::Return {
            src: src.to_register(),
        }
    }
    pub fn jump(offset: i16) -> Self {
        Instruction::Jump(offset)
    }
    pub fn jump_false(offset: i16) -> Self {
        Instruction::JumpFalse(offset)
    }
    pub fn print() -> Self {
        Instruction::Print
    }
}

use std::fmt;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add { dest, src1, src2 } => {
                write!(f, "Add {}, {}, {}", dest, src1, src2)
            }
            Instruction::Subtract { dest, src1, src2 } => {
                write!(f, "Subtract {}, {}, {}", dest, src1, src2)
            }
            Instruction::Multiply { dest, src1, src2 } => {
                write!(f, "Multiply {}, {}, {}", dest, src1, src2)
            }
            Instruction::Divide { dest, src1, src2 } => {
                write!(f, "Divide {}, {}, {}", dest, src1, src2)
            }
            Instruction::Modulo { dest, src1, src2 } => {
                write!(f, "Modulo {}, {}, {}", dest, src1, src2)
            }
            Instruction::Equal { dest, src1, src2 } => {
                write!(f, "Equal {}, {}, {}", dest, src1, src2)
            }
            Instruction::NotEqual { dest, src1, src2 } => {
                write!(f, "NotEqual {}, {}, {}", dest, src1, src2)
            }
            Instruction::Greater { dest, src1, src2 } => {
                write!(f, "Greater {}, {}, {}", dest, src1, src2)
            }
            Instruction::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GreaterEqual {}, {}, {}", dest, src1, src2)
            }
            Instruction::Less { dest, src1, src2 } => {
                write!(f, "Less {}, {}, {}", dest, src1, src2)
            }
            Instruction::LessEqual { dest, src1, src2 } => {
                write!(f, "LessEqual {}, {}, {}", dest, src1, src2)
            }
            Instruction::And { dest, src1, src2 } => {
                write!(f, "And {}, {}, {}", dest, src1, src2)
            }
            Instruction::Or { dest, src1, src2 } => {
                write!(f, "Or {}, {}, {}", dest, src1, src2)
            }
            Instruction::Negate { dest, src } => write!(f, "Negate {}, {}", dest, src),
            Instruction::Not { dest, src } => write!(f, "Not {}, {}", dest, src),
            Instruction::LoadConst { dest, src } => write!(f, "LoadConst {}, {}", dest, src),
            Instruction::Move { dest, src } => write!(f, "Move {}, {}", dest, src),
            Instruction::Call => write!(f, "Call"),
            Instruction::Return { src } => write!(f, "Return {}", src),
            Instruction::Jump(offset) => write!(f, "Jump {}", offset),
            Instruction::JumpFalse(offset) => write!(f, "JumpFalse {}", offset),
            Instruction::Print => write!(f, "Print"),
        }
    }
}
