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

    Jump(i16),
    JumpIfFalse(i16),
    Pop,
    Print,

    Nothing,
}
