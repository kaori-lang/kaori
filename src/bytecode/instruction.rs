use std::fmt;

#[derive(Debug)]
#[repr(u8, align(2))]
pub enum Instruction {
    AddRR { dest: u8, src1: u8, src2: u8 },
    AddRI { dest: u8, src1: u8, imm: u16 },
    AddIR { dest: u8, imm: u16, src2: u8 },

    SubtractRR { dest: u8, src1: u8, src2: u8 },
    SubtractRI { dest: u8, src1: u8, imm: u16 },
    SubtractIR { dest: u8, imm: u16, src2: u8 },

    MultiplyRR { dest: u8, src1: u8, src2: u8 },
    MultiplyRI { dest: u8, src1: u8, imm: u16 },
    MultiplyIR { dest: u8, imm: u16, src2: u8 },

    DivideRR { dest: u8, src1: u8, src2: u8 },
    DivideRI { dest: u8, src1: u8, imm: u16 },
    DivideIR { dest: u8, imm: u16, src2: u8 },

    ModuloRR { dest: u8, src1: u8, src2: u8 },
    ModuloRI { dest: u8, src1: u8, imm: u16 },
    ModuloIR { dest: u8, imm: u16, src2: u8 },

    PowerRR { dest: u8, src1: u8, src2: u8 },
    PowerRI { dest: u8, src1: u8, imm: u16 },
    PowerIR { dest: u8, imm: u16, src2: u8 },

    EqualRR { dest: u8, src1: u8, src2: u8 },
    EqualRI { dest: u8, src1: u8, imm: u16 },
    EqualIR { dest: u8, imm: u16, src2: u8 },

    NotEqualRR { dest: u8, src1: u8, src2: u8 },
    NotEqualRI { dest: u8, src1: u8, imm: u16 },
    NotEqualIR { dest: u8, imm: u16, src2: u8 },

    LessRR { dest: u8, src1: u8, src2: u8 },
    LessRI { dest: u8, src1: u8, imm: u16 },
    LessIR { dest: u8, imm: u16, src2: u8 },

    LessEqualRR { dest: u8, src1: u8, src2: u8 },
    LessEqualRI { dest: u8, src1: u8, imm: u16 },
    LessEqualIR { dest: u8, imm: u16, src2: u8 },

    GreaterRR { dest: u8, src1: u8, src2: u8 },
    GreaterRI { dest: u8, src1: u8, imm: u16 },
    GreaterIR { dest: u8, imm: u16, src2: u8 },

    GreaterEqualRR { dest: u8, src1: u8, src2: u8 },
    GreaterEqualRI { dest: u8, src1: u8, imm: u16 },
    GreaterEqualIR { dest: u8, imm: u16, src2: u8 },

    Not { dest: u8, src: u8 },
    Negate { dest: u8, src: u8 },

    LoadConst { dest: u8, src: u8 },
    Move { dest: u8, src: u8 },

    LoadNumber { dest: u8, imm: u16 },
    LoadBool { dest: u8, imm: bool },
    LoadNil { dest: u8 },

    CreateDict { dest: u8 },
    SetField { object: u8, key: u8, value: u8 },
    GetField { dest: u8, object: u8, key: u8 },

    Call { dest: u8, src: u8 },
    Return { src: u8 },

    Jump { offset: i16 },
    JumpIfTrue { src: u8, offset: i16 },
    JumpIfFalse { src: u8, offset: i16 },

    Print { src: u8 },
}

