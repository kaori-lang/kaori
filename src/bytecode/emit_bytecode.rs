use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, Terminator},
    cfg_constants::CfgConstant,
    cfg_function::CfgFunction,
    cfg_instruction::CfgInstruction,
    graph_traversal::reversed_postorder,
    operand::Operand,
};

use super::{bytecode::Bytecode, function::Function, instruction::Instruction, value::Value};

pub fn emit_bytecode(cfgs: Vec<CfgFunction>) -> Bytecode {
    let mut instructions = Vec::new();
    let mut functions_start_index = Vec::new();

    for cfg in &cfgs {
        let index = instructions.len();
        functions_start_index.push(index);

        let mut context = CodegenContext::new(
            &cfg.basic_blocks,
            cfg.allocated_variables,
            &mut instructions,
        );

        context.emit_instructions();
    }

    instructions.push(Instruction::Halt);

    let mut functions = Vec::new();

    for (index, cfg) in cfgs.iter().enumerate() {
        let constants = map_cfg_constants(&cfg.constants);
        let frame_size = cfg.allocated_variables;

        let ip = unsafe { instructions.as_ptr().add(functions_start_index[index]) };

        let function = Function::new(ip, frame_size as u8, constants);

        functions.push(function);
    }

    Bytecode::new(instructions, functions)
}

struct CodegenContext<'a> {
    basic_blocks: &'a [BasicBlock],
    frame_size: usize,
    instructions: &'a mut Vec<Instruction>,
}

impl<'a> CodegenContext<'a> {
    fn new(
        basic_blocks: &'a [BasicBlock],
        frame_size: usize,
        instructions: &'a mut Vec<Instruction>,
    ) -> Self {
        Self {
            basic_blocks,
            frame_size,
            instructions,
        }
    }

    fn emit_instructions(&mut self) {
        let basic_blocks = reversed_postorder(self.basic_blocks);

        let mut pending_backpatch = Vec::new();
        let mut bb_start_index = HashMap::new();

        for (index, bb_index) in basic_blocks.iter().copied().enumerate() {
            bb_start_index.insert(bb_index, self.instructions.len());

            let next_bb_index = basic_blocks.get(index + 1).copied();

            self.visit_block(bb_index, next_bb_index, &mut pending_backpatch);
        }

        resolve_backpatches(self.instructions, &pending_backpatch, &bb_start_index);
    }

    fn visit_block(
        &mut self,
        index: usize,
        next_bb_index: Option<usize>,
        pending_backpatch: &mut Vec<(usize, usize)>,
    ) {
        let basic_block = &self.basic_blocks[index];

        for instruction in &basic_block.instructions {
            let instruction = self.visit_instruction(instruction);
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

    fn visit_instruction(&self, instruction: &CfgInstruction) -> Instruction {
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
            CfgInstruction::Negate { dest, src } => Instruction::negate(dest, src),
            CfgInstruction::Not { dest, src } => Instruction::not(dest, src),
            CfgInstruction::Move { dest, src } => Instruction::move_(dest, src),
            CfgInstruction::MoveArg { dest, src } => {
                if let Operand::Variable(value) = dest {
                    let dest = Operand::Variable(self.frame_size + value);

                    Instruction::move_(dest, src)
                } else {
                    unreachable!("Wrong operand on move arg dest");
                }
            }
            CfgInstruction::Call { dest, src } => Instruction::call(dest, src),
            CfgInstruction::Print { src } => Instruction::print(src),
        }
    }
}

fn resolve_backpatches(
    instructions: &mut [Instruction],
    pending_backpatch: &[(usize, usize)],
    bb_start_index: &HashMap<usize, usize>,
) {
    for (instruction_index, bb_index) in pending_backpatch.iter().copied() {
        let instruction = &mut instructions[instruction_index];
        let bb_start_index = bb_start_index[&bb_index];

        let new_offset = bb_start_index as i16 - instruction_index as i16;

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

fn map_cfg_constants(constants: &[CfgConstant]) -> Vec<Value> {
    constants
        .iter()
        .map(|constant| match constant {
            CfgConstant::Boolean(v) => Value::boolean(*v),
            CfgConstant::Number(v) => Value::number(**v),
            CfgConstant::Function(index) => Value::function(*index),
            _ => todo!(),
        })
        .collect()
}
