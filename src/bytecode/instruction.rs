use std::fmt;

#[derive(Debug)]
#[repr(u8, align(2))]
pub enum Instruction {
    Add { dest: u8, src1: u8, src2: u8 },
    AddK { dest: u8, src1: u8, src2: u8 },
    SubtractRR { dest: u8, src1: u8, src2: u8 },
    SubtractRK { dest: u8, src1: u8, src2: u8 },
    SubtractKR { dest: u8, src1: u8, src2: u8 },
    Multiply { dest: u8, src1: u8, src2: u8 },
    MultiplyK { dest: u8, src1: u8, src2: u8 },
    DivideRR { dest: u8, src1: u8, src2: u8 },
    DivideRK { dest: u8, src1: u8, src2: u8 },
    DivideKR { dest: u8, src1: u8, src2: u8 },
    ModuloRR { dest: u8, src1: u8, src2: u8 },
    ModuloRK { dest: u8, src1: u8, src2: u8 },
    ModuloKR { dest: u8, src1: u8, src2: u8 },
    Equal { dest: u8, src1: u8, src2: u8 },
    EqualK { dest: u8, src1: u8, src2: u8 },
    NotEqual { dest: u8, src1: u8, src2: u8 },
    NotEqualK { dest: u8, src1: u8, src2: u8 },
    Less { dest: u8, src1: u8, src2: u8 },
    LessK { dest: u8, src1: u8, src2: u8 },
    LessEqual { dest: u8, src1: u8, src2: u8 },
    LessEqualK { dest: u8, src1: u8, src2: u8 },
    Greater { dest: u8, src1: u8, src2: u8 },
    GreaterK { dest: u8, src1: u8, src2: u8 },
    GreaterEqual { dest: u8, src1: u8, src2: u8 },
    GreaterEqualK { dest: u8, src1: u8, src2: u8 },
    Not { dest: u8, src: u8 },
    Negate { dest: u8, src: u8 },
    MoveR { dest: u8, src: u8 },
    MoveK { dest: u8, src: u8 },
    CreateDict { dest: u8 },
    SetFieldRR { object: u8, key: u8, value: u8 },
    SetFieldRK { object: u8, key: u8, value: u8 },
    SetFieldKR { object: u8, key: u8, value: u8 },
    SetFieldKK { object: u8, key: u8, value: u8 },
    GetFieldR { dest: u8, object: u8, key: u8 },
    GetFieldK { dest: u8, object: u8, key: u8 },
    Call { dest: u8, src: u8 },
    Return { src: u8 },
    Jump { offset: i16 },
    JumpIfTrue { src: u8, offset: i16 },
    JumpIfFalse { src: u8, offset: i16 },
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
            Instruction::Add { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::AddK { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} k{}", dest, src1, src2)
            }
            Instruction::SubtractRR { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Instruction::SubtractRK { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} k{}", dest, src1, src2)
            }
            Instruction::SubtractKR { dest, src1, src2 } => {
                write!(f, "SUB r{} k{} r{}", dest, src1, src2)
            }
            Instruction::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Instruction::MultiplyK { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} k{}", dest, src1, src2)
            }
            Instruction::DivideRR { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Instruction::DivideRK { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} k{}", dest, src1, src2)
            }
            Instruction::DivideKR { dest, src1, src2 } => {
                write!(f, "DIV r{} k{} r{}", dest, src1, src2)
            }
            Instruction::ModuloRR { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::ModuloRK { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} k{}", dest, src1, src2)
            }
            Instruction::ModuloKR { dest, src1, src2 } => {
                write!(f, "MOD r{} k{} r{}", dest, src1, src2)
            }
            Instruction::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::EqualK { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} k{}", dest, src1, src2)
            }
            Instruction::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::NotEqualK { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} k{}", dest, src1, src2)
            }
            Instruction::Less { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessK { dest, src1, src2 } => {
                write!(f, "LT r{} r{} k{}", dest, src1, src2)
            }
            Instruction::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessEqualK { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} k{}", dest, src1, src2)
            }
            Instruction::Greater { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterK { dest, src1, src2 } => {
                write!(f, "GT r{} r{} k{}", dest, src1, src2)
            }
            Instruction::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterEqualK { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} k{}", dest, src1, src2)
            }
            Instruction::Not { dest, src } => {
                write!(f, "NOT r{} r{}", dest, src)
            }
            Instruction::Negate { dest, src } => {
                write!(f, "NEG r{} r{}", dest, src)
            }
            Instruction::MoveR { dest, src } => {
                write!(f, "MOV r{} r{}", dest, src)
            }
            Instruction::MoveK { dest, src } => {
                write!(f, "MOV r{} k{}", dest, src)
            }
            Instruction::CreateDict { dest } => {
                write!(f, "DICT r{}", dest)
            }
            Instruction::SetFieldRR { object, key, value } => {
                write!(f, "SET r{} r{} r{}", object, key, value)
            }
            Instruction::SetFieldRK { object, key, value } => {
                write!(f, "SET r{} r{} k{}", object, key, value)
            }
            Instruction::SetFieldKR { object, key, value } => {
                write!(f, "SET r{} k{} r{}", object, key, value)
            }
            Instruction::SetFieldKK { object, key, value } => {
                write!(f, "SET r{} k{} k{}", object, key, value)
            }
            Instruction::GetFieldR { dest, object, key } => {
                write!(f, "GET r{} r{} r{}", dest, object, key)
            }
            Instruction::GetFieldK { dest, object, key } => {
                write!(f, "GET r{} r{} k{}", dest, object, key)
            }

            Instruction::Call { dest, src } => {
                write!(f, "CALL r{} r{}", dest, src)
            }
            Instruction::Return { src } => {
                write!(f, "RET r{}", src)
            }
            Instruction::Jump { offset } => {
                write!(f, "JMP {}", offset)
            }
            Instruction::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE r{} {}", src, offset)
            }
            Instruction::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{} {}", src, offset)
            }
            Instruction::Print { src } => {
                write!(f, "PRINT r{}", src)
            }
        }
    }
}
