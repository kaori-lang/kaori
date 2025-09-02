#[derive(Debug, Clone)]
pub struct UnaryOp {
    span: Span,
    kind: UnaryOpKind,
}

#[derive(Debug, Clone)]
pub enum UnaryOpKind {
    Negate,
    Not,
    Increment,
    Decrement,
}

impl UnaryOp {
    pub fn new(kind: UnaryOpKind, span: Span) -> UnaryOp {
        BinaryOp { span, kind }
    }
}
