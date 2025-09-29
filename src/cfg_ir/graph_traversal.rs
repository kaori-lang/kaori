use std::collections::{HashMap, HashSet};

use super::basic_block::{BasicBlock, BlockId, Terminator};

pub fn reversed_postorder(
    id: &BlockId,
    basic_blocks: &HashMap<BlockId, BasicBlock>,
) -> Vec<BlockId> {
    let mut visited = HashSet::new();
    let mut postorder = Vec::new();

    traverse(id, basic_blocks, &mut visited, &mut postorder);

    postorder.reverse();

    postorder
}

fn traverse(
    id: &BlockId,
    basic_blocks: &HashMap<BlockId, BasicBlock>,
    visited: &mut HashSet<BlockId>,
    postorder: &mut Vec<BlockId>,
) {
    if visited.contains(id) {
        return;
    }

    let bb = basic_blocks.get(id).unwrap();

    match &bb.terminator {
        Terminator::Branch { r#true, r#false } => {
            traverse(r#false, basic_blocks, visited, postorder);
            traverse(r#true, basic_blocks, visited, postorder);
        }
        Terminator::Goto(target) => {
            traverse(target, basic_blocks, visited, postorder);
        }
        _ => {}
    };

    visited.insert(*id);
    postorder.push(*id);
}
