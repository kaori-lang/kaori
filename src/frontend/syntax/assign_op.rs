use crate::frontend::lexer::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct AssignOp {
    pub span: Span,
    pub kind: AssignOpKind,
}

#[derive(Debug, Clone, Copy)]
pub enum AssignOpKind {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
}

impl AssignOp {
    pub fn new(kind: AssignOpKind, span: Span) -> AssignOp {
        AssignOp { span, kind }
    }
}
