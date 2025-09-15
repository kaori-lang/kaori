#[derive(Default)]
pub struct BasicBlock {
    pub instructions: Vec<CfgInstruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn add_instruction(&mut self, instruction: CfgInstruction) {
        self.instructions.push(instruction);
    }
}

pub enum Terminator {
    Conditional {
        then_branch: usize,
        else_branch: Option<usize>,
    },
    Jump(usize),
    None,
}

impl Default for Terminator {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum CfgInstruction {
    Add { dst: u8, r1: u8, r2: u8 },
    Subtract { dst: u8, r1: u8, r2: u8 },
    Multiply { dst: u8, r1: u8, r2: u8 },
    Divide { dst: u8, r1: u8, r2: u8 },
    Modulo { dst: u8, r1: u8, r2: u8 },
    Equal { dst: u8, r1: u8, r2: u8 },
    NotEqual { dst: u8, r1: u8, r2: u8 },
    Greater { dst: u8, r1: u8, r2: u8 },
    GreaterEqual { dst: u8, r1: u8, r2: u8 },
    Less { dst: u8, r1: u8, r2: u8 },
    LessEqual { dst: u8, r1: u8, r2: u8 },

    StringConst { dst: u8, value: String },
    NumberConst { dst: u8, value: f64 },
    BooleanConst { dst: u8, value: bool },

    LoadLocal { dst: u8, r1: u8 },
    StoreLocal { dst: u8, r1: u8 },

    Call,
    Return,
    Pop,
    Print,
}
