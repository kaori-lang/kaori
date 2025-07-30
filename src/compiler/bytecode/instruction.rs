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
    PushConst(String, f64, bool),

    // Control flow
    Jump,
    JumpIfFalse,

    // I/O
    Print,
}

impl Instruction {
    pub fn bool_const(value: bool) -> Instruction {
        Instruction::PushConst(String::from(""), 0.0, value)
    }

    pub fn str_const(value: &str) -> Instruction {
        Instruction::PushConst(String::from(value), 0.0, false)
    }

    pub fn number_const(value: f64) -> Instruction {
        Instruction::PushConst(String::from(""), value, false)
    }
}
