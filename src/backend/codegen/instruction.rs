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
    LoadGlobal(u16),
    StoreLocal(u16),
    StoreGlobal(u16),
    EnterScope,
    ExitScope,
    EnterFunction,
    ExitFunction,

    Jump(i16),
    JumpIfFalse(i16),

    Print,

    Nothing,
}
