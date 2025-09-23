use std::collections::HashMap;

use super::{
    basic_block::{BasicBlock, Terminator},
    cfg::Cfg,
    cfg_instruction::{CfgInstruction, CfgInstructionId, CfgInstructionKind},
};

pub struct LivenessAnalysis<'a> {
    cfgs: &'a [Cfg],
    register_lifetime: HashMap<usize, CfgInstructionId>,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfgs: &'a [Cfg]) -> Self {
        Self {
            cfgs,
            register_lifetime: HashMap::new(),
        }
    }

    pub fn analyze_cfgs(&self) {}

    pub fn analyze_basic_block(&mut self, basic_blocks: &[BasicBlock], bb: &BasicBlock) {
        self.analyze_instructions(&bb.instructions);

        match &bb.terminator {
            Terminator::Branch { r#true, r#false } => {
                let left_bb = &basic_blocks[*r#true];
                let right_bb = &basic_blocks[*r#false];

                self.analyze_basic_block(basic_blocks, left_bb);
                self.analyze_basic_block(basic_blocks, right_bb);
            }
            Terminator::Goto(target) => {
                let bb = &basic_blocks[*target];

                self.analyze_basic_block(basic_blocks, bb);
            }
            _ => {}
        }
    }

    pub fn analyze_instructions(&mut self, instructions: &[CfgInstruction]) {
        for instruction in instructions {
            match &instruction.kind {
                CfgInstructionKind::Add { dest, src1, src2 }
                | CfgInstructionKind::Subtract { dest, src1, src2 }
                | CfgInstructionKind::Multiply { dest, src1, src2 }
                | CfgInstructionKind::Divide { dest, src1, src2 }
                | CfgInstructionKind::Modulo { dest, src1, src2 }
                | CfgInstructionKind::Equal { dest, src1, src2 }
                | CfgInstructionKind::NotEqual { dest, src1, src2 }
                | CfgInstructionKind::Greater { dest, src1, src2 }
                | CfgInstructionKind::GreaterEqual { dest, src1, src2 }
                | CfgInstructionKind::Less { dest, src1, src2 }
                | CfgInstructionKind::LessEqual { dest, src1, src2 }
                | CfgInstructionKind::And { dest, src1, src2 }
                | CfgInstructionKind::Or { dest, src1, src2 } => {
                    self.register_lifetime.insert(*dest, instruction.id);
                    self.register_lifetime.insert(*src1, instruction.id);
                    self.register_lifetime.insert(*src2, instruction.id);
                }
                CfgInstructionKind::Negate { dest, src }
                | CfgInstructionKind::Not { dest, src }
                | CfgInstructionKind::Move { dest, src } => {
                    self.register_lifetime.insert(*dest, instruction.id);
                    self.register_lifetime.insert(*src, instruction.id);
                }
                CfgInstructionKind::StringConst { dest, .. }
                | CfgInstructionKind::NumberConst { dest, .. }
                | CfgInstructionKind::BooleanConst { dest, .. }
                | CfgInstructionKind::FunctionConst { dest, .. } => {
                    self.register_lifetime.insert(*dest, instruction.id);
                }
                CfgInstructionKind::Call => {}
                CfgInstructionKind::Return { .. } => {}
                CfgInstructionKind::Print => {}
            }
        }
    }
}
