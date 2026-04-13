use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Register(pub usize);

impl Register {
    #[inline(always)]
    pub fn to_u8(self) -> u8 {
        self.0 as u8
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}
