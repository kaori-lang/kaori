use core::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::block_id::BlockId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CfgInstructionId(usize);

impl Default for CfgInstructionId {
    fn default() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct CfgInstruction {
    pub id: CfgInstructionId,
    pub kind: CfgInstructionKind,
}

impl CfgInstruction {
    pub fn new(kind: CfgInstructionKind) -> Self {
        Self {
            id: CfgInstructionId::default(),
            kind,
        }
    }
}

#[derive(Debug)]
pub enum CfgInstructionKind {
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
    Call,
    Return {
        src: Operand,
    },
    Print,
}
