use std::collections::HashMap;

use crate::cfg::{
    self,
    basic_block::{BasicBlock, Terminator},
    constants::Constant,
    function::Function,
    graph_traversal::reversed_postorder,
    operand::Operand,
};

use super::{bytecode::Bytecode, function, instruction::Instruction, value::Value};

pub fn emit_bytecode(cfgs: Vec<Function>) -> Bytecode {
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
        let constant_pool = map_constant_pool(&cfg.constant_pool);
        let frame_size = cfg.allocated_variables;

        let ip = unsafe { instructions.as_ptr().add(functions_start_index[index]) };

        let function = function::Function::new(ip, frame_size as u8, constant_pool);

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

                    self.instructions.push(Instruction::JumpIfTrue {
                        src: src.to_i16(),
                        offset: 0,
                    });
                }

                if Some(r#false) != next_bb_index {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#false));

                    self.instructions.push(Instruction::JumpIfFalse {
                        src: src.to_i16(),
                        offset: 0,
                    });
                }
            }
            Terminator::Goto(target) => {
                if Some(target) != next_bb_index {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, target));

                    self.instructions.push(Instruction::Jump { offset: 0 });
                }
            }
            Terminator::Return { src } => {
                if let Some(src) = src {
                    self.instructions
                        .push(Instruction::Return { src: src.to_i16() });
                } else {
                    self.instructions.push(Instruction::ReturnVoid);
                }
            }
            Terminator::None => {
                panic!("Terminator missing!")
            }
        };
    }

    fn visit_instruction(&mut self, instruction: &cfg::Instruction) {
        let instruction = match instruction {
            cfg::Instruction::Add { dest, src1, src2 } => Instruction::Add {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Subtract { dest, src1, src2 } => Instruction::Subtract {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Multiply { dest, src1, src2 } => Instruction::Multiply {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Divide { dest, src1, src2 } => Instruction::Divide {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Modulo { dest, src1, src2 } => Instruction::Modulo {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Equal { dest, src1, src2 } => Instruction::Equal {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::NotEqual { dest, src1, src2 } => Instruction::NotEqual {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Greater { dest, src1, src2 } => Instruction::Greater {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::GreaterEqual { dest, src1, src2 } => Instruction::GreaterEqual {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Less { dest, src1, src2 } => Instruction::Less {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::LessEqual { dest, src1, src2 } => Instruction::LessEqual {
                dest: dest.to_u16(),
                src1: src1.to_i16(),
                src2: src2.to_i16(),
            },
            cfg::Instruction::Negate { dest, src } => Instruction::Negate {
                dest: dest.to_u16(),
                src: src.to_i16(),
            },

            cfg::Instruction::Not { dest, src } => Instruction::Not {
                dest: dest.to_u16(),
                src: src.to_i16(),
            },
            cfg::Instruction::Move { dest, src } => Instruction::Move {
                dest: dest.to_u16(),
                src: src.to_i16(),
            },

            cfg::Instruction::MoveArg { dest, src } => {
                let dest = match dest {
                    Operand::Variable(value) => Operand::Variable(self.frame_size + value),
                    _ => *dest,
                };

                Instruction::Move {
                    dest: dest.to_u16(),
                    src: src.to_i16(),
                }
            }
            cfg::Instruction::Call { dest, func } => Instruction::Call {
                dest: dest.to_u16(),
                src: func.to_i16(),
            },
            cfg::Instruction::Print { src } => Instruction::Print { src: src.to_i16() },
        };

        self.instructions.push(instruction);
    }
}

fn resolve_backpatches(
    instructions: &mut [Instruction],
    pending_backpatch: &[(usize, usize)],
    bb_start_index: &HashMap<usize, usize>,
) {
    for (instruction_index, bb_index) in pending_backpatch.iter().copied() {
        let bb_start_index = bb_start_index[&bb_index];

        let offset = bb_start_index as i16 - instruction_index as i16;

        match instructions[instruction_index] {
            Instruction::Jump { .. } => {
                instructions[instruction_index] = Instruction::Jump { offset };
            }
            Instruction::JumpIfTrue { src, .. } => {
                instructions[instruction_index] = Instruction::JumpIfTrue { src, offset };
            }
            Instruction::JumpIfFalse { src, .. } => {
                instructions[instruction_index] = Instruction::JumpIfFalse { src, offset };
            }
            _ => unreachable!("Wrong jump instruction"),
        }
    }
}

fn map_constant_pool(constants: &[Constant]) -> Vec<Value> {
    constants
        .iter()
        .map(|constant| match constant {
            Constant::Boolean(v) => Value::boolean(*v),
            Constant::Number(v) => Value::number(**v),
            Constant::Function(index) => Value::function(*index),
            _ => todo!(),
        })
        .collect()
}
