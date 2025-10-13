#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(pub i16);

impl Variable {
    pub fn to_i16(self) -> i16 {
        self.0
    }
}
