use crate::frontend::scanner::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct UnaryOp {
    pub span: Span,
    pub kind: UnaryOpKind,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpKind {
    Negate,
    Not,
    Increment,
    Decrement,
}

impl UnaryOp {
    pub fn new(kind: UnaryOpKind, span: Span) -> UnaryOp {
        UnaryOp { span, kind }
    }
}
