use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_constants::CfgConstant,
    cfg_instruction::CfgInstruction,
    graph_traversal::reversed_postorder,
};

use super::{bytecode::Bytecode, instruction::Instruction, value::Value};

pub fn emit_bytecode(
    cfgs: Vec<BlockId>,
    basic_blocks: Vec<BasicBlock>,
    constants: Vec<CfgConstant>,
) -> Bytecode {
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
    bb_start_index: HashMap<BlockId, usize>,
}

impl CodegenContext {
    fn visit_cfg(&mut self, cfg: BlockId) {
        let blocks = reversed_postorder(cfg, &self.basic_blocks);
        let mut pending_backpatch = Vec::new();

        for (index, bb_id) in blocks.iter().copied().enumerate() {
            self.bb_start_index.insert(bb_id, self.instructions.len());

            let next_bb_id = blocks.get(index + 1).copied();

            self.visit_block(bb_id, next_bb_id, &mut pending_backpatch);
        }

        self.resolve_backpatches(&pending_backpatch);
    }

    fn visit_block(
        &mut self,
        bb_id: BlockId,
        next_bb_id: Option<BlockId>,
        pending_backpatch: &mut Vec<(usize, BlockId)>,
    ) {
        let basic_block = &self.basic_blocks[bb_id.0];

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
                if Some(r#true) != next_bb_id {
                    let instruction = Instruction::jump_if_true(src, 0);
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#true));

                    self.instructions.push(instruction);
                }

                if Some(r#false) != next_bb_id {
                    let instruction = Instruction::jump_if_false(src, 0);
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#false));

                    self.instructions.push(instruction);
                }
            }

            Terminator::Goto(target) => {
                if Some(target) != next_bb_id {
                    let instruction = Instruction::jump(0);
                    let index = self.instructions.len();
                    pending_backpatch.push((index, target));

                    self.instructions.push(instruction);
                }
            }

            Terminator::Return { src } => {
                let instruction = Instruction::return_(src);

                self.instructions.push(instruction);
            }

            Terminator::None => {}
        };
    }

    fn resolve_backpatches(&mut self, pending_backpatch: &[(usize, BlockId)]) {
        for (instruction_index, target_bb_id) in pending_backpatch.iter().copied() {
            let instruction = &mut self.instructions[instruction_index];
            let target_bb_index = *self.bb_start_index.get(&target_bb_id).unwrap();

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

fn convert_constants(
    cfg_constants: &[CfgConstant],
    instructions: &[Instruction],
    bb_start_index: &HashMap<BlockId, usize>,
) -> Vec<Value> {
    let mut constants = cfg_constants
        .iter()
        .map(|constant| match constant {
            CfgConstant::Boolean(v) => Value::boolean(*v),
            CfgConstant::Number(v) => Value::number(**v),
            CfgConstant::Function(block_id) => {
                let idx = *bb_start_index
                    .get(block_id)
                    .expect("Missing block ID for function constant");
                let ptr = unsafe { instructions.as_ptr().add(idx) };
                Value::instruction(ptr)
            }
            _ => todo!("Unhandled constant kind"),
        })
        .collect::<Vec<Value>>();

    constants.reverse();

    constants.push(Value::default());

    constants
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
        CfgInstruction::Call {
            dest,
            src,
            caller_size,
        } => Instruction::call(dest, src, caller_size),
        CfgInstruction::Print { src } => Instruction::print(src),
    }
}
