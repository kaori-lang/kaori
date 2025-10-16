use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(pub i16);

impl Variable {
    pub fn to_i16(self) -> i16 {
        self.0
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            write!(f, "c{}", -self.0)
        } else {
            write!(f, "r{}", self.0)
        }
    }
}
