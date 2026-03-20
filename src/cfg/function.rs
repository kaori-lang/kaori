use std::fmt::{self, Display, Formatter};

use crate::cfg::graph_traversal::reversed_postorder;

use super::{basic_block::BasicBlock, constants::Constant};

#[derive(Default)]
pub struct Function {
    pub basic_blocks: Vec<BasicBlock>,
    pub constant_pool: Vec<Constant>,
    pub allocated_variables: usize,
}

impl Function {
    pub fn new(
        basic_blocks: Vec<BasicBlock>,
        constant_pool: Vec<Constant>,
        allocated_variables: usize,
    ) -> Self {
        Self {
            basic_blocks,
            constant_pool,
            allocated_variables,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Control Flow Graph:")?;

        writeln!(f, "constant pool: {:?}", self.constant_pool)?;
        for index in reversed_postorder(&self.basic_blocks) {
            write!(f, "{}", &self.basic_blocks[index])?;
        }

        Ok(())
    }
}
