use std::collections::HashMap;

use super::{
    basic_block::{BasicBlock, Terminator},
    block_id::BlockId,
    cfg_instruction::{CfgInstruction, CfgInstructionKind},
};

#[derive(Debug, Default)]
pub struct CfgStream {
    pub roots: Vec<BlockId>,
    pub basic_blocks: HashMap<BlockId, BasicBlock>,
    pub current_bb: BlockId,
}

impl CfgStream {
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            basic_blocks: HashMap::new(),
            current_bb: BlockId(0),
        }
    }

    pub fn emit_instruction(&mut self, kind: CfgInstructionKind) {
        let id = self.current_bb;
        let basic_block = self.basic_blocks.get_mut(&id).unwrap();

        if let Terminator::Return = basic_block.terminator {
            return;
        }

        let instruction = CfgInstruction::new(kind);

        basic_block.instructions.push(instruction);
    }

    pub fn create_bb(&mut self) -> BlockId {
        let id = BlockId::default();
        let basic_block = BasicBlock::new(id);

        self.basic_blocks.insert(id, basic_block);

        id
    }

    pub fn create_cfg(&mut self) -> BlockId {
        let id = BlockId::default();
        let basic_block = BasicBlock::new(id);

        self.roots.push(id);
        self.basic_blocks.insert(id, basic_block);

        id
    }

    pub fn get_bb(&mut self, id: BlockId) -> &mut BasicBlock {
        self.basic_blocks.get_mut(&id).unwrap()
    }
}
