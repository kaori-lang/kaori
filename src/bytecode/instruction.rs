use std::fmt;

#[derive(Clone, Copy)]
pub enum Instruction {
    // Arithmetic
    Add { dest: u8, src1: u8, src2: u8 },
    AddI { dest: u8, src1: u8, src2: u32 },
    Subtract { dest: u8, src1: u8, src2: u8 },
    SubtractRI { dest: u8, src1: u8, src2: u32 }, // src1 - src2
    SubtractIR { dest: u8, src1: u32, src2: u8 }, // src1 - src2
    Multiply { dest: u8, src1: u8, src2: u8 },
    MultiplyI { dest: u8, src1: u8, src2: u32 },
    Divide { dest: u8, src1: u8, src2: u8 },
    DivideRI { dest: u8, src1: u8, src2: u32 }, // src1 / src2
    DivideIR { dest: u8, src1: u32, src2: u8 }, // src1 / src2
    Modulo { dest: u8, src1: u8, src2: u8 },
    ModuloRI { dest: u8, src1: u8, src2: u32 }, // src1 % src2
    ModuloIR { dest: u8, src1: u32, src2: u8 }, // src1 % src2

    // Comparison
    Equal { dest: u8, src1: u8, src2: u8 },
    EqualI { dest: u8, src1: u8, src2: u32 },
    NotEqual { dest: u8, src1: u8, src2: u8 },
    NotEqualI { dest: u8, src1: u8, src2: u32 },
    Less { dest: u8, src1: u8, src2: u8 },
    LessRI { dest: u8, src1: u8, src2: u32 },
    LessIR { dest: u8, src1: u32, src2: u8 },
    LessEqual { dest: u8, src1: u8, src2: u8 },
    LessEqualRI { dest: u8, src1: u8, src2: u32 },
    LessEqualIR { dest: u8, src1: u32, src2: u8 },
    Greater { dest: u8, src1: u8, src2: u8 },
    GreaterRI { dest: u8, src1: u8, src2: u32 },
    GreaterIR { dest: u8, src1: u32, src2: u8 },
    GreaterEqual { dest: u8, src1: u8, src2: u8 },
    GreaterEqualRI { dest: u8, src1: u8, src2: u32 },
    GreaterEqualIR { dest: u8, src1: u32, src2: u8 },

    // Unary
    Not { dest: u8, src: u8 },
    Negate { dest: u8, src: u8 },

    // Loads
    Move { dest: u8, src: u8 },
    LoadK { dest: u8, src: u8 },
    LoadImm { dest: u8, src: u32 },

    // Tables
    CreateDict { dest: u8 },
    SetField { object: u8, key: u8, value: u8 },
    GetField { dest: u8, object: u8, key: u8 },

    // Calls
    Call { dest: u8, src: u8 },
    Return { src: u8 },

    // Jumps
    Jump { offset: i32 },
    JumpIfTrue { src: u8, offset: i32 },
    JumpIfFalse { src: u8, offset: i32 },

    JumpIfLess { src1: u8, src2: u8, offset: i32 },
    JumpIfLessI { src1: u8, src2: u32, offset: i32 },
    JumpIfLessEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfLessEqualI { src1: u8, src2: u32, offset: i32 },
    JumpIfGreater { src1: u8, src2: u8, offset: i32 },
    JumpIfGreaterI { src1: u8, src2: u32, offset: i32 },
    JumpIfGreaterEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfGreaterEqualI { src1: u8, src2: u32, offset: i32 },
    JumpIfEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfEqualI { src1: u8, src2: u32, offset: i32 },
    JumpIfNotEqual { src1: u8, src2: u8, offset: i32 },
    JumpIfNotEqualI { src1: u8, src2: u32, offset: i32 },

