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

#[derive(Debug)]
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
        src: usize,
    },
    Not {
        dest: usize,
        src: usize,
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
        src: usize,
    },
    Call,
    Return {
        src: usize,
    },
    Print,
}

impl fmt::Display for CfgInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            CfgInstructionKind::Add { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} + r{src2}")
            }
            CfgInstructionKind::Subtract { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} - r{src2}")
            }
            CfgInstructionKind::Multiply { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} * r{src2}")
            }
            CfgInstructionKind::Divide { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} / r{src2}")
            }
            CfgInstructionKind::Modulo { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} % r{src2}")
            }
            CfgInstructionKind::Equal { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} == r{src2}")
            }
            CfgInstructionKind::NotEqual { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} != r{src2}")
            }
            CfgInstructionKind::Greater { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} > r{src2}")
            }
            CfgInstructionKind::GreaterEqual { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} >= r{src2}")
            }
            CfgInstructionKind::Less { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} < r{src2}")
            }
            CfgInstructionKind::LessEqual { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} <= r{src2}")
            }
            CfgInstructionKind::And { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} && r{src2}")
            }
            CfgInstructionKind::Or { dest, src1, src2 } => {
                write!(f, "r{dest} = r{src1} || r{src2}")
            }
            CfgInstructionKind::Negate { dest, src } => write!(f, "r{dest} = -r{src}"),
            CfgInstructionKind::Not { dest, src } => write!(f, "r{dest} = !r{src}"),
            CfgInstructionKind::StringConst { dest, value } => write!(f, "r{dest} = \"{value}\""),
            CfgInstructionKind::NumberConst { dest, value } => write!(f, "r{dest} = {value}"),
            CfgInstructionKind::BooleanConst { dest, value } => write!(f, "r{dest} = {value}"),
            CfgInstructionKind::FunctionConst { dest, value } => {
                write!(f, "r{dest} = fn({value:?})")
            }
            CfgInstructionKind::Move { dest, src } => write!(f, "r{dest} = r{src}"),
            CfgInstructionKind::Call => write!(f, "call"),
            CfgInstructionKind::Return { src } => write!(f, "return r{src}"),
            CfgInstructionKind::Print => write!(f, "print"),
        }
    }
}
