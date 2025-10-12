use core::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[repr(align(8))]
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
    /* pub fn discriminant(&self) -> usize {
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
    */
    #[inline(always)]
    pub fn discriminant(&self) -> usize {
        unsafe { *<*const _>::from(self).cast::<u8>() as usize }
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
    pub fn call(dest: Operand, src: Operand, caller_size: isize) -> Self {
        Self::Call {
            dest: dest.to_register(),
            src: src.to_register(),
            caller_size: caller_size as u16,
        }
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
        // helper closure for formatting registers
        let reg = |v: &i16| {
            if *v < 0 {
                format!("c{}", v.abs())
            } else {
                format!("r{}", v)
            }
        };

        match self {
            Self::Add { dest, src1, src2 } => {
                write!(f, "Add {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Subtract { dest, src1, src2 } => {
                write!(f, "Subtract {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Multiply { dest, src1, src2 } => {
                write!(f, "Multiply {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Divide { dest, src1, src2 } => {
                write!(f, "Divide {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Modulo { dest, src1, src2 } => {
                write!(f, "Modulo {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Equal { dest, src1, src2 } => {
                write!(f, "Equal {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::NotEqual { dest, src1, src2 } => {
                write!(f, "NotEqual {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Greater { dest, src1, src2 } => {
                write!(f, "Greater {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::GreaterEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "GreaterEqual {}, {}, {}",
                    reg(dest),
                    reg(src1),
                    reg(src2)
                )
            }
            Self::Less { dest, src1, src2 } => {
                write!(f, "Less {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::LessEqual { dest, src1, src2 } => {
                write!(f, "LessEqual {}, {}, {}", reg(dest), reg(src1), reg(src2))
            }
            Self::Negate { dest, src } => write!(f, "Negate {}, {}", reg(dest), reg(src)),
            Self::Not { dest, src } => write!(f, "Not {}, {}", reg(dest), reg(src)),
            Self::Move { dest, src } => write!(f, "Move {}, {}", reg(dest), reg(src)),
            Self::Call {
                dest,
                src,
                caller_size,
            } => write!(f, "Call {}, {}, {}", reg(dest), reg(src), caller_size),
            Self::Return { src } => write!(f, "Return {}", reg(src)),
            Self::Jump { offset } => write!(f, "Jump {offset}"),
            Self::ConditionalJump {
                src,
                true_offset,
                false_offset,
            } => {
                write!(
                    f,
                    "ConditionalJump {}, {} {}",
                    reg(src),
                    true_offset,
                    false_offset
                )
            }
            Self::Print { src } => write!(f, "Print {}", reg(src)),
            Self::Halt => write!(f, "Halt"),
        }
    }
}
