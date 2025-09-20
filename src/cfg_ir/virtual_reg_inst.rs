use super::block_id::BlockId;

#[derive(Debug)]
pub enum VirtualRegInst {
    Add {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Subtract {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Multiply {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Divide {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Modulo {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Equal {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    NotEqual {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Greater {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    GreaterEqual {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Less {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    LessEqual {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    And {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Or {
        dest: usize,
        src1: usize,
        src2: usize,
    },
    Negate {
        dest: usize,
        src1: usize,
    },
    Not {
        dest: usize,
        src1: usize,
    },
    StringConst {
        dest: usize,
        value: String,
    },
    NumberConst {
        dest: usize,
        value: f64,
    },
    BooleanConst {
        dest: usize,
        value: bool,
    },
    FunctionConst {
        dest: usize,
        value: BlockId,
    },
    Move {
        dest: usize,
        src1: usize,
    },
    Call,
    Return {
        source: usize,
    },
    Print,
}
