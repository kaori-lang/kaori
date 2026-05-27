#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Ref,
    Deref,
}

#[derive(Debug, Clone, Copy)]
pub enum CompoundOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
