use super::{block_id::BlockId, cfg_instruction::CfgInstruction};

#[derive(Debug)]
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
    Conditional { then_bb: BlockId, else_bb: BlockId },
    Jump(BlockId),
    JumpIfFalse(BlockId),
    None,
    Return,
}
