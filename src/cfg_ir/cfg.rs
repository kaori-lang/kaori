use std::collections::HashMap;

use super::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_instruction::{CfgInstruction, CfgInstructionId, CfgInstructionKind},
};

#[derive(Debug, Default)]
pub struct Cfg {
    pub basic_blocks: HashMap<BlockId, BasicBlock>,
    pub current_bb: BlockId,
    pub instruction_id: CfgInstructionId,
}

impl Cfg {
    pub fn emit_instruction(&mut self, kind: CfgInstructionKind) {
        let index = self.current_bb;
        let basic_block = &mut self.basic_blocks[index];

        if let Terminator::Return = basic_block.terminator {
            return;
        }

        let instruction = CfgInstruction {
            id: self.instruction_id,
            kind,
        };

        self.instruction_id += 1;

        basic_block.instructions.push(instruction);
    }

    pub fn create_bb(&mut self) -> BlockId {
        let id = self.basic_blocks.len();
        let basic_block = BasicBlock::new(id);

        self.basic_blocks.push(basic_block);

        id
    }

    pub fn get_bb(&mut self, id: BlockId) -> &mut BasicBlock {
        &mut self.basic_blocks[id]
    }
}
