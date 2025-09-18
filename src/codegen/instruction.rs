use crate::frontend::cfgir::register::Register;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Subtract {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Multiply {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Divide {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Modulo {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Equal {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    NotEqual {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Greater {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    GreaterEqual {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Less {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    LessEqual {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    And {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Or {
        dst: Register,
        r1: Register,
        r2: Register,
    },
    Negate {
        dst: Register,
        r1: Register,
    },
    Not {
        dst: Register,
        r1: Register,
    },
    LoadConst {
        dst: Register,
        r1: Register,
    },
    LoadLocal {
        dst: Register,
        r1: Register,
    },
    StoreLocal {
        dst: Register,
        r1: Register,
    },
    Call {
        registers: u8,
    },
    Return {
        r1: Register,
    },
    Jump(u16),
    JumpIfFalse(u16),
    Pop,
    Print,
}
