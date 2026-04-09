use std::collections::HashMap;

use crate::cfg::{
    self,
    basic_block::{BasicBlock, Terminator},
    graph_traversal::reversed_postorder,
    operand::Operand,
};

use super::{Constant, function::Function, instruction::Instruction};

pub fn emit_bytecode(functions: Vec<cfg::Function>) -> Vec<Function> {
    let mut fn_id_to_fn_index = HashMap::new();

    for (index, function) in functions.iter().enumerate() {
        fn_id_to_fn_index.insert(function.id, index);
    }

    let mut functions = functions
        .iter()
        .map(|function| {
            let mut context =
                FunctionContext::new(&function.basic_blocks, function.registers_count);

            let instructions = context.emit_instructions();

            let constant_pool = function
                .constant_pool
                .iter()
                .map(|constant| match constant {
                    cfg::Constant::Boolean(value) => Constant::Boolean(*value),
                    cfg::Constant::Number(value) => Constant::Number(**value),
                    cfg::Constant::Function(id) => {
                        let function_index = fn_id_to_fn_index.get(id).unwrap();

                        Constant::Function(*function_index)
                    }
                    cfg::Constant::String(value) => Constant::String(value.to_owned()),
                })
                .collect();

            let registers_count = function.registers_count as u8;

            Function::new(instructions, registers_count, constant_pool)
        })
        .collect::<Vec<Function>>();

    if let Some(main) = functions.first_mut() {
        main.instructions.push(Instruction::Halt);
    }

    functions
}

struct FunctionContext<'a> {
    pub basic_blocks: &'a [BasicBlock],
    pub registers_count: usize,
}

impl<'a> FunctionContext<'a> {
    fn new(basic_blocks: &'a [BasicBlock], registers_count: usize) -> Self {
        Self {
            basic_blocks,
            registers_count,
        }
    }

    fn emit_instructions(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let basic_blocks = reversed_postorder(self.basic_blocks);

        let mut pending_backpatch = Vec::new();
        let mut bb_start_index = HashMap::new();

        for (i, &index) in basic_blocks.iter().enumerate() {
            bb_start_index.insert(index, instructions.len());

            let next_index = basic_blocks.get(i + 1).copied();
            self.visit_block(index, next_index, &mut pending_backpatch, &mut instructions);
        }

        resolve_backpatches(&mut instructions, &pending_backpatch, &bb_start_index);

        instructions
    }

    fn visit_block(
        &mut self,
        index: usize,
        next_index: Option<usize>,
        pending_backpatch: &mut Vec<(usize, usize)>,
        instructions: &mut Vec<Instruction>,
    ) {
        let basic_block = &self.basic_blocks[index];

        for instruction in &basic_block.instructions {
            self.visit_instruction(instruction, instructions);
        }

        let Some(terminator) = basic_block.terminator else {
            panic!("Terminator missing!");
        };

        match terminator {
            Terminator::Branch {
                src,
                r#true,
                r#false,
            } => {
                if next_index != Some(r#true) {
                    let index = instructions.len();
                    pending_backpatch.push((index, r#true));
                    instructions.push(Instruction::JumpIfTrue {
                        src: src.to_i16(),
                        offset: 0,
                    });
                }

                if next_index != Some(r#false) {
                    let index = instructions.len();
                    pending_backpatch.push((index, r#false));
                    instructions.push(Instruction::JumpIfFalse {
                        src: src.to_i16(),
                        offset: 0,
                    });
                }
            }
            Terminator::Goto(target) => {
                if next_index != Some(target) {
                    let index = instructions.len();
                    pending_backpatch.push((index, target));
                    instructions.push(Instruction::Jump { offset: 0 });
                }
            }
            Terminator::Return { src } => {
                if let Some(src) = src {
                    instructions.push(Instruction::Return { src: src.to_i16() });
                } else {
                    instructions.push(Instruction::ReturnVoid);
                }
            }
        };
    }

    fn visit_instruction(
        &mut self,
        instruction: &cfg::Instruction,
        instructions: &mut Vec<Instruction>,
    ) {
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
            cfg::Instruction::Power { dest, src1, src2 } => Instruction::Power {
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
                    Operand::Variable(value) => Operand::Variable(self.registers_count + value),
                    _ => *dest,
                };

                Instruction::Move {
                    dest: dest.to_u16(),
                    src: src.to_i16(),
                }
            }
            cfg::Instruction::SetField { object, key, value } => Instruction::SetField {
                object: object.to_u16(),
                key: key.to_i16(),
                value: value.to_i16(),
            },
            cfg::Instruction::GetField { dest, object, key } => Instruction::GetField {
                dest: dest.to_u16(),
                object: object.to_i16(),
                key: key.to_i16(),
            },
            cfg::Instruction::CreateDict { dest } => Instruction::CreateDict {
                dest: dest.to_u16(),
            },
            cfg::Instruction::Call { dest, func } => Instruction::Call {
                dest: dest.to_u16(),
                src: func.to_i16(),
            },
            cfg::Instruction::Print { src } => Instruction::Print { src: src.to_i16() },
        };

        instructions.push(instruction);
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
