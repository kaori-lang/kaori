#![allow(clippy::new_without_default)]

use std::collections::HashMap;

use crate::{
    cfg_ir::{
        basic_block::{BasicBlock, BlockId, Terminator},
        cfg_instruction::CfgInstruction,
        cfg_ir::CfgIr,
        graph_traversal::reversed_postorder,
    },
    virtual_machine::value::Value,
};

use super::instruction::Instruction;

type InstructionIndex = usize;
pub struct BytecodeGenerator {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
    pub instruction_index: InstructionIndex,
    pub pending_backpatch: Vec<(InstructionIndex, BlockId)>,
    pub basic_blocks: HashMap<BlockId, InstructionIndex>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
            instruction_index: 0,
            pending_backpatch: Vec::new(),
            basic_blocks: HashMap::new(),
        }
    }

    pub fn generate(&mut self, cfg_ir: &CfgIr) {
        for cfg in &cfg_ir.cfgs {
            self.visit_cfg(*cfg, &cfg_ir.basic_blocks);
        }

        self.backpatch_instructions();

        for instruction in &self.instructions {
            println!("{instruction}");
        }
    }

    fn emit_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);

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
            Terminator::Branch { r#true, r#false } => {
                let backpatch = (self.instruction_index, r#false);
                self.pending_backpatch.push(backpatch);

                let instruction = Instruction::jump_false(0);

                self.emit_instruction(instruction);
            }
            Terminator::Goto(target) => {
                let backpatch = (self.instruction_index, target);
                self.pending_backpatch.push(backpatch);

                let instruction = Instruction::jump(0);

                self.emit_instruction(instruction);
            }
            Terminator::Return => {}
            _ => {}
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
            CfgInstruction::Print { src } => Instruction::print(*src),
        }
    }

    fn backpatch_instructions(&mut self) {
        for (instruction_index, block_id) in &self.pending_backpatch {
            let block_index = self.basic_blocks.get(block_id).unwrap();

            let instruction = match &self.instructions[*instruction_index] {
                Instruction::Jump(..) => {
                    let offset = *block_index as i16 - *instruction_index as i16;

                    Instruction::jump(offset)
                }
                Instruction::JumpFalse(..) => {
                    let offset = *block_index as i16 - *instruction_index as i16;

                    Instruction::jump_false(offset)
                }
                _ => unreachable!(),
            };

            self.instructions[*instruction_index] = instruction;
        }
    }
}
