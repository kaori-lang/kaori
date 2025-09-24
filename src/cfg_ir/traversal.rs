use std::collections::{HashMap, HashSet};

use super::{
    basic_block::{BasicBlock, Terminator},
    block_id::BlockId,
};

pub struct Postorder<'a> {
    basic_blocks: &'a HashMap<BlockId, BasicBlock>,
    visited: HashSet<BlockId>,
    postorder: Vec<BlockId>,
}

impl<'a> Postorder<'a> {
    pub fn new(basic_blocks: &'a HashMap<BlockId, BasicBlock>) -> Self {
        Self {
            basic_blocks,
            visited: HashSet::new(),
            postorder: Vec::new(),
        }
    }

    pub fn reversed_postorder(&mut self, id: &BlockId) -> Vec<BlockId> {
        self.traverse(id);

        let mut reversed = Vec::new();

        while let Some(block_id) = self.postorder.pop() {
            reversed.push(block_id);
        }

        self.visited.clear();

        reversed
    }

    fn traverse(&mut self, id: &BlockId) {
        if self.visited.contains(id) {
            return;
        }

        let bb = self.basic_blocks.get(id).unwrap();

        match &bb.terminator {
            Terminator::Branch { r#true, r#false } => {
                self.traverse(r#false);
                self.traverse(r#true);
            }
            Terminator::Goto(target) => {
                self.traverse(target);
            }
            _ => {}
        };

        self.visited.insert(*id);
        self.postorder.push(*id);
    }
}
