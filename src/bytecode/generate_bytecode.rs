use std::collections::HashMap;

use crate::cfg_ir::{
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_constants::CfgConstant,
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    graph_traversal::reversed_postorder,
};

use super::{bytecode::Bytecode, instruction::Instruction, value::Value};

fn convert_constants(cfg_constants: &[CfgConstant], instructions: &[Instruction]) -> Vec<Value> {
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
pub fn generate_bytecode(cfg_ir: &CfgIr) -> Bytecode {
    let mut instructions = Vec::new();

    for cfg in &cfg_ir.cfgs {
        visit_cfg(*cfg, &cfg_ir.basic_blocks, &mut instructions);
    }

    instructions.push(Instruction::Halt);

    let constants = convert_constants(&cfg_ir.constants.constants, &instructions);

    Bytecode::new(instructions, constants)
}

fn visit_cfg(cfg: BlockId, basic_blocks: &[BasicBlock], instructions: &mut Vec<Instruction>) {
    let blocks = reversed_postorder(cfg, basic_blocks);

    let mut bb_start_index = HashMap::new();

    for id in blocks {
        bb_start_index.insert(id, instructions.len());

        let basic_block = &basic_blocks[id.0];

        visit_block(basic_block, instructions);
    }
}

fn visit_block(basic_block: &BasicBlock, instructions: &mut Vec<Instruction>) {
    for instruction in &basic_block.instructions {
        let instruction = visit_instruction(instruction);

        instructions.push(instruction);
    }

    match basic_block.terminator {
        Terminator::Branch {
            src,
            r#true,
            r#false,
        } => {}
        Terminator::Goto(target) => {}
        Terminator::Return { src } => {}
        _ => {}
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
        CfgInstruction::Move { dest, src } => Instruction::mov(dest, src),
        CfgInstruction::Call {
            dest,
            src,
            caller_size,
        } => Instruction::call(dest, src, caller_size),
        CfgInstruction::Print { src } => Instruction::print(src),
    }
}
