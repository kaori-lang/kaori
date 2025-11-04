#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // Arithmetic
    AddRR,
    AddRK,
    AddKR,
    AddKK,
    SubRR,
    SubRK,
    SubKR,
    SubKK,
    MulRR,
    MulRK,
    MulKR,
    MulKK,
    DivRR,
    DivRK,
    DivKR,
    DivKK,
    ModRR,
    ModRK,
    ModKR,
    ModKK,

    // Comparison
    EqRR,
    EqRK,
    EqKR,
    EqKK,
    NeRR,
    NeRK,
    NeKR,
    NeKK,
    GtRR,
    GtRK,
    GtKR,
    GtKK,
    GeRR,
    GeRK,
    GeKR,
    GeKK,
    LtRR,
    LtRK,
    LtKR,
    LtKK,
    LeRR,
    LeRK,
    LeKR,
    LeKK,

    // Unary
    NegR,
    NegK,
    NotR,
    NotK,

    // Data movement
    MoveRR,
    MoveRK,

    // Function and return
    Call,
    ReturnR,
    ReturnVoid,

    // Control flow
    Jump,
    JumpIfTrue,
    JumpIfFalse,

    // IO
    PrintR,
    PrintK,

    // Program termination
    Halt,
}
