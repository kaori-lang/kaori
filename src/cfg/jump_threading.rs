use std::collections::{HashMap, HashSet};

use super::{
    basic_block::{BasicBlock, Terminator},
    cfg_function::CfgFunction,
};

pub fn run_jump_threading_optimization(cfgs: &mut [CfgFunction]) {
    for cfg in cfgs {
        let mut visited = HashSet::new();
        let mut nodes = HashMap::new();

        traverse(0, &mut cfg.basic_blocks, &mut visited, &mut nodes);
    }
}

fn traverse(
    index: usize,
    basic_blocks: &mut [BasicBlock],
    visited: &mut HashSet<usize>,
    nodes: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(index) = visited.get(&index) {
        if let Some(index) = nodes.get(index) {
            return *index;
        };

        return *index;
    }

    visited.insert(index);

    let basic_block = &basic_blocks[index];

    let (bb_index, terminator) = &match basic_block.terminator {
        Terminator::Branch {
            src,
            r#true,
            r#false,
        } => {
            let terminator = Terminator::Branch {
                src,
                r#true: traverse(r#true, basic_blocks, visited, nodes),
                r#false: traverse(r#false, basic_blocks, visited, nodes),
            };

            (index, terminator)
        }
        Terminator::Goto(target) => {
            if basic_block.instructions.is_empty() {
                let terminator = Terminator::Goto(target);
                let index = traverse(target, basic_blocks, visited, nodes);

                (index, terminator)
            } else {
                let target = traverse(target, basic_blocks, visited, nodes);
                let terminator = Terminator::Goto(target);

                (index, terminator)
            }
        }
        Terminator::Return { src } => {
            let terminator = Terminator::Return { src };

            (index, terminator)
        }
        _ => unreachable!(),
    };

    nodes.insert(index, *bb_index);

    let basic_block = &mut basic_blocks[index];
    basic_block.terminator = *terminator;

    *bb_index
}
