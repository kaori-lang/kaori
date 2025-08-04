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
    LoadLocal(i16),
    LoadGlobal(i16),
    StoreLocal(i16),
    StoreGlobal(i16),
    LoadConst(i16),

    EnterScope,
    ExitScope,

    Jump(i16),
    JumpIfFalse(i16),

    Print,

    Nothing,
}
