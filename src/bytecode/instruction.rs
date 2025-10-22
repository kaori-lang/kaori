use core::fmt;

use crate::cfg_ir::variable::Variable;

#[derive(Debug, Clone)]
#[repr(u8, align(2))]
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
    Call {
        dest: i16,
        src: i16,
        caller_size: u16,
    },
    Return {
        src: i16,
    },
    Jump {
        offset: i16,
    },
    JumpIfTrue {
        src: i16,
        offset: i16,
    },
    JumpIfFalse {
        src: i16,
        offset: i16,
    },
    Print {
        src: i16,
    },
    // must be the last instruction so count is computed properly
    Halt,
}

impl Instruction {
    #[inline(always)]
    pub const fn discriminant(&self) -> usize {
        (unsafe { *(self as *const Self as *const u8) }) as usize
    }

    pub fn add(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Add {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn subtract(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Subtract {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn multiply(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Multiply {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn divide(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Divide {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn modulo(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Modulo {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Equal {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn not_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::NotEqual {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn greater(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Greater {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn greater_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::GreaterEqual {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn less(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Less {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn less_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::LessEqual {
            dest: dest.to_i16(),
            src1: src1.to_i16(),
            src2: src2.to_i16(),
        }
    }

    pub fn negate(dest: Variable, src: Variable) -> Self {
        Self::Negate {
            dest: dest.to_i16(),
            src: src.to_i16(),
        }
    }

    pub fn not(dest: Variable, src: Variable) -> Self {
        Self::Not {
            dest: dest.to_i16(),
            src: src.to_i16(),
        }
    }

    pub fn move_(dest: Variable, src: Variable) -> Self {
        Self::Move {
            dest: dest.to_i16(),
            src: src.to_i16(),
        }
    }
    pub fn call(dest: Variable, src: Variable, caller_size: u16) -> Self {
        Self::Call {
            dest: dest.to_i16(),
            src: src.to_i16(),
            caller_size,
        }
    }

    pub fn return_(src: Variable) -> Self {
        Self::Return { src: src.to_i16() }
    }

    pub fn jump(offset: i16) -> Self {
        Self::Jump { offset }
    }

    pub fn jump_if_true(src: Variable, offset: i16) -> Self {
        Self::JumpIfTrue {
            src: src.to_i16(),
            offset,
        }
    }

    pub fn jump_if_false(src: Variable, offset: i16) -> Self {
        Self::JumpIfFalse {
            src: src.to_i16(),
            offset,
        }
    }
    pub fn print(src: Variable) -> Self {
        Self::Print { src: src.to_i16() }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg = |v: &i16| {
            if *v < 0 {
                format!("c{}", v.abs())
            } else {
                format!("r{}", v)
            }
        };

        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "ADD {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "SUB {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "MUL {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "DIV {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "MOD {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "EQ {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "GT {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "LT {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Negate { dest, src } => write!(f, "NEG {}, {}", reg(dest), reg(src)),
            Self::Not { dest, src } => write!(f, "NOT {}, {}", reg(dest), reg(src)),
            Self::Move { dest, src } => write!(f, "MOV {}, {}", reg(dest), reg(src)),
            Self::Call {
                dest,
                src,
                caller_size,
            } => write!(f, "CALL {}, {}, {}", reg(dest), reg(src), caller_size),
            Self::Return { src } => write!(f, "RET {}", reg(src)),
            Self::Jump { offset } => write!(f, "JMP {}", offset),
            Self::JumpIfTrue { src, offset } => write!(f, "JIT {}, {}", reg(src), offset),
            Self::JumpIfFalse { src, offset } => write!(f, "JIF {}, {}", reg(src), offset),
            Self::Print { src } => write!(f, "PRT {}", reg(src)),
            Self::Halt => write!(f, "HLT"),
        }
    }
}
