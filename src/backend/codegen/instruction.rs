pub struct Instruction {
    opcode: Opcode,
    operand: u8,
}

#[derive(Debug, Clone)]
pub enum Opcode {
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
    LoadLocal,
    LoadGlobal,
    StoreLocal,
    StoreGlobal,
    LoadConst,

    EnterScope,
    ExitScope,

    Jump,
    JumpIfFalse,

    Print,

    Nothing,
}
