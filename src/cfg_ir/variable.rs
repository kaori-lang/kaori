#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Variable {
    Number(i16),
    String(i16),
    Boolean(i16),
    FunctionRef(i16),
}
