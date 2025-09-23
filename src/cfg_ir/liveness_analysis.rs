use std::collections::HashMap;

use super::{
    basic_block::{BasicBlock, Terminator},
    block_id::BlockId,
    cfg_instruction::{CfgInstruction, CfgInstructionId, CfgInstructionKind},
    cfg_stream::CfgStream,
};

pub struct LivenessAnalysis<'a> {
    cfg_stream: &'a CfgStream,
    register_lifetime: HashMap<usize, CfgInstructionId>,
    pos_order: Vec<BlockId>,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfg_stream: &'a CfgStream) -> Self {
        Self {
            cfg_stream,
            register_lifetime: HashMap::new(),
            pos_order: Vec::new(),
        }
    }

    pub fn analyze_cfgs(&mut self) {}

    pub fn dfs_postorder(&mut self, bb: &BasicBlock) {
        match &bb.terminator {
            Terminator::Branch { r#true, r#false } => {
                let left_bb = self.cfg_stream.basic_blocks.get(r#true).unwrap();
                let right_bb = self.cfg_stream.basic_blocks.get(r#false).unwrap();

                self.dfs_postorder(right_bb);
                self.dfs_postorder(left_bb);
            }
            Terminator::Goto(target) => {
                let bb = self.cfg_stream.basic_blocks.get(target).unwrap();

                self.dfs_postorder(bb);
            }
            _ => {}
        };

        self.pos_order.push(bb.id);
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
