use std::collections::{HashMap, HashSet};

use super::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_ir::CfgIr,
};

pub fn run_jump_threading_optimization(cfg_ir: &mut CfgIr) {
    let mut visited = HashSet::new();
    let mut nodes = HashMap::new();

    for cfg in &cfg_ir.cfgs {
        traverse(*cfg, &mut cfg_ir.basic_blocks, &mut visited, &mut nodes);
    }
}

fn traverse(
    id: BlockId,
    basic_blocks: &mut [BasicBlock],
    visited: &mut HashSet<BlockId>,
    nodes: &mut HashMap<BlockId, BlockId>,
) -> BlockId {
    if let Some(id) = visited.get(&id) {
        if let Some(id) = nodes.get(id) {
            return *id;
        };

        return *id;
    }

    visited.insert(id);

    let basic_block = &basic_blocks[id.0];

    let (bb_id, terminator) = &match basic_block.terminator {
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

            (id, terminator)
        }
        Terminator::Goto(target) => {
            if basic_block.instructions.is_empty() {
                let terminator = Terminator::Goto(target);
                let id = traverse(target, basic_blocks, visited, nodes);

                (id, terminator)
            } else {
                let target = traverse(target, basic_blocks, visited, nodes);
                let terminator = Terminator::Goto(target);

                (id, terminator)
            }
        }
        Terminator::Return { src } => {
            let terminator = Terminator::Return { src };

            (id, terminator)
        }
        _ => unreachable!(),
    };

    nodes.insert(id, *bb_id);

    let basic_block = &mut basic_blocks[id.0];
    basic_block.terminator = *terminator;

    *bb_id
}
