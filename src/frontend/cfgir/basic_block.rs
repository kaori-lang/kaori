use super::{block_id::BlockId, cfg_instruction::CfgInstruction};

pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<CfgInstruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            terminator: Terminator::None,
        }
    }
    pub fn add_instruction(&mut self, instruction: CfgInstruction) {
        self.instructions.push(instruction);
    }
}

pub enum Terminator {
    Conditional {
        then_block: usize,
        else_block: usize,
    },
    Jump(usize),
    None,
}
