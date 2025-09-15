use super::cfg_instruction::CfgInstruction;

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
