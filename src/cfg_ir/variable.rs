#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Variable {
    Number(i16),
    String(i16),
    Boolean(i16),
    Function(i16),
}

impl Variable {
    pub fn to_i16(self) -> i16 {
        match self {
            Variable::Number(value)
            | Variable::String(value)
            | Variable::Boolean(value)
            | Variable::Function(value) => value,
        }
    }
}
