use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Operand {
    Variable(usize),
    Constant(usize),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Variable(id) => write!(f, "r{}", id),
            Operand::Constant(id) => write!(f, "c{}", id),
        }
    }
}
