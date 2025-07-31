#[derive(Debug)]
pub enum Instruction {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Logical
    And,
    Or,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Unary
    Not,
    Negate,

    // Variable operations
    Declare,
    LoadLocal(usize),
    LoadGlobal(usize),
    StoreLocal(usize),
    StoreGlobal(usize),

    // Scope control
    EnterScope,
    ExitScope,

    // Constants
    PushConst(usize),

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),

    // I/O
    Print,

    // Do nothing
    Nothing,
}
