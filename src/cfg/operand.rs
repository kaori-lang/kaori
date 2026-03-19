use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Operand {
    Variable(usize),
    Constant(usize),
}

impl Operand {
    pub fn to_i16(self) -> i16 {
        match self {
            Self::Constant(value) => -((value + 1) as i16),
            Self::Variable(value) => value as i16,
        }
    }

    pub fn to_u16(self) -> u16 {
        match self {
            Self::Variable(value) => value as u16,
            Self::Constant(_) => panic!("Tried to use constant as destination"),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(id) => write!(f, "r{}", id),
            Self::Constant(id) => write!(f, "k{}", id),
        }
    }
}
