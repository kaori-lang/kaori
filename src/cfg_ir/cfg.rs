use super::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_instruction::CfgInstructionKind,
};

#[derive(Debug, Default)]
pub struct Cfg {
    pub basic_blocks: Vec<BasicBlock>,
    pub current_bb: BlockId,
}

impl Cfg {
    pub fn emit_instruction(&mut self, instruction: CfgInstructionKind) {
        let index = self.current_bb;
        let basic_block = &mut self.basic_blocks[index];

        if let Terminator::Return = basic_block.terminator {
            return;
        }

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
