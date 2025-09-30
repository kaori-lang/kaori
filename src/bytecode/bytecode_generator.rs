#![allow(clippy::new_without_default)]

use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    graph_traversal::reversed_postorder,
};

use super::{bytecode::Bytecode, instruction::Instruction, value::Value};

type InstructionIndex = usize;
pub struct BytecodeGenerator {
    pub cfg_instructions: Vec<CfgInstruction>,
    pub bytecode: Bytecode,
    pub basic_blocks: HashMap<BlockId, InstructionIndex>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            bytecode: Bytecode::default(),
            instruction_index: 0,
            basic_blocks: HashMap::new(),
        }
    }

    pub fn generate(&mut self, cfg_ir: &CfgIr) {
        self.update_bb_instruction_index(cfg_ir);

        for cfg in &cfg_ir.cfgs {
            self.visit_cfg(*cfg, &cfg_ir.basic_blocks);
        }
    }

    fn calculate_offset(&self, id: BlockId) -> i16 {
        let block_index = *self.basic_blocks.get(&id).unwrap() as i16;

        block_index - self.instruction_index as i16
    }

    fn emit_instruction(&mut self, instruction: Instruction) {
        self.bytecode.instructions.push(instruction);

        self.instruction_index += 1;
    }

    fn visit_cfg(&mut self, cfg: BlockId, basic_blocks: &[BasicBlock]) {
        let blocks = reversed_postorder(cfg, basic_blocks);

        for id in blocks {
            let basic_block = &basic_blocks[id.0];
            self.visit_block(basic_block);
        }
    }

    fn visit_block(&mut self, basic_block: &BasicBlock) {
        self.basic_blocks
            .insert(basic_block.id, self.instruction_index);

        for instruction in &basic_block.instructions {
            let instruction = self.visit_instruction(instruction);

            self.emit_instruction(instruction);
        }

        match basic_block.terminator {
            Terminator::Branch {
                src,
                r#true,
                r#false,
            } => {
                let offset = self.calculate_offset(r#false);

                let instruction = Instruction::jump_false(src, offset);

                self.emit_instruction(instruction);
            }
            Terminator::Goto(target) => {
                let offset = self.calculate_offset(target);

                let instruction = Instruction::jump(offset);

                self.emit_instruction(instruction);
            }
            Terminator::Return { src } => {
                let instruction = Instruction::return_(src.unwrap());

                self.emit_instruction(instruction);
            }
            _ => {}
        }
    }

    fn visit_instruction(&mut self, instruction: &CfgInstruction) -> Instruction {
        match *instruction {
            CfgInstruction::Add { dest, src1, src2 } => Instruction::add(dest, src1, src2),
            CfgInstruction::Subtract { dest, src1, src2 } => {
                Instruction::subtract(dest, src1, src2)
            }
            CfgInstruction::Multiply { dest, src1, src2 } => {
                Instruction::multiply(dest, src1, src2)
            }
            CfgInstruction::Divide { dest, src1, src2 } => Instruction::divide(dest, src1, src2),
            CfgInstruction::Modulo { dest, src1, src2 } => Instruction::modulo(dest, src1, src2),
            CfgInstruction::Equal { dest, src1, src2 } => Instruction::equal(dest, src1, src2),
            CfgInstruction::NotEqual { dest, src1, src2 } => {
                Instruction::not_equal(dest, src1, src2)
            }
            CfgInstruction::Greater { dest, src1, src2 } => Instruction::greater(dest, src1, src2),
            CfgInstruction::GreaterEqual { dest, src1, src2 } => {
                Instruction::greater_equal(dest, src1, src2)
            }
            CfgInstruction::Less { dest, src1, src2 } => Instruction::less(dest, src1, src2),
            CfgInstruction::LessEqual { dest, src1, src2 } => {
                Instruction::less_equal(dest, src1, src2)
            }
            CfgInstruction::And { dest, src1, src2 } => Instruction::and(dest, src1, src2),
            CfgInstruction::Or { dest, src1, src2 } => Instruction::or(dest, src1, src2),

            CfgInstruction::Negate { dest, src } => Instruction::negate(dest, src),
            CfgInstruction::Not { dest, src } => Instruction::not(dest, src),
            CfgInstruction::Move { dest, src } => Instruction::mov(dest, src),

            /*   CfgInstruction::StringConst { dest, value } => {
                let value = Value::boolean(false);

                let constant_index = self.bytecode.constant_pool.insert_value(value);

                Instruction::const_(dest, constant_index)
            } */
            CfgInstruction::NumberConst { dest, value } => {
                let value = Value::number(value);

                let constant_index = self.bytecode.constant_pool.insert_value(value);

                Instruction::const_(dest, constant_index)
            }
            CfgInstruction::BooleanConst { dest, value } => {
                let value = Value::boolean(value);

                let constant_index = self.bytecode.constant_pool.insert_value(value);

                Instruction::const_(dest, constant_index)
            }
            CfgInstruction::FunctionConst { dest, value } => {
                let instruction_index = *self.basic_blocks.get(&value).unwrap();

                let value = Value::instruction_index(instruction_index);

                let constant_index = self.bytecode.constant_pool.insert_value(value);

                Instruction::const_(dest, constant_index)
            }

            CfgInstruction::Call => Instruction::call(),
            CfgInstruction::Print { src } => Instruction::print(src),
            _ => unreachable!(),
        }
    }
}
