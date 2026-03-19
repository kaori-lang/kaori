use super::{instruction::Instruction, operand::Operand};
use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct BasicBlock {
    pub index: usize,
    pub instructions: Vec<Instruction>,
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
                write!(f, "{}: true -> BB{}; false -> BB{}", src, r#true, r#false)
            }
            Terminator::Goto(target) => write!(f, "goto BB{}", target),
            Terminator::Return { src } => {
                if let Some(operand) = src {
                    write!(f, "return {}", operand)
                } else {
                    write!(f, "return VOID")
                }
            }
            Terminator::None => write!(f, "<no terminator>"),
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "BB{}:", self.index)?;
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction)?;
        }
        writeln!(f, "  {}", self.terminator)
    }
}
