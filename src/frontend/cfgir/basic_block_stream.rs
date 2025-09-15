use super::{basic_block::BasicBlock, block_id::BlockId, cfg_instruction::CfgInstruction};

#[derive(Default)]
pub struct BasicBlockStream {
    pub basic_blocks: Vec<BasicBlock>,
    pub current_basic_block: BlockId,
}

impl BasicBlockStream {
    pub fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let index = self.current_basic_block.0;
        let basic_block = &mut self.basic_blocks[index];

        basic_block.instructions.push(instruction);
    }

    pub fn create_basic_block(&mut self) -> BlockId {
        let index = self.basic_blocks.len();

        let id = BlockId(index);

        let basic_block = BasicBlock::new(id);

        self.basic_blocks.push(basic_block);

        id
    }

    pub fn set_current(&mut self, id: BlockId) {
        self.current_basic_block = id;
    }

    pub fn get_basic_block(&mut self, id: BlockId) -> &mut BasicBlock {
        let index = id.0;

        &mut self.basic_blocks[index]
    }

    pub fn last(&self) -> BlockId {
        let index = self.basic_blocks.len() - 1;

        BlockId(index)
    }
}
