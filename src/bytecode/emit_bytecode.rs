use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, Terminator},
    cfg_constants::CfgConstant,
    cfg_function::CfgFunction,
    cfg_instruction::CfgInstruction,
    graph_traversal::reversed_postorder,
};

use super::{bytecode::Bytecode, instruction::Instruction, value::Value};

pub fn emit_bytecode(cfgs: Vec<CfgFunction>) -> Bytecode {
    let mut context = CodegenContext {
        basic_blocks,
        instructions: Vec::new(),
        bb_start_index: HashMap::new(),
    };

    for cfg in cfgs.iter().copied() {
        context.visit_cfg(cfg);
    }

    context.instructions.push(Instruction::Halt);

    let constants = convert_constants(&constants, &context.instructions, &context.bb_start_index);

    Bytecode::new(context.instructions, constants)
}

struct CodegenContext {
    basic_blocks: Vec<BasicBlock>,
    instructions: Vec<Instruction>,
    bb_start_index: HashMap<usize, usize>,
}

impl CodegenContext {
    fn visit_cfg(&mut self) {
        let basic_blocks = reversed_postorder(&self.basic_blocks);
        let mut pending_backpatch = Vec::new();

        for (index, bb_index) in basic_blocks.iter().copied().enumerate() {
            self.bb_start_index
                .insert(bb_index, self.instructions.len());

            let next_bb_index = basic_blocks.get(index + 1).copied();

            self.visit_block(index, next_bb_index, &mut pending_backpatch);
        }

        self.resolve_backpatches(&pending_backpatch);
    }

    fn visit_block(
        &mut self,
        index: usize,
        next_bb_index: Option<usize>,
        pending_backpatch: &mut Vec<(usize, usize)>,
    ) {
        let basic_block = &self.basic_blocks[index];

        for instruction in &basic_block.instructions {
            let instruction = visit_instruction(instruction);
            self.instructions.push(instruction);
        }

        match basic_block.terminator {
            Terminator::Branch {
                src,
                r#true,
                r#false,
            } => {
                if Some(r#true) != next_bb_index {
                    let instruction = Instruction::jump_if_true(src, 0);
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#true));

                    self.instructions.push(instruction);
                }

                if Some(r#false) != next_bb_index {
                    let instruction = Instruction::jump_if_false(src, 0);
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#false));

                    self.instructions.push(instruction);
                }
            }
            Terminator::Goto(target) => {
                if Some(target) != next_bb_index {
                    let instruction = Instruction::jump(0);
                    let index = self.instructions.len();
                    pending_backpatch.push((index, target));

                    self.instructions.push(instruction);
                }
            }
            Terminator::Return { src } => {
                let instruction = match src {
                    Some(src) => Instruction::return_(src),
                    _ => Instruction::return_void(),
                };

                self.instructions.push(instruction);
            }
            Terminator::None => {}
        };
    }

    fn resolve_backpatches(&mut self, pending_backpatch: &[(usize, BlockId)]) {
        for (instruction_index, target_index) in pending_backpatch.iter().copied() {
            let instruction = &mut self.instructions[instruction_index];
            let target_bb_index = *self.bb_start_index.get(&target_index).unwrap();

            let new_offset = target_bb_index as i16 - instruction_index as i16;

            match instruction {
                Instruction::Jump { offset } => {
                    *offset = new_offset;
                }
                Instruction::JumpIfTrue { offset, .. } => {
                    *offset = new_offset;
                }
                Instruction::JumpIfFalse { offset, .. } => {
                    *offset = new_offset;
                }
                _ => {}
            };
        }
    }
}

fn visit_instruction(instruction: &CfgInstruction) -> Instruction {
    match *instruction {
        CfgInstruction::Add { dest, src1, src2 } => Instruction::add(dest, src1, src2),
        CfgInstruction::Subtract { dest, src1, src2 } => Instruction::subtract(dest, src1, src2),
        CfgInstruction::Multiply { dest, src1, src2 } => Instruction::multiply(dest, src1, src2),
        CfgInstruction::Divide { dest, src1, src2 } => Instruction::divide(dest, src1, src2),
        CfgInstruction::Modulo { dest, src1, src2 } => Instruction::modulo(dest, src1, src2),
        CfgInstruction::Equal { dest, src1, src2 } => Instruction::equal(dest, src1, src2),
        CfgInstruction::NotEqual { dest, src1, src2 } => Instruction::not_equal(dest, src1, src2),
        CfgInstruction::Greater { dest, src1, src2 } => Instruction::greater(dest, src1, src2),
        CfgInstruction::GreaterEqual { dest, src1, src2 } => {
            Instruction::greater_equal(dest, src1, src2)
        }
        CfgInstruction::Less { dest, src1, src2 } => Instruction::less(dest, src1, src2),
        CfgInstruction::LessEqual { dest, src1, src2 } => Instruction::less_equal(dest, src1, src2),
        CfgInstruction::Negate { dest, src } => Instruction::negate(dest, src),
        CfgInstruction::Not { dest, src } => Instruction::not(dest, src),
        CfgInstruction::Move { dest, src } => Instruction::move_(dest, src),
        CfgInstruction::Call { dest, src } => Instruction::call(dest, src),
        CfgInstruction::Print { src } => Instruction::print(src),
    }
}

fn map_cfg_constants(cfg_constants: &[CfgConstant], functions_start_index: &[usize]) -> Vec<Value> {
    cfg_constants
        .iter()
        .map(|constant| match constant {
            CfgConstant::Boolean(v) => Value::boolean(*v),
            CfgConstant::Number(v) => Value::number(**v),
            CfgConstant::Function(index) => {
                let index = functions_start_index[*index];

                Value::function(index)
            }
            _ => todo!(),
        })
        .collect()
}
