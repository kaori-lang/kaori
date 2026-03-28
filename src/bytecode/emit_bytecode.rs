use std::collections::HashMap;

use crate::cfg::{
    self,
    basic_block::{BasicBlock, Terminator},
    constant_pool::Constant,
    function::Function,
    graph_traversal::reversed_postorder,
    operand::Operand,
};

use super::{bytecode::Bytecode, function, instruction::Instruction, value::Value};

pub fn emit_bytecode(functions: Vec<Function>) -> Bytecode {
    let mut instructions = Vec::new();
    let mut fn_id_to_fn_index = HashMap::new();
    let mut fn_start_end = HashMap::new();

    for (fn_index, function) in functions.iter().enumerate() {
        let fn_start_index = instructions.len();

        fn_id_to_fn_index.insert(function.id, fn_index);

        let mut context = FunctionContext::new(
            &function.basic_blocks,
            function.allocated_variables,
            &mut instructions,
        );

        context.emit_instructions();

        let fn_end_index = instructions.len() - 1;

        fn_start_end.insert(function.id, (fn_start_index, fn_end_index));
    }

    instructions.push(Instruction::Halt);

    let base_ptr = instructions.as_ptr();

    let compiled_functions = functions
        .iter()
        .map(|function| {
            let constant_pool = function
                .constant_pool
                .iter()
                .map(|constant| match constant {
                    Constant::Boolean(v) => Value::boolean(*v),
                    Constant::Number(v) => Value::number(**v),
                    Constant::Function(id) => {
                        let function_index = fn_id_to_fn_index.get(id).unwrap();

                        Value::function(*function_index)
                    }
                    _ => todo!(),
                })
                .collect();

            let frame_size = function.allocated_variables;

            let (start_index, end_index) = *fn_start_end.get(&function.id).unwrap();
            let start = unsafe { base_ptr.add(start_index) };
            let end = unsafe { base_ptr.add(end_index) };

            function::Function::new(start, end, frame_size as u8, constant_pool)
        })
        .collect();

    Bytecode::new(instructions, compiled_functions)
}

struct FunctionContext<'a> {
    basic_blocks: &'a [BasicBlock],
    frame_size: usize,
    instructions: &'a mut Vec<Instruction>,
}

impl<'a> FunctionContext<'a> {
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

        for (i, &index) in basic_blocks.iter().enumerate() {
            bb_start_index.insert(index, self.instructions.len());

            let next_index = basic_blocks.get(i + 1).copied();
            self.visit_block(index, next_index, &mut pending_backpatch);
        }

        resolve_backpatches(self.instructions, &pending_backpatch, &bb_start_index);
    }

    fn visit_block(
        &mut self,
        index: usize,
        next_index: Option<usize>,
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
                if next_index != Some(r#true) {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#true));
                    self.instructions.push(Instruction::JumpIfTrue {
                        src: src.to_i16(),
                        offset: 0,
                    });
                }

                if next_index != Some(r#false) {
                    let index = self.instructions.len();
                    pending_backpatch.push((index, r#false));
                    self.instructions.push(Instruction::JumpIfFalse {
                        src: src.to_i16(),
                        offset: 0,
                    });
                }
            }
            Terminator::Goto(target) => {
                if next_index != Some(target) {
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
