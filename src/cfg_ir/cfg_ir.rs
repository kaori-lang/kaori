use super::{
    basic_block::{BasicBlock, BlockId},
    cfg_constants::CfgConstants,
};

pub struct CfgIr {
    pub cfgs: Vec<BlockId>,
    pub basic_blocks: Vec<BasicBlock>,
    pub constants: CfgConstants,
}

impl CfgIr {
    pub fn new() -> Self {
        Self {
            cfgs: Vec::new(),
            basic_blocks: Vec::new(),
            constants: CfgConstants::new(),
        }
    }
}
