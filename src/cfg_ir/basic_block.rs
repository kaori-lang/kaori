use core::fmt;

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
}

#[derive(Debug)]
pub enum Terminator {
    Branch { r#true: BlockId, r#false: BlockId },
    Goto(BlockId),
    Return,
    None,
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminator::Branch { r#true, r#false } => {
                write!(f, "branch to {:?} / {:?}", r#true, r#false)
            }
            Terminator::Goto(target) => write!(f, "goto {:?}", target),
            Terminator::Return => write!(f, "return"),
            Terminator::None => write!(f, "<no terminator>"),
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BasicBlock {:?}:", self.id)?;
        for instr in &self.instructions {
            writeln!(f, "  {}", instr)?;
        }
        writeln!(f, "  Terminator: {}", self.terminator)
    }
}
