use super::{cfg_instruction::CfgInstruction, variable::Variable};
use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockId(pub usize);

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
pub enum Terminator {
    Branch {
        src: Variable,
        r#true: BlockId,
        r#false: BlockId,
    },
    Goto(BlockId),
    Return {
        src: Option<Variable>,
    },
    None,
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Terminator::Branch {
                src,
                r#true,
                r#false,
            } => {
                write!(f, "br {} -> BB{}, BB{}", src, r#true.0, r#false.0)
            }
            Terminator::Goto(target) => write!(f, "goto BB{}", target.0),
            Terminator::Return { src } => write!(f, "ret {}", 1),
            Terminator::None => write!(f, "<no terminator>"),
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "BB{}:", self.id.0)?;
        for instr in &self.instructions {
            writeln!(f, "  {}", instr)?;
        }
        writeln!(f, "  {}", self.terminator)
    }
}
