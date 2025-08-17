pub struct BasicBlock {
    instructions: Vec<CfgInstruction>,
    terminator: Terminator,
}

pub enum Terminator {
    Conditional {
        then_branch: usize,
        else_branch: usize,
    },
    Jump(usize),
}

#[derive(Debug, Clone)]
pub enum CfgInstruction {
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

    Declare,
    StringConst(String),
    NumberConst(f64),
    BooleanConst(bool),
    FunctionConst { function_id: usize },
    LoadLocal(usize),
    StoreLocal(usize),
    EnterScope,
    ExitScope,
    Call,
    Return,
    Pop,
    Print,
}
