use std::collections::HashSet;

use super::basic_block::{BasicBlock, Terminator};

pub fn reversed_postorder(basic_blocks: &[BasicBlock]) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut postorder = Vec::new();

    traverse(0, basic_blocks, &mut visited, &mut postorder);

    postorder.reverse();

    postorder
}

fn traverse(
    index: usize,
    basic_blocks: &[BasicBlock],
    visited: &mut HashSet<usize>,
    postorder: &mut Vec<usize>,
) {
    if visited.contains(&index) {
        return;
    }

    visited.insert(index);

    let bb = &basic_blocks[index];

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

    postorder.push(index);
}
