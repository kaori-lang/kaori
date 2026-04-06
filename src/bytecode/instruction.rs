use std::fmt;

#[derive(Debug)]
#[repr(u8, align(2))]
pub enum Instruction {
    Add { dest: u16, src1: i16, src2: i16 },
    Subtract { dest: u16, src1: i16, src2: i16 },
    Multiply { dest: u16, src1: i16, src2: i16 },
    Divide { dest: u16, src1: i16, src2: i16 },
    Modulo { dest: u16, src1: i16, src2: i16 },
    Equal { dest: u16, src1: i16, src2: i16 },
    NotEqual { dest: u16, src1: i16, src2: i16 },
    Greater { dest: u16, src1: i16, src2: i16 },
    GreaterEqual { dest: u16, src1: i16, src2: i16 },
    Less { dest: u16, src1: i16, src2: i16 },
    LessEqual { dest: u16, src1: i16, src2: i16 },
    Negate { dest: u16, src: i16 },
    Not { dest: u16, src: i16 },
    Move { dest: u16, src: i16 },
    CreateDict { dest: u16 },
    SetField { object: u16, key: i16, value: i16 },
    GetField { dest: u16, object: i16, key: i16 },
    Call { dest: u16, src: i16 },
    Return { src: i16 },
    ReturnVoid,
    Jump { offset: i16 },
    JumpIfTrue { src: i16, offset: i16 },
    JumpIfFalse { src: i16, offset: i16 },
    Print { src: i16 },
    Halt,
}

impl Instruction {
    pub const fn discriminant(&self) -> usize {
        (unsafe { *(self as *const Self as *const u8) }) as usize
    }
}

fn fmt_operand(v: i16) -> String {
    if v < 0 {
        format!("k{}", -(v + 1)) // constant
    } else {
        format!("r{}", v) // register
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add { dest, src1, src2 } => {
                write!(
                    f,
                    "ADD r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::Subtract { dest, src1, src2 } => {
                write!(
                    f,
                    "SUB r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::Multiply { dest, src1, src2 } => {
                write!(
                    f,
                    "MUL r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::Divide { dest, src1, src2 } => {
                write!(
                    f,
                    "DIV r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::Modulo { dest, src1, src2 } => {
                write!(
                    f,
                    "MOD r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }

            Instruction::Equal { dest, src1, src2 } => {
                write!(
                    f,
                    "EQ r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::NotEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "NEQ r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::Greater { dest, src1, src2 } => {
                write!(
                    f,
                    "GT r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::GreaterEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "GTE r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::Less { dest, src1, src2 } => {
                write!(
                    f,
                    "LT r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }
            Instruction::LessEqual { dest, src1, src2 } => {
                write!(
                    f,
                    "LTE r{} {} {}",
                    dest,
                    fmt_operand(*src1),
                    fmt_operand(*src2)
                )
            }

            Instruction::Negate { dest, src } => {
                write!(f, "NEG r{} {}", dest, fmt_operand(*src))
            }
            Instruction::Not { dest, src } => {
                write!(f, "NOT r{} {}", dest, fmt_operand(*src))
            }

            Instruction::Move { dest, src } => {
                write!(f, "MOV r{} {}", dest, fmt_operand(*src))
            }

            Instruction::CreateDict { dest } => {
                write!(f, "CREATE_DICT r{}", dest)
            }
            Instruction::SetField { object, key, value } => {
                write!(
                    f,
                    "SET_FIELD r{} {} {}",
                    object,
                    fmt_operand(*key),
                    fmt_operand(*value)
                )
            }
            Instruction::GetField { dest, object, key } => {
                write!(
                    f,
                    "GET_FIELD r{} {} {}",
                    dest,
                    fmt_operand(*object),
                    fmt_operand(*key)
                )
            }

            Instruction::Call { dest, src } => {
                write!(f, "CALL r{} {}", dest, fmt_operand(*src))
            }

            Instruction::Return { src } => {
                write!(f, "RET {}", fmt_operand(*src))
            }
            Instruction::ReturnVoid => {
                write!(f, "RET")
            }

            Instruction::Jump { offset } => {
                write!(f, "JMP {}", offset)
            }
            Instruction::JumpIfTrue { src, offset } => {
                write!(f, "JMP_IF_TRUE {} {}", fmt_operand(*src), offset)
            }
            Instruction::JumpIfFalse { src, offset } => {
                write!(f, "JMP_IF_FALSE {} {}", fmt_operand(*src), offset)
            }

            Instruction::Print { src } => {
                write!(f, "PRINT {}", fmt_operand(*src))
            }

            Instruction::Halt => {
                write!(f, "HALT")
            }
        }
    }
}
