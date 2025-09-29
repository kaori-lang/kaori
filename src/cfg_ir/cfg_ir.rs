use super::basic_block::{BasicBlock, BlockId};

#[derive(Default)]
pub struct CfgIr {
    pub cfgs: Vec<BlockId>,
    pub basic_blocks: Vec<BasicBlock>,
}
