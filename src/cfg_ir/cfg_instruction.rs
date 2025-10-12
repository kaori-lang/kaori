use super::basic_block::BlockId;

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
    Jump {
        target: BlockId,
    },
    ConditionalJump {
        src: Operand,
        true_target: BlockId,
        false_target: BlockId,
    },
    Return {
        src: Option<Operand>,
    },
    Call {
        dest: Operand,
        src: Operand,
        caller_size: isize,
    },
    Print {
        src: Operand,
    },
}

impl CfgInstruction {
    pub fn add(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Add {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn subtract(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Subtract {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn multiply(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Multiply {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn divide(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Divide {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn modulo(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Modulo {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn equal(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Equal {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn not_equal(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::NotEqual {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn greater(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Greater {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn greater_equal(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::GreaterEqual {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn less(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Less {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn less_equal(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::LessEqual {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn negate(dest: impl Into<Operand>, src: impl Into<Operand>) -> Self {
        Self::Negate {
            dest: dest.into(),
            src: src.into(),
        }
    }

    pub fn not(dest: impl Into<Operand>, src: impl Into<Operand>) -> Self {
        Self::Not {
            dest: dest.into(),
            src: src.into(),
        }
    }

    pub fn move_(dest: impl Into<Operand>, src: impl Into<Operand>) -> Self {
        Self::Move {
            dest: dest.into(),
            src: src.into(),
        }
    }

    pub fn jump(target: BlockId) -> Self {
        Self::Jump { target }
    }

    pub fn conditional_jump(
        src: impl Into<Operand>,
        true_target: BlockId,
        false_target: BlockId,
    ) -> Self {
        Self::ConditionalJump {
            src: src.into(),
            true_target,
            false_target,
        }
    }

    pub fn return_(src: Option<impl Into<Operand>>) -> Self {
        Self::Return {
            src: src.map(|src| src.into()),
        }
    }

    pub fn call(dest: impl Into<Operand>, src: impl Into<Operand>, caller_size: isize) -> Self {
        Self::Call {
            dest: dest.into(),
            src: src.into(),
            caller_size,
        }
    }

    pub fn print(src: impl Into<Operand>) -> Self {
        Self::Print { src: src.into() }
    }
}
