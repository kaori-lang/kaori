pub struct BasicBlock {
    instructions: Vec<Instruction>,
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
pub enum Instruction {
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
    LoadConst(u16),
    LoadLocal(u16),
    StoreLocal(u16),

    EnterScope,
    ExitScope,
    Call,
    Return,

    Jump(u16),
    JumpIfFalse(u16),
    Pop,
    Print,

    Nothing,
}
