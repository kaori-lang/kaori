use std::collections::{HashMap, HashSet};

use super::{
    basic_block::{BasicBlock, Terminator},
    block_id::BlockId,
};

pub struct Postorder {
    visited: HashSet<BlockId>,
    postorder: Vec<BlockId>,
}

impl Postorder {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            postorder: Vec::new(),
        }
    }

    pub fn reversed_postorder(
        &mut self,
        id: &BlockId,
        basic_blocks: &HashMap<BlockId, BasicBlock>,
    ) -> Vec<BlockId> {
        self.traverse(id, basic_blocks);

        let mut reversed = Vec::new();

        while let Some(block_id) = self.postorder.pop() {
            reversed.push(block_id);
        }

        self.visited.clear();

        reversed
    }

    fn traverse(&mut self, id: &BlockId, basic_blocks: &HashMap<BlockId, BasicBlock>) {
        if self.visited.contains(id) {
            return;
        }

        let bb = basic_blocks.get(id).unwrap();

        match &bb.terminator {
            Terminator::Branch { r#true, r#false } => {
                self.traverse(r#false, basic_blocks);
                self.traverse(r#true, basic_blocks);
            }
            Terminator::Goto(target) => {
                self.traverse(target, basic_blocks);
            }
            _ => {}
        };

        self.visited.insert(*id);
        self.postorder.push(*id);
    }
}
