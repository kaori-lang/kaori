use std::fmt;

#[derive(Debug)]
#[repr(u8, align(2))]
pub enum Instruction {
    Add { dest: u8, src1: u8, src2: u8 },
    Subtract { dest: u8, src1: u8, src2: u8 },
    Multiply { dest: u8, src1: u8, src2: u8 },
    Divide { dest: u8, src1: u8, src2: u8 },
    Modulo { dest: u8, src1: u8, src2: u8 },
    Power { dest: u8, src1: u8, src2: u8 },
    Equal { dest: u8, src1: u8, src2: u8 },
    NotEqual { dest: u8, src1: u8, src2: u8 },
    Greater { dest: u8, src1: u8, src2: u8 },
    GreaterEqual { dest: u8, src1: u8, src2: u8 },
    Less { dest: u8, src1: u8, src2: u8 },
    LessEqual { dest: u8, src1: u8, src2: u8 },
    Negate { dest: u8, src: u8 },
    Not { dest: u8, src: u8 },
    Move { dest: u8, src: u8 },
    LoadConst { dest: u8, src: u8 },
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
        (unsafe { *(self as *const Self as *const u8) }) as usize
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add { dest, src1, src2 } => {
                write!(f, "ADD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Subtract { dest, src1, src2 } => {
                write!(f, "SUB r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Multiply { dest, src1, src2 } => {
                write!(f, "MUL r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Divide { dest, src1, src2 } => {
                write!(f, "DIV r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Modulo { dest, src1, src2 } => {
                write!(f, "MOD r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Power { dest, src1, src2 } => {
                write!(f, "POW r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Equal { dest, src1, src2 } => {
                write!(f, "EQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::NotEqual { dest, src1, src2 } => {
                write!(f, "NEQ r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Greater { dest, src1, src2 } => {
                write!(f, "GT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::GreaterEqual { dest, src1, src2 } => {
                write!(f, "GTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Less { dest, src1, src2 } => {
                write!(f, "LT r{} r{} r{}", dest, src1, src2)
            }
            Instruction::LessEqual { dest, src1, src2 } => {
                write!(f, "LTE r{} r{} r{}", dest, src1, src2)
            }
            Instruction::Negate { dest, src } => {
                write!(f, "NEG r{} r{}", dest, src)
            }
            Instruction::Not { dest, src } => {
                write!(f, "NOT r{} r{}", dest, src)
            }
            Instruction::Move { dest, src } => {
                write!(f, "MOV r{} r{}", dest, src)
            }
            Instruction::LoadConst { dest, src } => {
                write!(f, "LOADK r{} k{}", dest, src)
            }
            Instruction::CreateDict { dest } => {
                write!(f, "CREATE_DICT r{}", dest)
            }
            Instruction::SetField { object, key, value } => {
                write!(f, "SET_FIELD r{} r{} r{}", object, key, value)
            }
            Instruction::GetField { dest, object, key } => {
                write!(f, "GET_FIELD r{} r{} r{}", dest, object, key)
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
