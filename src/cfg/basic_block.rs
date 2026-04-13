use super::{instruction::Instruction, register::Register};
use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug, Clone, Copy)]
pub enum Terminator {
    Branch {
        src: Register,
        r#true: usize,
        r#false: usize,
    },
    Goto(usize),
    Return {
        src: Register,
    },
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
                write!(f, "return {}", src)
            }
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "BB:")?;
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction)?;
        }

        if let Some(terminator) = self.terminator {
            writeln!(f, "  {}", terminator)
        } else {
            write!(f, "<no terminator>")
        }
    }
}
