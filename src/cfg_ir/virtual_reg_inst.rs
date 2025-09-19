use super::block_id::BlockId;

#[derive(Debug)]
pub enum VirtualRegInst {
    Add { dst: usize, r1: usize, r2: usize },
    Subtract { dst: usize, r1: usize, r2: usize },
    Multiply { dst: usize, r1: usize, r2: usize },
    Divide { dst: usize, r1: usize, r2: usize },
    Modulo { dst: usize, r1: usize, r2: usize },
    Equal { dst: usize, r1: usize, r2: usize },
    NotEqual { dst: usize, r1: usize, r2: usize },
    Greater { dst: usize, r1: usize, r2: usize },
    GreaterEqual { dst: usize, r1: usize, r2: usize },
    Less { dst: usize, r1: usize, r2: usize },
    LessEqual { dst: usize, r1: usize, r2: usize },
    And { dst: usize, r1: usize, r2: usize },
    Or { dst: usize, r1: usize, r2: usize },
    Negate { dst: usize, r1: usize },
    Not { dst: usize, r1: usize },
    StringConst { dst: usize, value: String },
    NumberConst { dst: usize, value: f64 },
    BooleanConst { dst: usize, value: bool },
    FunctionConst { dst: usize, value: BlockId },
    Move { dst: usize, r1: usize },
    Call,
    Return { r1: usize },
    Print,
}
