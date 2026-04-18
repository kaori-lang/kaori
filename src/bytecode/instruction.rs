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
    EqualRR { dest: u8, src1: u8, src2: u8 },
    EqualRK { dest: u8, src1: u8, src2: u8 },
    EqualKR { dest: u8, src1: u8, src2: u8 },
    NotEqualRR { dest: u8, src1: u8, src2: u8 },
    NotEqualRK { dest: u8, src1: u8, src2: u8 },
    NotEqualKR { dest: u8, src1: u8, src2: u8 },
    LessRR { dest: u8, src1: u8, src2: u8 },
    LessRK { dest: u8, src1: u8, src2: u8 },
    LessKR { dest: u8, src1: u8, src2: u8 },
    LessEqualRR { dest: u8, src1: u8, src2: u8 },
    LessEqualRK { dest: u8, src1: u8, src2: u8 },
    LessEqualKR { dest: u8, src1: u8, src2: u8 },
    GreaterRR { dest: u8, src1: u8, src2: u8 },
    GreaterRK { dest: u8, src1: u8, src2: u8 },
    GreaterKR { dest: u8, src1: u8, src2: u8 },
    GreaterEqualRR { dest: u8, src1: u8, src2: u8 },
    GreaterEqualRK { dest: u8, src1: u8, src2: u8 },
    GreaterEqualKR { dest: u8, src1: u8, src2: u8 },
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
    CallK { dest: u8, src: u8 },
    CallR { dest: u8, src: u8 },
    ReturnK { src: u8 },
    ReturnR { src: u8 },
    Jump { offset: i16 },
    JumpIfTrue { src: u8, offset: i16 },
    JumpIfFalse { src: u8, offset: i16 },
    Print { src: u8 },
}

impl Instruction {
    pub const fn discriminant(&self) -> usize {
        (unsafe { *(self as *const Self as *const u8) }) as usize
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::AddRR { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::AddRK { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} k{}", dest, src1, src2)
            }
            Instruction::AddKR { dest, src1, src2 } => {
                write!(f, "ADD r{} k{} r{}", dest, src1, src2)
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

            Instruction::MultiplyRR { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Instruction::MultiplyRK { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} k{}", dest, src1, src2)
            }
            Instruction::MultiplyKR { dest, src1, src2 } => {
                write!(f, "MUL r{} k{} r{}", dest, src1, src2)
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

            Instruction::EqualRR { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::EqualRK { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} k{}", dest, src1, src2)
            }
            Instruction::EqualKR { dest, src1, src2 } => {
                write!(f, "EQ r{} k{} r{}", dest, src1, src2)
            }

            Instruction::NotEqualRR { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::NotEqualRK { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} k{}", dest, src1, src2)
            }
            Instruction::NotEqualKR { dest, src1, src2 } => {
                write!(f, "NEQ r{} k{} r{}", dest, src1, src2)
            }

            Instruction::LessRR { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessRK { dest, src1, src2 } => {
                write!(f, "LT r{} r{} k{}", dest, src1, src2)
            }
            Instruction::LessKR { dest, src1, src2 } => {
                write!(f, "LT r{} k{} r{}", dest, src1, src2)
            }

            Instruction::LessEqualRR { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessEqualRK { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} k{}", dest, src1, src2)
            }
            Instruction::LessEqualKR { dest, src1, src2 } => {
                write!(f, "LTE r{} k{} r{}", dest, src1, src2)
            }

            Instruction::GreaterRR { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterRK { dest, src1, src2 } => {
                write!(f, "GT r{} r{} k{}", dest, src1, src2)
            }
            Instruction::GreaterKR { dest, src1, src2 } => {
                write!(f, "GT r{} k{} r{}", dest, src1, src2)
            }

            Instruction::GreaterEqualRR { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterEqualRK { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} k{}", dest, src1, src2)
            }
            Instruction::GreaterEqualKR { dest, src1, src2 } => {
                write!(f, "GTE r{} k{} r{}", dest, src1, src2)
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

            Instruction::CallR { dest, src } => {
                write!(f, "CALL r{} r{}", dest, src)
            }
            Instruction::CallK { dest, src } => {
                write!(f, "CALL r{} k{}", dest, src)
            }

            Instruction::ReturnR { src } => {
                write!(f, "RET r{}", src)
            }
            Instruction::ReturnK { src } => {
                write!(f, "RET k{}", src)
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
