use std::collections::HashMap;

use crate::cfg_ir::graph_traversal::Postorder;

use super::{
    cfg_instruction::{CfgInstruction, CfgInstructionId, CfgInstructionKind},
    cfg_ir::CfgIr,
    register_allocator::RegisterAllocator,
};

pub struct LivenessAnalysis<'a> {
    cfg_ir: &'a CfgIr,
    register_lifetime: HashMap<usize, CfgInstructionId>,
    traversal: Postorder<'a>,
    register_allocator: RegisterAllocator,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfg_ir: &'a CfgIr) -> Self {
        let traversal = Postorder::new(&cfg_ir.basic_blocks);

        Self {
            cfg_ir,
            register_lifetime: HashMap::new(),
            traversal,
            register_allocator: RegisterAllocator::new(),
        }
    }

    pub fn analyze_cfgs(&mut self) {
        for cfg in &self.cfg_ir.cfgs {
            for block_id in self.traversal.reversed_postorder(cfg) {
                let bb = self.cfg_ir.basic_blocks.get(&block_id).unwrap();

                self.analyze_instructions(&bb.instructions);
            }

            println!("\n");

            self.register_lifetime.clear();
        }
    }

    fn try_to_free(&mut self, register: usize, instruction: CfgInstructionId) {
        let register_last_instruction = *self.register_lifetime.get(&register).unwrap();

        if instruction == register_last_instruction {
            self.register_allocator.free_register(register);
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
