use std::collections::HashMap;

use crate::{
    bytecode::op_code::Opcode,
    cfg_ir::{
        basic_block::{BasicBlock, Terminator},
        cfg_constants::CfgConstant,
        cfg_function::CfgFunction,
        graph_traversal::reversed_postorder,
        instruction::{CfgOpcode, Instruction},
        operand::Operand,
    },
};

use super::{bytecode::Bytecode, function::Function, value::Value};

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

    instructions.push(Opcode::Halt as i16);

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
    instructions: &'a mut Vec<i16>,
}

impl<'a> CodegenContext<'a> {
    fn new(
        basic_blocks: &'a [BasicBlock],
        frame_size: usize,
        instructions: &'a mut Vec<i16>,
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
            self.visit_instruction(instruction);
        }

        match basic_block.terminator {
            Terminator::Branch {
                src,
                r#true,
                r#false,
            } => {
                if Some(r#true) != next_bb_index {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#true));

                    self.instructions.push(Opcode::JumpIfTrue as i16);
                    self.instructions.push(src.to_i16());

                    self.instructions.push(0);
                }

                if Some(r#false) != next_bb_index {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#false));

                    self.instructions.push(Opcode::JumpIfFalse as i16);
                    self.instructions.push(src.to_i16());

                    self.instructions.push(0);
                }
            }
            Terminator::Goto(target) => {
                if Some(target) != next_bb_index {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, target));

                    self.instructions.push(Opcode::Jump as i16);
                    self.instructions.push(0);
                }
            }
            Terminator::Return { src } => {
                if let Some(src) = src {
                    self.instructions.push(Opcode::Return as i16);
                    self.instructions.push(src.to_i16());
                } else {
                    self.instructions.push(Opcode::ReturnVoid as i16);
                }
            }
            Terminator::None => {}
        };
    }

    fn visit_instruction(&mut self, instruction: &Instruction) {
        let Instruction {
            op_code,
            mut dest,
            src1,
            src2,
        } = *instruction;
        use CfgOpcode::*;

        let op_code = match op_code {
            // === Arithmetic ===
            Add => Opcode::Add,
            Subtract => Opcode::Subtract,
            Multiply => Opcode::Multiply,
            Divide => Opcode::Divide,
            Modulo => Opcode::Modulo,

            // === Comparison ===
            Equal => Opcode::Equal,
            NotEqual => Opcode::NotEqual,
            Greater => Opcode::Greater,
            GreaterEqual => Opcode::GreaterEqual,
            Less => Opcode::Less,
            LessEqual => Opcode::LessEqual,

            // === Unary ===
            Negate => Opcode::Negate,
            Not => Opcode::Not,
            Move => Opcode::Move,
            // === Data movement ===
            MoveArg => {
                if let Operand::Variable(value) = dest {
                    dest = Operand::Variable(self.frame_size + value);
                }

                Opcode::Move
            }

            // === Function and control ===
            Call => Opcode::Call,
            Print => Opcode::Print,

            _ => unreachable!("Unsupported opcode: {:?}", op_code),
        };

        self.instructions.push(op_code as i16);

        match dest {
            Operand::Constant(_) | Operand::Variable(_) => self.instructions.push(dest.to_i16()),
            _ => {}
        }

        match src1 {
            Operand::Constant(_) | Operand::Variable(_) => self.instructions.push(src1.to_i16()),
            _ => {}
        }

        match src2 {
            Operand::Constant(_) | Operand::Variable(_) => self.instructions.push(src2.to_i16()),
            _ => {}
        }
    }
}

fn resolve_backpatches(
    instructions: &mut [i16],
    pending_backpatch: &[(usize, usize)],
    bb_start_index: &HashMap<usize, usize>,
) {
    for (instruction_index, bb_index) in pending_backpatch.iter().copied() {
        let bb_start_index = bb_start_index[&bb_index];

        let offset = bb_start_index as i16 - instruction_index as i16;

        let instruction = instructions[instruction_index];
        let op_code = Opcode::from(instruction as u16);

        if let Opcode::Jump = op_code {
            instructions[instruction_index + 1] = offset;
        } else {
            instructions[instruction_index + 2] = offset;
        }
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
