use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_constants::CfgConstant,
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    graph_traversal::reversed_postorder,
};

use super::{bytecode::Bytecode, instruction::Instruction, value::Value};

pub fn generate_bytecode(cfg_ir: &CfgIr) -> Bytecode {
    let mut context = CodegenContext {
        basic_blocks: &cfg_ir.basic_blocks,
        instructions: Vec::new(),
        bb_start_index: HashMap::new(),
        pending_backpatch: Vec::new(),
    };

    for cfg in &cfg_ir.cfgs {
        context.visit_cfg(*cfg);
    }

    context.instructions.push(Instruction::Halt);

    let constants = convert_constants(
        &cfg_ir.constants.constants,
        &context.instructions,
        &context.bb_start_index,
    );

    Bytecode::new(context.instructions, constants)
}

struct CodegenContext<'a> {
    basic_blocks: &'a [BasicBlock],
    instructions: Vec<Instruction>,
    bb_start_index: HashMap<BlockId, usize>,
    pending_backpatch: Vec<(usize, BlockId)>,
}

impl<'a> CodegenContext<'a> {
    fn visit_cfg(&mut self, cfg: BlockId) {
        let blocks = reversed_postorder(cfg, self.basic_blocks);

        for (index, bb_id) in blocks.iter().enumerate() {
            self.bb_start_index.insert(*bb_id, self.instructions.len());
            let next_bb_id = blocks.get(index + 1).copied();
            self.visit_block(*bb_id, next_bb_id);
        }

        self.resolve_backpatches();
    }

    fn visit_block(&mut self, bb_id: BlockId, next_bb_id: Option<BlockId>) {
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
            } => {}

            Terminator::Goto(target) => {
                if Some(target) != next_bb_id {
                    let instruction = Instruction::jump(0);
                    let index = self.instructions.len();
                    self.pending_backpatch.push((index, target));
                    self.instructions.push(instruction);
                }
            }

            Terminator::Return { src } => {
                self.instructions.push(Instruction::return_(src));
            }

            _ => {}
        };
    }

    fn resolve_backpatches(&mut self) {
        for (instr_index, target_block) in &self.pending_backpatch {
            if let Some(&target_idx) = self.bb_start_index.get(target_block) {
            } else {
                panic!("Unresolved block target {:?}", target_block);
            }
        }

        self.pending_backpatch.clear();
    }
}

fn convert_constants(
    cfg_constants: &[CfgConstant],
    instructions: &[Instruction],
    bb_start_index: &HashMap<BlockId, usize>,
) -> Vec<Value> {
    let mut constants = Vec::new();

    constants.push(Value::default());

    for constant in cfg_constants {
        let value = match constant {
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
        };

        constants.push(value);
    }

    constants
}

fn visit_instruction(instr: &CfgInstruction) -> Instruction {
    match *instr {
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
        CfgInstruction::Move { dest, src } => Instruction::mov(dest, src),
        CfgInstruction::Call {
            dest,
            src,
            caller_size,
        } => Instruction::call(dest, src, caller_size),
        CfgInstruction::Print { src } => Instruction::print(src),
    }
}
