use std::collections::HashSet;

use super::basic_block::{BasicBlock, BlockId, Terminator};

pub fn reversed_postorder(root: BlockId, basic_blocks: &[BasicBlock]) -> Vec<BlockId> {
    let mut visited = HashSet::new();
    let mut postorder = Vec::new();

    traverse(root, basic_blocks, &mut visited, &mut postorder);

    postorder.reverse();

    postorder
}

fn traverse(
    id: BlockId,
    basic_blocks: &[BasicBlock],
    visited: &mut HashSet<BlockId>,
    postorder: &mut Vec<BlockId>,
) {
    if visited.contains(&id) {
        return;
    }

    visited.insert(id);

    let bb = &basic_blocks[id.0];

    match bb.terminator {
        Terminator::Branch {
            r#true, r#false, ..
        } => {
            traverse(r#false, basic_blocks, visited, postorder);
            traverse(r#true, basic_blocks, visited, postorder);
        }
        Terminator::Goto(target) => {
            traverse(target, basic_blocks, visited, postorder);
        }
        _ => {}
    };

    postorder.push(id);
}
