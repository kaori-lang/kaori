use std::fmt;

#[derive(Debug)]
#[repr(u8, align(2))]
pub enum Instruction {
    AddRR { dest: u8, src1: u8, src2: u8 },
    AddRK { dest: u8, src1: u8, src2: u8 },
    AddKR { dest: u8, src1: u8, src2: u8 },
    SubtractRR { dest: u8, src1: u8, src2: u8 },
    SubtractRK { dest: u8, src1: u8, src2: u8 },
    SubtractKR { dest: u8, src1: u8, src2: u8 },
    MultiplyRR { dest: u8, src1: u8, src2: u8 },
    MultiplyRK { dest: u8, src1: u8, src2: u8 },
    MultiplyKR { dest: u8, src1: u8, src2: u8 },
    DivideRR { dest: u8, src1: u8, src2: u8 },
    DivideRK { dest: u8, src1: u8, src2: u8 },
    DivideKR { dest: u8, src1: u8, src2: u8 },
    ModuloRR { dest: u8, src1: u8, src2: u8 },
    ModuloRK { dest: u8, src1: u8, src2: u8 },
    ModuloKR { dest: u8, src1: u8, src2: u8 },
    PowerRR { dest: u8, src1: u8, src2: u8 },
    PowerRK { dest: u8, src1: u8, src2: u8 },
    PowerKR { dest: u8, src1: u8, src2: u8 },
    EqualRR { dest: u8, src1: u8, src2: u8 },
    EqualRK { dest: u8, src1: u8, src2: u8 },
    EqualKR { dest: u8, src1: u8, src2: u8 },
    NotEqualRR { dest: u8, src1: u8, src2: u8 },
    NotEqualRK { dest: u8, src1: u8, src2: u8 },
    NotEqualKR { dest: u8, src1: u8, src2: u8 },
    GreaterRR { dest: u8, src1: u8, src2: u8 },
    GreaterRK { dest: u8, src1: u8, src2: u8 },
    GreaterKR { dest: u8, src1: u8, src2: u8 },
    GreaterEqualRR { dest: u8, src1: u8, src2: u8 },
    GreaterEqualRK { dest: u8, src1: u8, src2: u8 },
    GreaterEqualKR { dest: u8, src1: u8, src2: u8 },

    NotK { dest: u8, src: u8 },
    NotR { dest: u8, src: u8 },
    NegateK { dest: u8, src: u8 },
    NegateR { dest: u8, src: u8 },
    // ===== Data Movement =====
    MoveR { dest: u8, src: u8 },
    MoveK { dest: u8, src: u8 },

    // ===== Objects =====
    CreateDict { dest: u8 },

    // object[key] = value
    SetFieldRR { object: u8, key: u8, value: u8 },
    SetFieldRK { object: u8, key: u8, value: u8 },
    SetFieldKR { object: u8, key: u8, value: u8 },
    SetFieldKK { object: u8, key: u8, value: u8 },

    // dest = object[key]
    GetFieldR { dest: u8, object: u8, key: u8 },
    GetFieldK { dest: u8, object: u8, key: u8 },

    // ===== Calls =====
    CallK { dest: u8, src: u8 },
    CallR { dest: u8, src: u8 },

    // ===== Control Flow =====
    ReturnK { src: u8 },
    ReturnR { src: u8 },

    Jump { offset: i16 },
    JumpIfTrueK { src: u8, offset: i16 },
    JumpIfTrueR { src: u8, offset: i16 },

    JumpIfFalseK { src: u8, offset: i16 },
    JumpIfFalseR { src: u8, offset: i16 },

    // ===== Misc =====
    PrintK { src: u8 },
    PrintR { src: u8 },
}
impl Instruction {
    pub const fn discriminant(&self) -> usize {
        (unsafe { *(self as *const Self as *const u8) }) as usize
    }
}
