#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: usize,
}

#[derive(Debug, Clone)]
#[repr(u8)]
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

impl Instruction {
    pub fn nullary(opcode: Opcode) -> Instruction {
        Instruction { opcode, operand: 0 }
    }

    pub fn unary(opcode: Opcode, operand: usize) -> Instruction {
        Instruction { opcode, operand }
    }
}
