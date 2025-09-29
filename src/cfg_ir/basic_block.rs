use super::cfg_instruction::CfgInstruction;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<CfgInstruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            terminator: Terminator::None,
        }
    }
}

#[derive(Debug)]
pub enum Terminator {
    Branch { r#true: BlockId, r#false: BlockId },
    Goto(BlockId),
    Return,
    None,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockId(pub usize);

impl Default for BlockId {
    fn default() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
