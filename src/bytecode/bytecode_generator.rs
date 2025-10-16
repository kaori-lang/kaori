use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_constants::CfgConstant,
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    graph_traversal::reversed_postorder,
};

use super::{bytecode::Bytecode, instruction::Instruction, value::Value};

type InstructionIndex = usize;

#[derive(Default)]
pub struct BytecodeGenerator {
    pub cfg_instructions: Vec<CfgInstruction>,
    pub basic_blocks: HashMap<BlockId, InstructionIndex>,
}

impl BytecodeGenerator {
    pub fn convert_constants(
        &self,
        cfg_constants: &[CfgConstant],
        instructions: &[Instruction],
    ) -> Vec<Value> {
        let mut constants = Vec::new();

        constants.push(Value::default());

        for constant in cfg_constants {
            let constant = match constant {
                CfgConstant::Boolean(value) => Value::boolean(*value),
                CfgConstant::Function(value) => {
                    let instruction_index = *self.basic_blocks.get(value).unwrap();

                    let ptr = unsafe { instructions.as_ptr().add(instruction_index) };
                    Value::instruction(ptr)
                }
                CfgConstant::Number(value) => Value::number(**value),
                _ => todo!(),
            };

            constants.push(constant);
        }

        constants
    }
    pub fn generate(&mut self, cfg_ir: &CfgIr) -> Bytecode {
        self.flatten_cfg_ir(cfg_ir);

        let mut instructions = Vec::new();

        for index in 0..self.cfg_instructions.len() {
            let instruction = self.convert_instruction(index);
            instructions.push(instruction);
        }

        instructions.push(Instruction::Halt);

        let constants = self.convert_constants(&cfg_ir.constants.constants, &instructions);

        Bytecode::new(instructions, constants)
    }

    fn flatten_cfg_ir(&mut self, cfg_ir: &CfgIr) {
        for cfg in &cfg_ir.cfgs {
            self.visit_cfg(*cfg, &cfg_ir.basic_blocks);
        }
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
            .insert(basic_block.id, self.cfg_instructions.len());

        for instruction in &basic_block.instructions {
            self.cfg_instructions.push(instruction.to_owned());
        }

        match basic_block.terminator {
            Terminator::Branch {
                src,
                r#true,
                r#false,
            } => {
                let instruction = CfgInstruction::conditional_jump(src, r#true, r#false);

                self.cfg_instructions.push(instruction);
            }
            Terminator::Goto(target) => {
                let instruction = CfgInstruction::jump(target);

                self.cfg_instructions.push(instruction);
            }
            Terminator::Return { src } => {
                let instruction = CfgInstruction::return_(src);

                self.cfg_instructions.push(instruction);
            }
            _ => {}
        }
    }

    fn convert_instruction(&self, index: InstructionIndex) -> Instruction {
        match self.cfg_instructions[index] {
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
            CfgInstruction::Negate { dest, src } => Instruction::negate(dest, src),
            CfgInstruction::Not { dest, src } => Instruction::not(dest, src),
            CfgInstruction::Move { dest, src } => Instruction::mov(dest, src),
            CfgInstruction::Call {
                dest,
                src,
                caller_size,
            } => Instruction::call(dest, src, caller_size),
            CfgInstruction::Print { src } => Instruction::print(src),
            CfgInstruction::Jump { target } => {
                let offset = *self.basic_blocks.get(&target).unwrap() as i16 - index as i16;

                Instruction::jump(offset)
            }
            CfgInstruction::ConditionalJump {
                src,
                true_target,
                false_target,
            } => {
                let true_offset =
                    *self.basic_blocks.get(&true_target).unwrap() as i16 - index as i16;
                let false_offset =
                    *self.basic_blocks.get(&false_target).unwrap() as i16 - index as i16;

                Instruction::conditional_jump(src, true_offset, false_offset)
            }
            CfgInstruction::Return { src } => Instruction::return_(src),
        }
    }
}
