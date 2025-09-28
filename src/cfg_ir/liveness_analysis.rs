use std::collections::HashMap;

use crate::cfg_ir::graph_traversal::Postorder;

use super::{
    block_id::BlockId,
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    operand::{Operand, Variable},
    register_allocator::RegisterAllocator,
};

type Instruction = usize;

pub struct LivenessAnalysis<'a> {
    cfg_ir: &'a mut CfgIr,
    variable_lifetime: HashMap<Variable, Instruction>,
    register_allocator: RegisterAllocator,
    current_instruction: Instruction,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfg_ir: &'a mut CfgIr) -> Self {
        Self {
            cfg_ir,
            variable_lifetime: HashMap::new(),
            register_allocator: RegisterAllocator::new(),
            current_instruction: 0,
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

    fn try_to_free(&mut self, register: usize) {
        /*   let register_last_instruction = *self.variable_lifetime.get(&register).unwrap();

        if instruction == register_last_instruction {
            self.register_allocator.free_register(register);
        } */
    }

    fn update_variable_lifetime(&mut self, operand: Operand) {
        if let Operand::Variable(variable) = operand {
            self.variable_lifetime
                .insert(variable, self.current_instruction);
        }
    }

    fn analyze_instructions(&mut self, instructions: &[CfgInstruction]) {
        for instruction in instructions {
            self.analyze_instruction(instruction);

            self.current_instruction += 1;
        }
    }

    fn analyze_instruction(&mut self, instruction: &CfgInstruction) {
        match instruction {
            CfgInstruction::Add { dest, src1, src2 }
            | CfgInstruction::Subtract { dest, src1, src2 }
            | CfgInstruction::Multiply { dest, src1, src2 }
            | CfgInstruction::Divide { dest, src1, src2 }
            | CfgInstruction::Modulo { dest, src1, src2 }
            | CfgInstruction::Equal { dest, src1, src2 }
            | CfgInstruction::NotEqual { dest, src1, src2 }
            | CfgInstruction::Greater { dest, src1, src2 }
            | CfgInstruction::GreaterEqual { dest, src1, src2 }
            | CfgInstruction::Less { dest, src1, src2 }
            | CfgInstruction::LessEqual { dest, src1, src2 }
            | CfgInstruction::And { dest, src1, src2 }
            | CfgInstruction::Or { dest, src1, src2 } => {
                self.update_variable_lifetime(*dest);
                self.update_variable_lifetime(*src1);
                self.update_variable_lifetime(*src2);
            }
            CfgInstruction::Negate { dest, src }
            | CfgInstruction::Not { dest, src }
            | CfgInstruction::Move { dest, src } => {
                self.update_variable_lifetime(*dest);
                self.update_variable_lifetime(*src);
            }
            CfgInstruction::StringConst { dest, .. }
            | CfgInstruction::NumberConst { dest, .. }
            | CfgInstruction::BooleanConst { dest, .. }
            | CfgInstruction::FunctionConst { dest, .. } => {
                self.update_variable_lifetime(*dest);
            }
            CfgInstruction::Call => {}
            CfgInstruction::Return { .. } => {}
            CfgInstruction::Print => {}
        }
    }

    fn allocate_register(&mut self, instruction: &mut CfgInstruction) {
        match instruction {
            CfgInstruction::Add { dest, src1, src2 }
            | CfgInstruction::Subtract { dest, src1, src2 }
            | CfgInstruction::Multiply { dest, src1, src2 }
            | CfgInstruction::Divide { dest, src1, src2 }
            | CfgInstruction::Modulo { dest, src1, src2 }
            | CfgInstruction::Equal { dest, src1, src2 }
            | CfgInstruction::NotEqual { dest, src1, src2 }
            | CfgInstruction::Greater { dest, src1, src2 }
            | CfgInstruction::GreaterEqual { dest, src1, src2 }
            | CfgInstruction::Less { dest, src1, src2 }
            | CfgInstruction::LessEqual { dest, src1, src2 }
            | CfgInstruction::And { dest, src1, src2 }
            | CfgInstruction::Or { dest, src1, src2 } => {
                self.update_variable_lifetime(*dest);
                self.update_variable_lifetime(*src1);
                self.update_variable_lifetime(*src2);
            }
            CfgInstruction::Negate { dest, src }
            | CfgInstruction::Not { dest, src }
            | CfgInstruction::Move { dest, src } => {
                self.update_variable_lifetime(*dest);
                self.update_variable_lifetime(*src);
            }
            CfgInstruction::StringConst { dest, .. }
            | CfgInstruction::NumberConst { dest, .. }
            | CfgInstruction::BooleanConst { dest, .. }
            | CfgInstruction::FunctionConst { dest, .. } => {
                self.update_variable_lifetime(*dest);
            }
            CfgInstruction::Call => {}
            CfgInstruction::Return { .. } => {}
            CfgInstruction::Print => {}
        }

        //println!(" {instruction}");
    }
}