impl Instruction {
    pub const fn discriminant(&self) -> usize {
        unsafe { *(self as *const Self as *const u8) as usize }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::AddRR { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::AddRI { dest, src1, imm } => write!(f, "ADD r{} r{} {}", dest, src1, imm),
            Instruction::AddIR { dest, imm, src2 } => write!(f, "ADD r{} {} r{}", dest, imm, src2),

            Instruction::SubtractRR { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Instruction::SubtractRI { dest, src1, imm } => {
                write!(f, "SUB r{} r{} {}", dest, src1, imm)
            }
            Instruction::SubtractIR { dest, imm, src2 } => {
                write!(f, "SUB r{} {} r{}", dest, imm, src2)
            }

            Instruction::MultiplyRR { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Instruction::MultiplyRI { dest, src1, imm } => {
                write!(f, "MUL r{} r{} {}", dest, src1, imm)
            }
            Instruction::MultiplyIR { dest, imm, src2 } => {
                write!(f, "MUL r{} {} r{}", dest, imm, src2)
            }

            Instruction::DivideRR { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Instruction::DivideRI { dest, src1, imm } => {
                write!(f, "DIV r{} r{} {}", dest, src1, imm)
            }
            Instruction::DivideIR { dest, imm, src2 } => {
                write!(f, "DIV r{} {} r{}", dest, imm, src2)
            }

            Instruction::ModuloRR { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::ModuloRI { dest, src1, imm } => {
                write!(f, "MOD r{} r{} {}", dest, src1, imm)
            }
            Instruction::ModuloIR { dest, imm, src2 } => {
                write!(f, "MOD r{} {} r{}", dest, imm, src2)
            }

            Instruction::PowerRR { dest, src1, src2 } => {
                write!(f, "POW r{} r{} r{}", dest, src1, src2)
            }
            Instruction::PowerRI { dest, src1, imm } => {
                write!(f, "POW r{} r{} {}", dest, src1, imm)
            }
            Instruction::PowerIR { dest, imm, src2 } => {
                write!(f, "POW r{} {} r{}", dest, imm, src2)
            }

            Instruction::EqualRR { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::EqualRI { dest, src1, imm } => write!(f, "EQ r{} r{} {}", dest, src1, imm),
            Instruction::EqualIR { dest, imm, src2 } => write!(f, "EQ r{} {} r{}", dest, imm, src2),

            Instruction::NotEqualRR { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::NotEqualRI { dest, src1, imm } => {
                write!(f, "NEQ r{} r{} {}", dest, src1, imm)
            }
            Instruction::NotEqualIR { dest, imm, src2 } => {
                write!(f, "NEQ r{} {} r{}", dest, imm, src2)
            }

            Instruction::LessRR { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessRI { dest, src1, imm } => write!(f, "LT r{} r{} {}", dest, src1, imm),
            Instruction::LessIR { dest, imm, src2 } => write!(f, "LT r{} {} r{}", dest, imm, src2),

            Instruction::LessEqualRR { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessEqualRI { dest, src1, imm } => {
                write!(f, "LTE r{} r{} {}", dest, src1, imm)
            }
            Instruction::LessEqualIR { dest, imm, src2 } => {
                write!(f, "LTE r{} {} r{}", dest, imm, src2)
            }

            Instruction::GreaterRR { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterRI { dest, src1, imm } => {
                write!(f, "GT r{} r{} {}", dest, src1, imm)
            }
            Instruction::GreaterIR { dest, imm, src2 } => {
                write!(f, "GT r{} {} r{}", dest, imm, src2)
            }

            Instruction::GreaterEqualRR { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterEqualRI { dest, src1, imm } => {
                write!(f, "GTE r{} r{} {}", dest, src1, imm)
            }
            Instruction::GreaterEqualIR { dest, imm, src2 } => {
                write!(f, "GTE r{} {} r{}", dest, imm, src2)
            }

            Instruction::Not { dest, src } => write!(f, "NOT r{} r{}", dest, src),
            Instruction::Negate { dest, src } => write!(f, "NEG r{} r{}", dest, src),

            Instruction::LoadConst { dest, src } => write!(f, "LOADK r{} k{}", dest, src),
            Instruction::Move { dest, src } => write!(f, "MOV r{} r{}", dest, src),

            Instruction::LoadNumber { dest, imm } => write!(f, "LOADN r{} {}", dest, imm),
            Instruction::LoadBool { dest, imm } => write!(f, "LOADB r{} {}", dest, imm),
            Instruction::LoadNil { dest } => write!(f, "LOADNIL r{}", dest),

            Instruction::CreateDict { dest } => write!(f, "DICT r{}", dest),
            Instruction::SetField { object, key, value } => {
                write!(f, "SET r{} r{} r{}", object, key, value)
            }
            Instruction::GetField { dest, object, key } => {
                write!(f, "GET r{} r{} r{}", dest, object, key)
            }

            Instruction::Call { dest, src } => write!(f, "CALL r{} r{}", dest, src),
            Instruction::Return { src } => write!(f, "RET r{}", src),

            Instruction::Jump { offset } => write!(f, "JMP {}", offset),
            Instruction::JumpIfTrue { src, offset } => write!(f, "JMP_IF_TRUE r{} {}", src, offset),
            Instruction::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE r{} {}", src, offset)
            }

            Instruction::Print { src } => write!(f, "PRINT r{}", src),
        }
    }
}
