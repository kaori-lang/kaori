use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Operand {
    Variable(usize),
    Constant(usize),
    None,
}

impl Operand {
    pub fn to_i16(self) -> i16 {
        match self {
            Self::Constant(value) => -((value + 1) as i16),
            Self::Variable(value) => value as i16,
            Self::None => unreachable!("Tried to convert invalid op"),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(id) => write!(f, "r{}", id),
            Self::Constant(id) => write!(f, "k{}", id),
            Self::None => Ok(()),
        }
    }
}
