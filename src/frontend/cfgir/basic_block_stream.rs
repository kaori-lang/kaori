use super::{basic_block::BasicBlock, cfg_instruction::CfgInstruction};

pub struct BasicBlockStream {
    pub basic_blocks: Vec<BasicBlock>,
    pub current_bb: usize,
}

impl BasicBlockStream {
    pub fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let basic_block = &mut self.basic_blocks[self.current_bb];

        basic_block.instructions.push(instruction);
    }
}
