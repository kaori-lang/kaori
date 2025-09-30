use super::{cfg_instruction::CfgInstruction, operand::Operand};

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
}

#[derive(Debug)]
pub enum Terminator {
    Branch {
        src: Operand,
        r#true: BlockId,
        r#false: BlockId,
    },
    Goto(BlockId),
    Return {
        src: Option<Operand>,
    },
    None,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockId(pub usize);
