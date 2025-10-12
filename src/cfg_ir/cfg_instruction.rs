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
    Jump {
        target: BlockId,
    },
    ConditionalJump {
        src: Variable,
        true_target: BlockId,
        false_target: BlockId,
    },
    Return {
        src: Option<Variable>,
    },
    Call {
        dest: Variable,
        src: Variable,
        caller_size: isize,
    },
    Print {
        src: Variable,
    },
}

impl CfgInstruction {
    pub fn add(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Add {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn subtract(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Subtract {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn multiply(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Multiply {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn divide(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Divide {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn modulo(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Modulo {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Equal {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn not_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::NotEqual {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn greater(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Greater {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn greater_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::GreaterEqual {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn less(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::Less {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn less_equal(dest: Variable, src1: Variable, src2: Variable) -> Self {
        Self::LessEqual {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn negate(dest: Variable, src: Variable) -> Self {
        Self::Negate {
            dest: dest.into(),
            src: src.into(),
        }
    }

    pub fn not(dest: Variable, src: Variable) -> Self {
        Self::Not {
            dest: dest.into(),
            src: src.into(),
        }
    }

    pub fn move_(dest: Variable, src: Variable) -> Self {
        Self::Move {
            dest: dest.into(),
            src: src.into(),
        }
    }

    pub fn jump(target: BlockId) -> Self {
        Self::Jump { target }
    }

    pub fn conditional_jump(src: Variable, true_target: BlockId, false_target: BlockId) -> Self {
        Self::ConditionalJump {
            src: src.into(),
            true_target,
            false_target,
        }
    }

    pub fn return_(src: Option<Variable>) -> Self {
        Self::Return {
            src: src.map(|src| src.into()),
        }
    }

    pub fn call(dest: Variable, src: Variable, caller_size: isize) -> Self {
        Self::Call {
            dest: dest.into(),
            src: src.into(),
            caller_size,
        }
    }

    pub fn print(src: Variable) -> Self {
        Self::Print { src: src.into() }
    }
}
