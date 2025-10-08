use core::fmt;

use crate::cfg_ir::operand::Operand;

pub enum Instruction {
    Add {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Subtract {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Multiply {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Divide {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Modulo {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Equal {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    NotEqual {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Greater {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    GreaterEqual {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    Less {
        dest: i16,
        src1: i16,
        src2: i16,
    },
    LessEqual {
        dest: i16,
        src1: i16,
        src2: i16,
    },

    Negate {
        dest: i16,
        src: i16,
    },
    Not {
        dest: i16,
        src: i16,
    },
    Move {
        dest: i16,
        src: i16,
    },
    Call,
    Return {
        src: i16,
    },
    Jump {
        offset: i16,
    },
    ConditionalJump {
        src: i16,
        true_offset: i16,
        false_offset: i16,
    },
    Print {
        src: i16,
    },
    Halt,
}

impl Instruction {
    pub fn index(&self) -> usize {
        match self {
            Instruction::Add { .. } => 0,
            Instruction::Subtract { .. } => 1,
            Instruction::Multiply { .. } => 2,
            Instruction::Divide { .. } => 3,
            Instruction::Modulo { .. } => 4,
            Instruction::Equal { .. } => 5,
            Instruction::NotEqual { .. } => 6,
            Instruction::Greater { .. } => 7,
            Instruction::GreaterEqual { .. } => 8,
            Instruction::Less { .. } => 9,
            Instruction::LessEqual { .. } => 10,
            Instruction::Negate { .. } => 11,
            Instruction::Not { .. } => 12,
            Instruction::Move { .. } => 13,
            Instruction::Call => 14,
            Instruction::Return { .. } => 15,
            Instruction::Jump { .. } => 16,
            Instruction::ConditionalJump { .. } => 17,
            Instruction::Print { .. } => 18,
            Instruction::Halt => 19,
        }
    }

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

    pub fn conditional_jump(src: Operand, true_offset: i16, false_offset: i16) -> Self {
        Self::ConditionalJump {
            src: src.to_register(),
            true_offset,
            false_offset,
        }
    }

    pub fn print(src: Operand) -> Self {
        Self::Print {
            src: src.to_register(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "Add {dest}, {src1}, {src2}")
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "Subtract {dest}, {src1}, {src2}")
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "Multiply {dest}, {src1}, {src2}")
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "Divide {dest}, {src1}, {src2}")
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "Modulo {dest}, {src1}, {src2}")
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "Equal {dest}, {src1}, {src2}")
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NotEqual {dest}, {src1}, {src2}")
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "Greater {dest}, {src1}, {src2}")
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GreaterEqual {dest}, {src1}, {src2}")
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "Less {dest}, {src1}, {src2}")
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LessEqual {dest}, {src1}, {src2}")
            }

            Self::Negate { dest, src } => write!(f, "Negate {dest}, {src}"),
            Self::Not { dest, src } => write!(f, "Not {dest}, {src}"),
            Self::Move { dest, src } => write!(f, "Move {dest}, {src}"),
            Self::Call => write!(f, "Call"),
            Self::Return { src } => write!(f, "Return {src}"),
            Self::Jump { offset } => write!(f, "Jump {offset}"),
            Self::ConditionalJump {
                src,
                true_offset,
                false_offset,
            } => write!(f, "ConditionalJump {src}, {true_offset} {false_offset}"),
            Self::Print { src } => write!(f, "Print {src}"),
            Self::Halt => write!(f, "Halt"),
        }
    }
}
