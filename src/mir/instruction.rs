use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(pub i16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstIndex(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
    Const(ConstIndex),
}

impl From<Register> for Operand {
    fn from(r: Register) -> Self {
        Operand::Register(r)
    }
}

impl From<ConstIndex> for Operand {
    fn from(k: ConstIndex) -> Self {
        Operand::Const(k)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Arith {
        op: ArithOp,
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Cmp {
        op: CmpOp,
        dest: Register,
        src1: Operand,
        src2: Operand,
    },
    Not {
        dest: Register,
        src: Register,
    },
    Negate {
        dest: Register,
        src: Register,
    },
    Move {
        dest: Register,
        src: Register,
    },
    LoadConst {
        dest: Register,
        src: ConstIndex,
    },
    CreateDict {
        dest: Register,
    },
    SetField {
        object: Register,
        key: Register,
        value: Register,
    },
    GetField {
        dest: Register,
        object: Register,
        key: Register,
    },
    CreateClosure {
        dest: Register,
        src: u32,
    },
    CaptureValue {
        dest: Register,
        src: Register,
    },
    CreateCell {
        dest: Register,
        src: Register,
    },
    SetCell {
        dest: Register,
        src: Register,
    },
    GetCell {
        dest: Register,
        src: Register,
    },
    Call {
        dest: Register,
        src: Register,
        arity: u8,
    },
    Return {
        src: Register,
    },
    Jump {
        offset: i32,
    },
    JumpIfFalse {
        src: Register,
        offset: i32,
    },
    JumpIfTrue {
        src: Register,
        offset: i32,
    },
    JumpIf {
        op: CmpOp,
        not: bool,
        src1: Operand,
        src2: Operand,
        offset: i32,
    },
    Nop,
}

impl Instruction {
    pub fn add(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Arith {
            op: ArithOp::Add,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn sub(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Arith {
            op: ArithOp::Subtract,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn mul(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Arith {
            op: ArithOp::Multiply,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn div(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Arith {
            op: ArithOp::Divide,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn mod_(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Arith {
            op: ArithOp::Modulo,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn eq(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Cmp {
            op: CmpOp::Equal,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn neq(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Cmp {
            op: CmpOp::NotEqual,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn lt(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Cmp {
            op: CmpOp::Less,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn lte(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Cmp {
            op: CmpOp::LessEqual,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn gt(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Cmp {
            op: CmpOp::Greater,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }
    pub fn gte(dest: Register, src1: impl Into<Operand>, src2: impl Into<Operand>) -> Self {
        Self::Cmp {
            op: CmpOp::GreaterEqual,
            dest,
            src1: src1.into(),
            src2: src2.into(),
        }
    }

    pub fn not(dest: Register, src: Register) -> Self {
        Self::Not { dest, src }
    }
    pub fn neg(dest: Register, src: Register) -> Self {
        Self::Negate { dest, src }
    }

    pub fn mov(dest: Register, src: Register) -> Self {
        Self::Move { dest, src }
    }
    pub fn load_const(dest: Register, src: ConstIndex) -> Self {
        Self::LoadConst { dest, src }
    }

    pub fn create_dict(dest: Register) -> Self {
        Self::CreateDict { dest }
    }
    pub fn set_field(object: Register, key: Register, value: Register) -> Self {
        Self::SetField { object, key, value }
    }
    pub fn get_field(dest: Register, object: Register, key: Register) -> Self {
        Self::GetField { dest, object, key }
    }

    pub fn create_closure(dest: Register, src: u32) -> Self {
        Self::CreateClosure { dest, src }
    }
    pub fn capture_value(dest: Register, src: Register) -> Self {
        Self::CaptureValue { dest, src }
    }
    pub fn create_cell(dest: Register, src: Register) -> Self {
        Self::CreateCell { dest, src }
    }
    pub fn set_cell(dest: Register, src: Register) -> Self {
        Self::SetCell { dest, src }
    }
    pub fn get_cell(dest: Register, src: Register) -> Self {
        Self::GetCell { dest, src }
    }

    pub fn call(dest: Register, src: Register, arity: u8) -> Self {
        Self::Call { dest, src, arity }
    }
    pub fn ret(src: Register) -> Self {
        Self::Return { src }
    }
    pub fn jump(offset: i32) -> Self {
        Self::Jump { offset }
    }

    pub fn jump_if_true(src: Register, offset: i32) -> Self {
        Self::JumpIfTrue { src, offset }
    }
    pub fn jump_if_false(src: Register, offset: i32) -> Self {
        Self::JumpIfFalse { src, offset }
    }

    pub fn jump_if_eq(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::Equal,
            not: false,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_not_eq(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::Equal,
            not: true,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_lt(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::Less,
            not: false,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_not_lt(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::Less,
            not: true,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_lte(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::LessEqual,
            not: false,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_not_lte(
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
        offset: i32,
    ) -> Self {
        Self::JumpIf {
            op: CmpOp::LessEqual,
            not: true,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_gt(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::Greater,
            not: false,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_not_gt(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::Greater,
            not: true,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_gte(src1: impl Into<Operand>, src2: impl Into<Operand>, offset: i32) -> Self {
        Self::JumpIf {
            op: CmpOp::GreaterEqual,
            not: false,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }
    pub fn jump_if_not_gte(
        src1: impl Into<Operand>,
        src2: impl Into<Operand>,
        offset: i32,
    ) -> Self {
        Self::JumpIf {
            op: CmpOp::GreaterEqual,
            not: true,
            src1: src1.into(),
            src2: src2.into(),
            offset,
        }
    }

    pub fn nop() -> Self {
        Self::Nop
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            write!(f, "arg({})", -(self.0 + 1))
        } else {
            write!(f, "R{}", self.0)
        }
    }
}

impl fmt::Display for ConstIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Const(c) => write!(f, "{}", c),
        }
    }
}

impl ArithOp {
    pub fn mnemonic(self) -> &'static str {
        match self {
            ArithOp::Add => "ADD",
            ArithOp::Subtract => "SUB",
            ArithOp::Multiply => "MUL",
            ArithOp::Divide => "DIV",
            ArithOp::Modulo => "MOD",
        }
    }
}

impl CmpOp {
    pub fn mnemonic(self) -> &'static str {
        match self {
            CmpOp::Equal => "EQ",
            CmpOp::NotEqual => "NEQ",
            CmpOp::Less => "LT",
            CmpOp::LessEqual => "LTE",
            CmpOp::Greater => "GT",
            CmpOp::GreaterEqual => "GTE",
        }
    }

    pub fn jump_mnemonic(self) -> &'static str {
        match self {
            CmpOp::Equal => "JMP_IF_EQ",
            CmpOp::NotEqual => "JMP_IF_NEQ",
            CmpOp::Less => "JMP_IF_LT",
            CmpOp::LessEqual => "JMP_IF_LTE",
            CmpOp::Greater => "JMP_IF_GT",
            CmpOp::GreaterEqual => "JMP_IF_GTE",
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arith {
                op,
                dest,
                src1,
                src2,
            } => write!(f, "{} {} {} {}", op.mnemonic(), dest, src1, src2),
            Self::Cmp {
                op,
                dest,
                src1,
                src2,
            } => write!(f, "{} {} {} {}", op.mnemonic(), dest, src1, src2),
            Self::Not { dest, src } => write!(f, "NOT {} {}", dest, src),
            Self::Negate { dest, src } => write!(f, "NEG {} {}", dest, src),
            Self::Move { dest, src } => write!(f, "MOV {} {}", dest, src),
            Self::LoadConst { dest, src } => write!(f, "LOAD_CONST {} {}", dest, src),
            Self::CreateDict { dest } => write!(f, "DICT {}", dest),
            Self::SetField { object, key, value } => write!(f, "SET {} {} {}", object, key, value),
            Self::GetField { dest, object, key } => write!(f, "GET {} {} {}", dest, object, key),
            Self::CreateClosure { dest, src } => {
                write!(f, "CREATE_CLOSURE {} FUNCTIONS[{}]", dest, src)
            }
            Self::CaptureValue { dest, src } => write!(f, "CAPTURE_VALUE {} {}", dest, src),
            Self::CreateCell { dest, src } => write!(f, "CREATE_CELL {} {}", dest, src),
            Self::SetCell { dest, src } => write!(f, "SET_CELL {} {}", dest, src),
            Self::GetCell { dest, src } => write!(f, "GET_CELL {} {}", dest, src),
            Self::Call { dest, src, arity } => write!(f, "CALL {} {} ARITY({})", dest, src, arity),
            Self::Return { src } => write!(f, "RET {}", src),
            Self::Jump { offset } => write!(f, "JMP {}", offset),
            Self::JumpIfTrue { src, offset } => write!(f, "JMP_IF_TRUE {} {}", src, offset),
            Self::JumpIfFalse { src, offset } => write!(f, "JMP_IF_FALSE {} {}", src, offset),
            Self::JumpIf {
                op,
                not,
                src1,
                src2,
                offset,
            } => {
                if *not {
                    write!(
                        f,
                        "JMP_IF_NOT_{} {} {} {}",
                        op.mnemonic(),
                        src1,
                        src2,
                        offset
                    )
                } else {
                    write!(f, "JMP_IF_{} {} {} {}", op.mnemonic(), src1, src2, offset)
                }
            }
            Self::Nop => write!(f, "NOP"),
        }
    }
}
