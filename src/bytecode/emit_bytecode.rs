use std::collections::HashMap;

use crate::{
    bytecode::op_code::Opcode,
    cfg_ir::{
        basic_block::{BasicBlock, Terminator},
        cfg_constants::CfgConstant,
        cfg_function::CfgFunction,
        cfg_instruction::{CfgInstruction, CfgOpcode},
        graph_traversal::reversed_postorder,
        operand::Operand,
    },
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

    fn visit_instruction(&self, instruction: &CfgInstruction) {
        let CfgInstruction {
            op_code,
            dest,
            src1,
            src2,
        } = instruction;
        use CfgOpcode::*;
        use Operand::*;

        let op_code = match (op_code, src1, src2) {
            // === Arithmetic ===
            (Add, Variable(_), Variable(_)) => Opcode::AddRR,
            (Add, Variable(_), Constant(_)) => Opcode::AddRK,
            (Add, Constant(_), Variable(_)) => Opcode::AddKR,
            (Add, Constant(_), Constant(_)) => Opcode::AddKK,

            (Subtract, Variable(_), Variable(_)) => Opcode::SubtractRR,
            (Subtract, Variable(_), Constant(_)) => Opcode::SubtractRK,
            (Subtract, Constant(_), Variable(_)) => Opcode::SubtractKR,
            (Subtract, Constant(_), Constant(_)) => Opcode::SubtractKK,

            (Multiply, Variable(_), Variable(_)) => Opcode::MultiplyRR,
            (Multiply, Variable(_), Constant(_)) => Opcode::MultiplyRK,
            (Multiply, Constant(_), Variable(_)) => Opcode::MultiplyKR,
            (Multiply, Constant(_), Constant(_)) => Opcode::MultiplyKK,

            (Divide, Variable(_), Variable(_)) => Opcode::DivideRR,
            (Divide, Variable(_), Constant(_)) => Opcode::DivideRK,
            (Divide, Constant(_), Variable(_)) => Opcode::DivideKR,
            (Divide, Constant(_), Constant(_)) => Opcode::DivideKK,

            (Modulo, Variable(_), Variable(_)) => Opcode::ModuloRR,
            (Modulo, Variable(_), Constant(_)) => Opcode::ModuloRK,
            (Modulo, Constant(_), Variable(_)) => Opcode::ModuloKR,
            (Modulo, Constant(_), Constant(_)) => Opcode::ModuloKK,

            // === Comparison ===
            (Equal, Variable(_), Variable(_)) => Opcode::EqualRR,
            (Equal, Variable(_), Constant(_)) => Opcode::EqualRK,
            (Equal, Constant(_), Variable(_)) => Opcode::EqualKR,
            (Equal, Constant(_), Constant(_)) => Opcode::EqualKK,

            (NotEqual, Variable(_), Variable(_)) => Opcode::NotEqualRR,
            (NotEqual, Variable(_), Constant(_)) => Opcode::NotEqualRK,
            (NotEqual, Constant(_), Variable(_)) => Opcode::NotEqualKR,
            (NotEqual, Constant(_), Constant(_)) => Opcode::NotEqualKK,

            (Greater, Variable(_), Variable(_)) => Opcode::GreaterRR,
            (Greater, Variable(_), Constant(_)) => Opcode::GreaterRK,
            (Greater, Constant(_), Variable(_)) => Opcode::GreaterKR,
            (Greater, Constant(_), Constant(_)) => Opcode::GreaterKK,

            (GreaterEqual, Variable(_), Variable(_)) => Opcode::GreaterEqualRR,
            (GreaterEqual, Variable(_), Constant(_)) => Opcode::GreaterEqualRK,
            (GreaterEqual, Constant(_), Variable(_)) => Opcode::GreaterEqualKR,
            (GreaterEqual, Constant(_), Constant(_)) => Opcode::GreaterEqualKK,

            (Less, Variable(_), Variable(_)) => Opcode::LessRR,
            (Less, Variable(_), Constant(_)) => Opcode::LessRK,
            (Less, Constant(_), Variable(_)) => Opcode::LessKR,
            (Less, Constant(_), Constant(_)) => Opcode::LessKK,

            (LessEqual, Variable(_), Variable(_)) => Opcode::LessEqualRR,
            (LessEqual, Variable(_), Constant(_)) => Opcode::LessEqualRK,
            (LessEqual, Constant(_), Variable(_)) => Opcode::LessEqualKR,
            (LessEqual, Constant(_), Constant(_)) => Opcode::LessEqualKK,

            // === Unary ===
            (Negate, Variable(_), None) => Opcode::NegateR,
            (Negate, Constant(_), None) => Opcode::NegateK,
            (Not, Variable(_), None) => Opcode::NotR,
            (Not, Constant(_), None) => Opcode::NotK,

            // === Data movement ===
            (Move, Variable(_), None) => Opcode::MoveR,
            (Move, Constant(_), None) => Opcode::MoveK,

            (MoveArg, Variable(_), None) => Opcode::MoveR,
            (MoveArg, Constant(_), None) => Opcode::MoveK,

            // === Function and control ===
            (Call, Variable(_), _) => Opcode::CallR,
            (Call, Constant(_), _) => Opcode::CallK,

            (Print, Variable(_), _) => Opcode::PrintR,
            (Print, Constant(_), _) => Opcode::PrintK,

            // === Fallback ===
            _ => unreachable!(
                "Invalid operand combination for {:?}: {:?}, {:?}",
                op_code, src1, src2
            ),
        };
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
