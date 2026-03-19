pub enum Instruction {
    // Arithmetic
    Add { dest: u16, src1: i16, src2: i16 },
    Subtract { dest: u16, src1: i16, src2: i16 },
    Multiply { dest: u16, src1: i16, src2: i16 },
    Divide { dest: u16, src1: i16, src2: i16 },
    Modulo { dest: u16, src1: i16, src2: i16 },

    // Comparison (result: 0 = false, 1 = true)
    Equal { dest: u16, src1: i16, src2: i16 },
    NotEqual { dest: u16, src1: i16, src2: i16 },
    Greater { dest: u16, src1: i16, src2: i16 },
    GreaterEqual { dest: u16, src1: i16, src2: i16 },
    Less { dest: u16, src1: i16, src2: i16 },
    LessEqual { dest: u16, src1: i16, src2: i16 },

    // Unary
    Negate { dest: u16, src: i16 }, // dst = -src
    Not { dest: u16, src: i16 },    // logical not

    // Data movement
    Move { dest: u16, src: i16 }, // copy value

    // Function calls
    Call { dest: u16, src: i16 }, // jump + push return addr
    Return { src: i16 },          // return with value (assume in register 0)
    ReturnVoid,                   // return without value

    // Control flow
    Jump { offset: i16 },
    JumpIfTrue { src: i16, offset: i16 },
    JumpIfFalse { src: i16, offset: i16 },

    // I/O
    Print { src: i16 },

    // Program control
    Halt,
}
