use std::collections::HashMap;

use super::{basic_block::BasicBlock, block_id::BlockId};

#[derive(Default)]
pub struct CfgIr {
    pub roots: Vec<BlockId>,
    pub basic_blocks: HashMap<BlockId, BasicBlock>,
}
