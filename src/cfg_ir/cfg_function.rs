use std::fmt::{self, Display, Formatter};

use crate::cfg_ir::graph_traversal::reversed_postorder;

use super::{basic_block::BasicBlock, cfg_constants::CfgConstant};

#[derive(Default)]
pub struct CfgFunction {
    pub basic_blocks: Vec<BasicBlock>,
    pub constants: Vec<CfgConstant>,
    pub allocated_variables: usize,
}

impl CfgFunction {
    pub fn new(
        basic_blocks: Vec<BasicBlock>,
        constants: Vec<CfgConstant>,
        allocated_variables: usize,
    ) -> Self {
        Self {
            basic_blocks,
            constants,
            allocated_variables,
        }
    }
}

impl Display for CfgFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Control Flow Graph:")?;

        writeln!(f, "CONSTANTS:")?;
        writeln!(f, "{:#?}", self.constants)?;

        for index in reversed_postorder(&self.basic_blocks) {
            write!(f, "{}", &self.basic_blocks[index])?;
        }

        Ok(())
    }
}
