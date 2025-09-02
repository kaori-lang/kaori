#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Subtract,
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

    Call { arguments_size: u8, frame_size: u8 },
    Return,

    Jump(u16),
    JumpIfFalse(u16),
    Pop,
    Print,

    Nothing,
}
