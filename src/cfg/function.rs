use std::fmt::{self, Display, Formatter};

use uuid::Uuid;

use crate::cfg::graph_traversal::reversed_postorder;

use super::{basic_block::BasicBlock, constant_pool::Constant};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FunctionId(Uuid);

impl Default for FunctionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct Function {
    pub id: FunctionId,
    pub basic_blocks: Vec<BasicBlock>,
    pub constant_pool: Vec<Constant>,
    pub registers_count: usize,
}

impl Function {
    pub fn new(
        id: FunctionId,
        basic_blocks: Vec<BasicBlock>,
        constant_pool: Vec<Constant>,
        registers_count: usize,
    ) -> Self {
        Self {
            id,
            basic_blocks,
            constant_pool,
            registers_count,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Function:")?;

        writeln!(f, "constant pool: {:?}", self.constant_pool)?;

        for index in reversed_postorder(&self.basic_blocks) {
            write!(f, "{}", &self.basic_blocks[index])?;
        }

        Ok(())
    }
}
