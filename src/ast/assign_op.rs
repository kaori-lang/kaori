#[derive(Debug, Clone, Copy)]
pub struct AssignOp {
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
    PowerAssign,
}

impl AssignOp {
    pub fn new(kind: AssignOpKind) -> AssignOp {
        AssignOp { kind }
    }
}
