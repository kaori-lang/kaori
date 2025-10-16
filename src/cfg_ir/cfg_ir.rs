use std::fmt::{self, Display, Formatter};

use crate::cfg_ir::graph_traversal::reversed_postorder;

use super::{
    basic_block::{BasicBlock, BlockId},
    cfg_constants::CfgConstants,
};

#[derive(Default)]
pub struct CfgIr {
    pub cfgs: Vec<BlockId>,
    pub basic_blocks: Vec<BasicBlock>,
    pub constants: CfgConstants,
}

impl Display for CfgIr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Control Flow Graph (CFG):")?;

        for cfg in &self.cfgs {
            for bb in reversed_postorder(*cfg, &self.basic_blocks) {
                let block = &self.basic_blocks[bb.0];
                write!(f, "{}", block)?;
            }
        }

        Ok(())
    }
}
