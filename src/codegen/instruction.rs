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
