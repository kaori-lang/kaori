#[derive(Debug, Clone, Copy)]
pub struct BinaryOp {
    pub kind: BinaryOpKind,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpKind {
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

impl BinaryOp {
    pub fn new(kind: BinaryOpKind) -> BinaryOp {
        BinaryOp { kind }
    }
}
