use super::{basic_block::BlockId, operand::Operand};

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
    And {
        dest: Operand,
        src1: Operand,
        src2: Operand,
    },
    Or {
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
    StringConst {
        dest: Operand,
        value: String,
    },
    NumberConst {
        dest: Operand,
        value: f64,
    },
    BooleanConst {
        dest: Operand,
        value: bool,
    },
    FunctionConst {
        dest: Operand,
        value: BlockId,
    },
    Move {
        dest: Operand,
        src: Operand,
    },
    Jump {
        target: BlockId,
    },
    JumpIfFalse {
        src: Operand,
        target: BlockId,
    },
    Return {
        src: Option<Operand>,
    },
    Call,
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

    pub fn and(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::And {
            dest: dest.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn or(
        dest: impl Into<Operand>,
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
    ) -> Self {
        Self::Or {
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

    pub fn string_const(dest: impl Into<Operand>, value: impl Into<String>) -> Self {
        Self::StringConst {
            dest: dest.into(),
            value: value.into(),
        }
    }

    pub fn number_const(dest: impl Into<Operand>, value: f64) -> Self {
        Self::NumberConst {
            dest: dest.into(),
            value,
        }
    }

    pub fn boolean_const(dest: impl Into<Operand>, value: bool) -> Self {
        Self::BooleanConst {
            dest: dest.into(),
            value,
        }
    }

    pub fn function_const(dest: impl Into<Operand>, value: BlockId) -> Self {
        Self::FunctionConst {
            dest: dest.into(),
            value,
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

    pub fn jump_if_false(src: impl Into<Operand>, target: BlockId) -> Self {
        Self::JumpIfFalse {
            src: src.into(),
            target,
        }
    }

    pub fn return_(src: Option<impl Into<Operand>>) -> Self {
        Self::Return {
            src: src.map(|src| src.into()),
        }
    }

    pub fn call() -> Self {
        Self::Call
    }

    pub fn print(src: impl Into<Operand>) -> Self {
        Self::Print { src: src.into() }
    }
}
