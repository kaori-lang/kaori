#[derive(Debug)]
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
    LoadLocal(usize),
    LoadGlobal(usize),
    StoreLocal(usize),
    StoreGlobal(usize),
    LoadConst(usize),

    EnterScope,
    ExitScope,

    Jump(usize),
    JumpIfFalse(usize),

    // I/O
    Print,

    // Do nothing
    Nothing,
}
