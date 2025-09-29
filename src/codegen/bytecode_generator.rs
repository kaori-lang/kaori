#![allow(clippy::new_without_default)]

use crate::{
    cfg_ir::{
        basic_block::BlockId, cfg_instruction::CfgInstruction, cfg_ir::CfgIr,
        graph_traversal::reversed_postorder,
    },
    virtual_machine::value::Value,
};

use super::instruction::Instruction;

pub struct BytecodeGenerator {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
    pub cfg_ir: CfgIr,

    pub current_instruction: usize,
}

impl BytecodeGenerator {
    pub fn new(cfg_ir: CfgIr) -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
            cfg_ir,
            current_instruction: 0,
        }
    }

    pub fn generate(&mut self) {
        for i in 0..self.cfg_ir.cfgs.len() {
            let cfg = self.cfg_ir.cfgs[i];

            self.visit_cfg(cfg);
        }
    }

    fn emit_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);

        self.current_instruction += 1;
    }

    fn visit_cfg(&mut self, cfg: BlockId) {
        let blocks = reversed_postorder(&cfg, &self.cfg_ir.basic_blocks);

        for block_id in blocks {
            self.visit_block(block_id);
        }
    }

    fn visit_block(&mut self, id: BlockId) {
        let instructions = &self.cfg_ir.basic_blocks.get(&id).unwrap().instructions;

        for instruction in instructions {
            let instruction = self.visit_instruction(instruction);
        }
    }

    fn visit_instruction(&mut self, instruction: &CfgInstruction) -> Instruction {
        match instruction {
            CfgInstruction::Add { dest, src1, src2 } => Instruction::add(*dest, *src1, *src2),
            CfgInstruction::Subtract { dest, src1, src2 } => {
                Instruction::subtract(*dest, *src1, *src2)
            }
            CfgInstruction::Multiply { dest, src1, src2 } => {
                Instruction::multiply(*dest, *src1, *src2)
            }
            CfgInstruction::Divide { dest, src1, src2 } => Instruction::divide(*dest, *src1, *src2),
            CfgInstruction::Modulo { dest, src1, src2 } => Instruction::modulo(*dest, *src1, *src2),
            CfgInstruction::Equal { dest, src1, src2 } => Instruction::equal(*dest, *src1, *src2),
            CfgInstruction::NotEqual { dest, src1, src2 } => {
                Instruction::not_equal(*dest, *src1, *src2)
            }
            CfgInstruction::Greater { dest, src1, src2 } => {
                Instruction::greater(*dest, *src1, *src2)
            }
            CfgInstruction::GreaterEqual { dest, src1, src2 } => {
                Instruction::greater_equal(*dest, *src1, *src2)
            }
            CfgInstruction::Less { dest, src1, src2 } => Instruction::less(*dest, *src1, *src2),
            CfgInstruction::LessEqual { dest, src1, src2 } => {
                Instruction::less_equal(*dest, *src1, *src2)
            }
            CfgInstruction::And { dest, src1, src2 } => Instruction::and(*dest, *src1, *src2),
            CfgInstruction::Or { dest, src1, src2 } => Instruction::or(*dest, *src1, *src2),

            CfgInstruction::Negate { dest, src } => Instruction::negate(*dest, *src),
            CfgInstruction::Not { dest, src } => Instruction::not(*dest, *src),
            CfgInstruction::Move { dest, src } => Instruction::mov(*dest, *src),

            CfgInstruction::StringConst { dest, value } => Instruction::load_const(*dest, 0),
            CfgInstruction::NumberConst { dest, value } => Instruction::load_const(*dest, 0),
            CfgInstruction::BooleanConst { dest, value } => Instruction::load_const(*dest, 0),
            CfgInstruction::FunctionConst { dest, value } => Instruction::load_const(*dest, 0),

            CfgInstruction::Call => Instruction::call(),
            CfgInstruction::Return { src } => Instruction::return_(*src),
            CfgInstruction::Print => Instruction::print(),
        }
    }
}
