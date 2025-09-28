use std::collections::HashMap;

use crate::cfg_ir::graph_traversal::Postorder;

use super::{
    block_id::BlockId,
    cfg_instruction::{CfgInstruction, CfgInstructionId, CfgInstructionKind},
    cfg_ir::CfgIr,
    operand::{Operand, Variable},
    register_allocator::RegisterAllocator,
};

pub struct LivenessAnalysis<'a> {
    cfg_ir: &'a mut CfgIr,
    variable_lifetime: HashMap<Variable, CfgInstructionId>,
    register_allocator: RegisterAllocator,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfg_ir: &'a mut CfgIr) -> Self {
        Self {
            cfg_ir,
            variable_lifetime: HashMap::new(),
            register_allocator: RegisterAllocator::new(),
        }
    }

    pub fn analyze_cfgs(&mut self) {
        /* for cfg in self.cfg_ir.cfgs.as_ref() {
            self.analyze_cfg(cfg);
        } */
    }

    pub fn analyze_cfg(&mut self, cfg: BlockId) {
        println!("\n");

        self.variable_lifetime.clear();
    }

    fn try_to_free(&mut self, register: usize, instruction: CfgInstructionId) {
        /*   let register_last_instruction = *self.variable_lifetime.get(&register).unwrap();

        if instruction == register_last_instruction {
            self.register_allocator.free_register(register);
        } */
    }

    fn update_variable_lifetime(&mut self, operand: Operand, instruction_id: CfgInstructionId) {
        if let Operand::Variable(variable) = operand {
            self.variable_lifetime.insert(variable, instruction_id);
        }
    }

    fn analyze_instructions(&mut self, instructions: &[CfgInstruction]) {
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
                    self.update_variable_lifetime(*dest, instruction.id);
                    self.update_variable_lifetime(*src1, instruction.id);
                    self.update_variable_lifetime(*src2, instruction.id);
                }
                CfgInstructionKind::Negate { dest, src }
                | CfgInstructionKind::Not { dest, src }
                | CfgInstructionKind::Move { dest, src } => {
                    self.update_variable_lifetime(*dest, instruction.id);
                    self.update_variable_lifetime(*src, instruction.id);
                }
                CfgInstructionKind::StringConst { dest, .. }
                | CfgInstructionKind::NumberConst { dest, .. }
                | CfgInstructionKind::BooleanConst { dest, .. }
                | CfgInstructionKind::FunctionConst { dest, .. } => {
                    self.update_variable_lifetime(*dest, instruction.id);
                }
                CfgInstructionKind::Call => {}
                CfgInstructionKind::Return { .. } => {}
                CfgInstructionKind::Print => {}
            }

            //println!(" {instruction}");
        }
    }

    fn allocate_register(&mut self, instruction: &mut CfgInstruction) {
        match &mut instruction.kind {
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
                self.update_variable_lifetime(*dest, instruction.id);
                self.update_variable_lifetime(*src1, instruction.id);
                self.update_variable_lifetime(*src2, instruction.id);
            }
            CfgInstructionKind::Negate { dest, src }
            | CfgInstructionKind::Not { dest, src }
            | CfgInstructionKind::Move { dest, src } => {
                self.update_variable_lifetime(*dest, instruction.id);
                self.update_variable_lifetime(*src, instruction.id);
            }
            CfgInstructionKind::StringConst { dest, .. }
            | CfgInstructionKind::NumberConst { dest, .. }
            | CfgInstructionKind::BooleanConst { dest, .. }
            | CfgInstructionKind::FunctionConst { dest, .. } => {
                self.update_variable_lifetime(*dest, instruction.id);
            }
            CfgInstructionKind::Call => {}
            CfgInstructionKind::Return { .. } => {}
            CfgInstructionKind::Print => {}
        }

        //println!(" {instruction}");
    }
}
