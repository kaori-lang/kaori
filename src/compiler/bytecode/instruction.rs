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
    PushConst(String, f64, ),

    // Control flow
    Jump,
    JumpIfFalse,

    // I/O
    Print,
}
