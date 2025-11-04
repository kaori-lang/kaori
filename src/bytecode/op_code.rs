#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Opcode {
    // === Arithmetic ===
    AddRR = 0,
    AddRK = 1,
    AddKR = 2,
    AddKK = 3,

    SubtractRR = 4,
    SubtractRK = 5,
    SubtractKR = 6,
    SubtractKK = 7,

    MultiplyRR = 8,
    MultiplyRK = 9,
    MultiplyKR = 10,
    MultiplyKK = 11,

    DivideRR = 12,
    DivideRK = 13,
    DivideKR = 14,
    DivideKK = 15,

    ModuloRR = 16,
    ModuloRK = 17,
    ModuloKR = 18,
    ModuloKK = 19,

    // === Comparison ===
    EqualRR = 20,
    EqualRK = 21,
    EqualKR = 22,
    EqualKK = 23,

    NotEqualRR = 24,
    NotEqualRK = 25,
    NotEqualKR = 26,
    NotEqualKK = 27,

    GreaterRR = 28,
    GreaterRK = 29,
    GreaterKR = 30,
    GreaterKK = 31,

    GreaterEqualRR = 32,
    GreaterEqualRK = 33,
    GreaterEqualKR = 34,
    GreaterEqualKK = 35,

    LessRR = 36,
    LessRK = 37,
    LessKR = 38,
    LessKK = 39,

    LessEqualRR = 40,
    LessEqualRK = 41,
    LessEqualKR = 42,
    LessEqualKK = 43,

    // === Unary ===
    NegateR = 44,
    NegateK = 45,
    NotR = 46,
    NotK = 47,

    // === Data movement ===
    MoveR = 48,
    MoveK = 49,

    // === Function and return ===
    CallR = 50,
    CallK = 51,
    ReturnR = 52,
    ReturnK = 53,
    ReturnVoid = 54,

    // === Control flow ===
    Jump = 55,
    JumpIfTrueR = 56,
    JumpIfTrueK = 57,
    JumpIfFalseR = 58,
    JumpIfFalseK = 59,

    // === IO ===
    PrintR = 60,
    PrintK = 61,

    // === Program termination ===
    Halt = 62,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
