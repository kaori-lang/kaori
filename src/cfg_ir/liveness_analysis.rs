use std::collections::{HashMap, HashSet};

use crate::cfg_ir::traversal::Postorder;

use super::{
    basic_block::{BasicBlock, Terminator},
    block_id::BlockId,
    cfg_instruction::{CfgInstruction, CfgInstructionId, CfgInstructionKind},
    cfg_stream::CfgStream,
};

pub struct LivenessAnalysis<'a> {
    cfg_stream: &'a CfgStream,
    register_lifetime: HashMap<usize, CfgInstructionId>,
    post_order: Vec<BlockId>,
    visited: HashSet<BlockId>,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfg_stream: &'a CfgStream) -> Self {
        Self {
            cfg_stream,
            register_lifetime: HashMap::new(),
            post_order: Vec::new(),
            visited: HashSet::new(),
        }
    }

    pub fn analyze_cfgs(&mut self) {
        for root in &self.cfg_stream.roots {
            let traversal = Postorder::new(&self.cfg_stream.basic_blocks);

            self.analyze_instructions(&bb.instructions);

            println!("\n");

            self.register_lifetime.clear();
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

            println!(" {instruction}");
        }
    }
}
