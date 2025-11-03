use std::fmt::{self, Display, Formatter};

use crate::cfg_ir::graph_traversal::reversed_postorder;

use super::{basic_block::BasicBlock, cfg_constants::CfgConstants};

#[derive(Default)]
pub struct CfgFunction {
    pub basic_blocks: Vec<BasicBlock>,
    pub constants: CfgConstants,
}

impl Display for CfgFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Control Flow Graph (CFG):")?;

        for index in reversed_postorder(&self.basic_blocks) {
            let block = &self.basic_blocks[index];
            write!(f, "{}", block)?;
        }

        Ok(())
    }
}
