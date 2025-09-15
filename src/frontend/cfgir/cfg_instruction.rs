use super::register::Register;

#[derive(Debug)]
pub enum CfgInstruction {
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
    StringConst {
        dst: Register,
        value: String,
    },
    NumberConst {
        dst: Register,
        value: f64,
    },
    BooleanConst {
        dst: Register,
        value: bool,
    },

    LoadLocal {
        dst: Register,
        r1: Register,
    },
    StoreLocal {
        dst: Register,
        r1: Register,
    },
    Call,
    Return {
        dst: Register,
        r1: Register,
    },
    Print,
}
