#[derive(Debug, Clone, Copy)]
pub struct UnaryOp {
    pub kind: UnaryOpKind,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpKind {
    Negate,
}

impl UnaryOp {
    pub fn new(kind: UnaryOpKind) -> UnaryOp {
        UnaryOp { kind }
    }
}
