use std::sync::atomic::{AtomicUsize, Ordering};

use super::virtual_reg_inst::VirtualRegInst;

#[derive(Debug, Clone, Copy)]
pub struct BlockId(pub usize);

impl Default for BlockId {
    fn default() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<VirtualRegInst>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            id: BlockId::default(),
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
