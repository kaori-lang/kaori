use crate::lexer::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct BinaryOp {
    pub span: Span,
    pub kind: BinaryOpKind,
}

#[derive(Debug, Clone, Copy)]
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
