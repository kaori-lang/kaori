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

    LoadConst(u16),
    LoadLocal(u16),
    StoreLocal(u16),

    Call,
    Return,

    Jump(u16),
    JumpIfFalse(u16),
    Pop,
    Print,

    Nothing,
}
