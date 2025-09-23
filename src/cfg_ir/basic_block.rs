use super::virtual_reg_inst::VirtualRegInst;

pub type BlockId = usize;

#[derive(Debug)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<VirtualRegInst>,
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
