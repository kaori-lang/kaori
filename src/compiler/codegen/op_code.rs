#[derive(Debug)]
pub enum Opcode {
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
    LoadLocal,
    LoadGlobal,
    StoreLocal,
    StoreGlobal,

    // Scope control
    EnterScope,
    ExitScope,

    // Constants
    PushConst,

    // Control flow
    Jump,
    JumpIfFalse,

    // I/O
    Print,

    // Do nothing
    Nothing,
}
