use super::{cfg_instruction::CfgInstruction, operand::Operand};
use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct BasicBlock {
    pub index: usize,
    pub instructions: Vec<CfgInstruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            instructions: Vec::new(),
            terminator: Terminator::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Terminator {
    Branch {
        src: Operand,
        r#true: usize,
        r#false: usize,
    },
    Goto(usize),
    Return {
        src: Option<Operand>,
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
                write!(f, "br {} -> BB{}, BB{}", src, r#true, r#false)
            }
            Terminator::Goto(target) => write!(f, "goto BB{}", target),
            Terminator::Return { src } => write!(f, "ret {}", 1),
            Terminator::None => write!(f, "<no terminator>"),
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "BB{}:", self.index)?;
        for instr in &self.instructions {
            writeln!(f, "  {}", instr)?;
        }
        writeln!(f, "  {}", self.terminator)
    }
}