    // Misc
    Print { src: u8 },
    EnterUncheckedBlock,
    ExitUncheckedBlock,
}
impl Instruction {
    pub const fn discriminant(&self) -> usize {
        (unsafe { *(self as *const Self as *const u8) }) as usize
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::EnterUncheckedBlock => write!(f, "ENTER_UNCHECKED_BLOCK"),
            Instruction::ExitUncheckedBlock => write!(f, "EXIT_UNCHECKED_BLOCK"),

            // Arithmetic (floats)
            Instruction::Add { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::AddI { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }

            Instruction::Subtract { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Instruction::SubtractRI { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::SubtractIR { dest, src1, src2 } => {
                write!(f, "SUB r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            Instruction::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Instruction::MultiplyI { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }

            Instruction::Divide { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Instruction::DivideRI { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::DivideIR { dest, src1, src2 } => {
                write!(f, "DIV r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            Instruction::Modulo { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::ModuloRI { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::ModuloIR { dest, src1, src2 } => {
                write!(f, "MOD r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            // Comparison (floats)
            Instruction::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::EqualI { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }

            Instruction::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::NotEqualI { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }

            Instruction::Less { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessRI { dest, src1, src2 } => {
                write!(f, "LT r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::LessIR { dest, src1, src2 } => {
                write!(f, "LT r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            Instruction::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessEqualRI { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::LessEqualIR { dest, src1, src2 } => {
                write!(f, "LTE r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            Instruction::Greater { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterRI { dest, src1, src2 } => {
                write!(f, "GT r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::GreaterIR { dest, src1, src2 } => {
                write!(f, "GT r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            Instruction::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterEqualRI { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} {}", dest, src1, f32::from_bits(*src2))
            }
            Instruction::GreaterEqualIR { dest, src1, src2 } => {
                write!(f, "GTE r{} {} r{}", dest, f32::from_bits(*src1), src2)
            }

            // Unary
            Instruction::Not { dest, src } => {
                write!(f, "NOT r{} r{}", dest, src)
            }
            Instruction::Negate { dest, src } => {
                write!(f, "NEG r{} r{}", dest, src)
            }

            // Loads
            Instruction::Move { dest, src } => {
                write!(f, "MOV r{} r{}", dest, src)
            }
            Instruction::LoadK { dest, src } => {
                write!(f, "LOADK r{} k{}", dest, src)
            }
            Instruction::LoadImm { dest, src } => {
                write!(f, "LOADF r{} {}", dest, f32::from_bits(*src))
            }

            // Tables
            Instruction::CreateDict { dest } => {
                write!(f, "DICT r{}", dest)
            }
            Instruction::SetField { object, key, value } => {
                write!(f, "SET r{} r{} r{}", object, key, value)
            }
            Instruction::GetField { dest, object, key } => {
                write!(f, "GET r{} r{} r{}", dest, object, key)
            }

            // Calls
            Instruction::Call { dest, src } => {
                write!(f, "CALL r{} r{}", dest, src)
            }
            Instruction::Return { src } => {
                write!(f, "RET r{}", src)
            }

            // Jumps (comparisons still float immediates)
            Instruction::Jump { offset } => write!(f, "JMP {}", offset),

            Instruction::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE r{} {}", src, offset)
            }
            Instruction::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{} {}", src, offset)
            }

            Instruction::JumpIfLess { src1, src2, offset } => {
                write!(f, "JMP_IF_LT r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfLessI { src1, src2, offset } => {
                write!(
                    f,
                    "JMP_IF_LT r{} {} {}",
                    src1,
                    f32::from_bits(*src2),
                    offset
                )
            }

            Instruction::JumpIfEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_EQ r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfEqualI { src1, src2, offset } => {
                write!(
                    f,
                    "JMP_IF_EQ r{} {} {}",
                    src1,
                    f32::from_bits(*src2),
                    offset
                )
            }

            Instruction::JumpIfNotEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_NEQ r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfNotEqualI { src1, src2, offset } => {
                write!(
                    f,
                    "JMP_IF_NEQ r{} {} {}",
                    src1,
                    f32::from_bits(*src2),
                    offset
                )
            }

            Instruction::JumpIfLessEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_LTE r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfLessEqualI { src1, src2, offset } => {
                write!(
                    f,
                    "JMP_IF_LTE r{} {} {}",
                    src1,
                    f32::from_bits(*src2),
                    offset
                )
            }

            Instruction::JumpIfGreater { src1, src2, offset } => {
                write!(f, "JMP_IF_GT r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfGreaterI { src1, src2, offset } => {
                write!(
                    f,
                    "JMP_IF_GT r{} {} {}",
                    src1,
                    f32::from_bits(*src2),
                    offset
                )
            }

            Instruction::JumpIfGreaterEqual { src1, src2, offset } => {
                write!(f, "JMP_IF_GTE r{} r{} {}", src1, src2, offset)
            }
            Instruction::JumpIfGreaterEqualI { src1, src2, offset } => {
                write!(
                    f,
                    "JMP_IF_GTE r{} {} {}",
                    src1,
                    f32::from_bits(*src2),
                    offset
                )
            }

            // Misc
            Instruction::Print { src } => {
                write!(f, "PRINT r{}", src)
            }
        }
    }
}
