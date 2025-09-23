use super::{
    basic_block::{BasicBlock, BlockId, Terminator},
    virtual_reg_inst::VirtualRegInst,
};

#[derive(Default, Debug)]
pub struct Cfg {
    pub basic_blocks: Vec<BasicBlock>,
    pub current_bb: BlockId,
}

impl Cfg {
    pub fn new() -> Self {
        let id = BlockId(0);
        let basic_block = BasicBlock::new(id);

        Self {
            basic_blocks: vec![basic_block],
            current_bb: id,
        }
    }

    pub fn emit_instruction(&mut self, instruction: VirtualRegInst) {
        let index = self.current_bb.0;
        let basic_block = &mut self.basic_blocks[index];

        if let Terminator::Return = basic_block.terminator {
            return;
        }

        basic_block.instructions.push(instruction);
    }

    pub fn create_bb(&mut self) -> BlockId {
        let index = self.basic_blocks.len();

        let id = BlockId(index);

        let basic_block = BasicBlock::new(id);

        self.basic_blocks.push(basic_block);

        id
    }

    pub fn get_bb(&mut self, id: BlockId) -> &mut BasicBlock {
        let index = id.0;

        &mut self.basic_blocks[index]
    }
}
