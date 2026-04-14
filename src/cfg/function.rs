use std::fmt::{self, Display, Formatter};

use uuid::Uuid;

use crate::cfg::graph_traversal::reversed_postorder;

use super::{Constant, basic_block::BasicBlock};

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
    pub constants: Vec<Constant>,
    pub registers_count: usize,
}

impl Function {
    pub fn new(
        id: FunctionId,
        basic_blocks: Vec<BasicBlock>,
        constants: Vec<Constant>,
        registers_count: usize,
    ) -> Self {
        Self {
            id,
            basic_blocks,
            constants,
            registers_count,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Function:")?;

        writeln!(f, "constant pool: {:?}", self.constants)?;

        for index in reversed_postorder(&self.basic_blocks) {
            write!(f, "{}", &self.basic_blocks[index])?;
        }

        Ok(())
    }
}
