#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // === Arithmetic ===
    AddRR,
    AddRK,
    AddKR,
    AddKK,

    SubtractRR,
    SubtractRK,
    SubtractKR,
    SubtractKK,

    MultiplyRR,
    MultiplyRK,
    MultiplyKR,
    MultiplyKK,

    DivideRR,
    DivideRK,
    DivideKR,
    DivideKK,

    ModuloRR,
    ModuloRK,
    ModuloKR,
    ModuloKK,

    // === Comparison ===
    EqualRR,
    EqualRK,
    EqualKR,
    EqualKK,

    NotEqualRR,
    NotEqualRK,
    NotEqualKR,
    NotEqualKK,

    GreaterRR,
    GreaterRK,
    GreaterKR,
    GreaterKK,

    GreaterEqualRR,
    GreaterEqualRK,
    GreaterEqualKR,
    GreaterEqualKK,

    LessRR,
    LessRK,
    LessKR,
    LessKK,

    LessEqualRR,
    LessEqualRK,
    LessEqualKR,
    LessEqualKK,

    // === Unary ===
    NegateR,
    NegateK,
    NotR,
    NotK,

    // === Data movement ===
    MoveR,
    MoveK,

    // === Function and return ===
    CallR,
    CallK,
    ReturnR,
    ReturnK,
    ReturnVoid,

    // === Control flow ===
    Jump,
    JumpIfTrue,
    JumpIfFalse,

    // === IO ===
    PrintR,
    PrintK,

    // === Program termination ===
    Halt,
}
