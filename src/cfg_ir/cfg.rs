use super::{
    basic_block::{BasicBlock, BlockId, Terminator},
    virtual_reg_inst::VirtualRegInst,
};

#[derive(Debug, Default)]
pub struct Cfg {
    pub basic_blocks: Vec<BasicBlock>,
    pub current_bb: BlockId,
}

impl Cfg {
    pub fn emit_instruction(&mut self, instruction: VirtualRegInst) {
        let index = self.current_bb.0;
        let basic_block = &mut self.basic_blocks[index];

        if let Terminator::Return = basic_block.terminator {
            return;
        }

        basic_block.instructions.push(instruction);
    }

    pub fn create_bb(&mut self) -> BlockId {
        let id = BlockId(self.basic_blocks.len());
        let basic_block = BasicBlock::new(id);
        let id = basic_block.id;

        self.basic_blocks.push(basic_block);

        id
    }

    pub fn get_bb(&mut self, id: BlockId) -> &mut BasicBlock {
        let index = id.0;

        &mut self.basic_blocks[index]
    }
}
