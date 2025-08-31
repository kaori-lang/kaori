use crate::frontend::hir::node_id::NodeId;

#[derive(Default)]
pub struct BasicBlock {
    pub instructions: Vec<CfgInst>,
    pub terminator: Terminator,
}

pub enum Terminator {
    Conditional {
        then_branch: usize,
        else_branch: Option<usize>,
    },
    Jump(usize),
    None,
}

impl Default for Terminator {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum CfgInst {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Not,
    Negate,

    StringConst(String),
    NumberConst(f64),
    BooleanConst(bool),
    LoadGlobal(NodeId),
    LoadLocal(usize),
    StoreLocal(usize),

    Call,
    Return,
    Pop,
    Print,
}
