#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Opcode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Negate,
    Not,

    Move,

    Call,
    Return,
    ReturnVoid,

    Jump,
    JumpIfTrue,
    JumpIfFalse,

    Print,

    Halt,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
