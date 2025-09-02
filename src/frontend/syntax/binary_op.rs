use crate::frontend::scanner::span::Span;

#[derive(Debug, Clone)]
pub struct BinaryOp {
    span: Span,
    kind: BinaryOpKind,
}

#[derive(Debug, Clone)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl BinaryOp {
    pub fn new(kind: BinaryOpKind, span: Span) -> BinaryOp {
        BinaryOp { span, kind }
    }
}
