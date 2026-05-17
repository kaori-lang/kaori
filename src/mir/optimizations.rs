use std::{collections::HashMap, ops::Range};

use crate::mir::{
    function::Function,
    instruction::{Instruction, Register},
};

impl Function {
    pub fn run_optimization_passes(&mut self) {
        let basic_blocks = self.build_cfg();

        Self::eliminate_dead_code(&mut self.instructions);

        for basic_block in basic_blocks.iter() {
            /*     self.coalesce_copies(basic_block);
            self.fuse_compare_branch(basic_block); */
        }

        //self.eliminate_nops();

        println!("{:?}", self.live_ranges);
        let registers_map = self.allocate_registers();

        /*  let frame_size = registers_map.values().copied().max().unwrap() + 1;

        self.patch_move_args(frame_size);

        self.registers = frame_size; */
    }

    fn patch_move_args(&mut self, frame_size: u16) {
        for instruction in self.instructions.iter_mut() {
            if let Instruction::MoveArg { dest, .. } = instruction {
                *dest = Register(dest.0 + frame_size);
            }
        }
    }

    fn build_cfg(&self) -> Vec<Range<usize>> {
        let mut leaders = vec![false; self.instructions.len()];
        leaders[0] = true;

        for (index, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                Instruction::Jump { offset }
                | Instruction::JumpIfFalse { offset, .. }
                | Instruction::JumpIfTrue { offset, .. } => {
                    let target = (index as i32 + offset) as usize;
                    leaders[target] = true;
                    leaders[index + 1] = true;
                }
                Instruction::Return { .. } => {
                    if index + 1 < self.instructions.len() {
                        leaders[index + 1] = true;
                    }
                }
                _ => {}
            }
        }

        let mut basic_blocks = Vec::new();
        let mut start = 0;

        for (end, leader) in leaders.iter().copied().enumerate().skip(1) {
            if leader {
                basic_blocks.push(start..end);
                start = end;
            }
        }

        let end = self.instructions.len() - 1;
        basic_blocks.push(start..end);

        basic_blocks
    }

    fn allocate_registers(&self) {
        /* let mut sorted_ranges = Vec::new();
        let mut registers_map = HashMap::new();

        for (&register, range) in &self.live_ranges {
            sorted_ranges.push((register, range.clone()));
        }

        sorted_ranges.sort_by(|a, b| a.1.start.cmp(&b.1.start).then(a.0.cmp(&b.0))); */
    }

    fn rewrite_instructions(&mut self, assignment: &HashMap<Register, Register>) {
        let replace = |register: Register| *assignment.get(&register).unwrap();

        for instruction in &mut self.instructions {
            *instruction = match *instruction {
                Instruction::Add { dest, src1, src2 } => Instruction::Add {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::AddK { dest, src1, src2 } => Instruction::AddK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::Subtract { dest, src1, src2 } => Instruction::Subtract {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::SubtractRK { dest, src1, src2 } => Instruction::SubtractRK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::SubtractKR { dest, src1, src2 } => Instruction::SubtractKR {
                    dest: replace(dest),
                    src1,
                    src2: replace(src2),
                },
                Instruction::Multiply { dest, src1, src2 } => Instruction::Multiply {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::MultiplyK { dest, src1, src2 } => Instruction::MultiplyK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::Divide { dest, src1, src2 } => Instruction::Divide {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::DivideRK { dest, src1, src2 } => Instruction::DivideRK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::DivideKR { dest, src1, src2 } => Instruction::DivideKR {
                    dest: replace(dest),
                    src1,
                    src2: replace(src2),
                },
                Instruction::Modulo { dest, src1, src2 } => Instruction::Modulo {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::ModuloRK { dest, src1, src2 } => Instruction::ModuloRK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::ModuloKR { dest, src1, src2 } => Instruction::ModuloKR {
                    dest: replace(dest),
                    src1,
                    src2: replace(src2),
                },
                Instruction::Equal { dest, src1, src2 } => Instruction::Equal {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::EqualK { dest, src1, src2 } => Instruction::EqualK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::NotEqual { dest, src1, src2 } => Instruction::NotEqual {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::NotEqualK { dest, src1, src2 } => Instruction::NotEqualK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::Less { dest, src1, src2 } => Instruction::Less {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::LessK { dest, src1, src2 } => Instruction::LessK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::LessEqual { dest, src1, src2 } => Instruction::LessEqual {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::LessEqualK { dest, src1, src2 } => Instruction::LessEqualK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::Greater { dest, src1, src2 } => Instruction::Greater {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::GreaterK { dest, src1, src2 } => Instruction::GreaterK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::GreaterEqual { dest, src1, src2 } => Instruction::GreaterEqual {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2: replace(src2),
                },
                Instruction::GreaterEqualK { dest, src1, src2 } => Instruction::GreaterEqualK {
                    dest: replace(dest),
                    src1: replace(src1),
                    src2,
                },
                Instruction::Not { dest, src } => Instruction::Not {
                    dest: replace(dest),
                    src: replace(src),
                },
                Instruction::Negate { dest, src } => Instruction::Negate {
                    dest: replace(dest),
                    src: replace(src),
                },
                Instruction::Move { dest, src } => Instruction::Move {
                    dest: replace(dest),
                    src: replace(src),
                },
                Instruction::MoveArg { dest, src } => Instruction::MoveArg {
                    dest: replace(dest),
                    src: replace(src),
                },
                Instruction::LoadK { dest, src } => Instruction::LoadK {
                    dest: replace(dest),
                    src,
                },
                Instruction::CreateDict { dest } => Instruction::CreateDict {
                    dest: replace(dest),
                },
                Instruction::SetField { object, key, value } => Instruction::SetField {
                    object: replace(object),
                    key: replace(key),
                    value: replace(value),
                },
                Instruction::GetField { dest, object, key } => Instruction::GetField {
                    dest: replace(dest),
                    object: replace(object),
                    key: replace(key),
                },
                Instruction::CreateClosure { dest, src } => Instruction::CreateClosure {
                    dest: replace(dest),
                    src,
                },
                Instruction::CaptureValue { dest, src } => Instruction::CaptureValue {
                    dest: replace(dest),
                    src: replace(src),
                },
                Instruction::Call { dest, src, arity } => Instruction::Call {
                    dest: replace(dest),
                    src: replace(src),
                    arity,
                },
                Instruction::Return { src } => Instruction::Return { src: replace(src) },
                Instruction::JumpIfFalse { src, offset } => Instruction::JumpIfFalse {
                    src: replace(src),
                    offset,
                },
                Instruction::JumpIfTrue { src, offset } => Instruction::JumpIfTrue {
                    src: replace(src),
                    offset,
                },
                Instruction::JumpIfLess { src1, src2, offset } => Instruction::JumpIfLess {
                    src1: replace(src1),
                    src2: replace(src2),
                    offset,
                },
                Instruction::JumpIfLessK { src1, src2, offset } => Instruction::JumpIfLessK {
                    src1: replace(src1),
                    src2,
                    offset,
                },
                Instruction::JumpIfLessEqual { src1, src2, offset } => {
                    Instruction::JumpIfLessEqual {
                        src1: replace(src1),
                        src2: replace(src2),
                        offset,
                    }
                }
                Instruction::JumpIfLessEqualK { src1, src2, offset } => {
                    Instruction::JumpIfLessEqualK {
                        src1: replace(src1),
                        src2,
                        offset,
                    }
                }
                Instruction::JumpIfGreater { src1, src2, offset } => Instruction::JumpIfGreater {
                    src1: replace(src1),
                    src2: replace(src2),
                    offset,
                },
                Instruction::JumpIfGreaterK { src1, src2, offset } => Instruction::JumpIfGreaterK {
                    src1: replace(src1),
                    src2,
                    offset,
                },
                Instruction::JumpIfGreaterEqual { src1, src2, offset } => {
                    Instruction::JumpIfGreaterEqual {
                        src1: replace(src1),
                        src2: replace(src2),
                        offset,
                    }
                }
                Instruction::JumpIfGreaterEqualK { src1, src2, offset } => {
                    Instruction::JumpIfGreaterEqualK {
                        src1: replace(src1),
                        src2,
                        offset,
                    }
                }
                Instruction::JumpIfEqual { src1, src2, offset } => Instruction::JumpIfEqual {
                    src1: replace(src1),
                    src2: replace(src2),
                    offset,
                },
                Instruction::JumpIfEqualK { src1, src2, offset } => Instruction::JumpIfEqualK {
                    src1: replace(src1),
                    src2,
                    offset,
                },
                Instruction::JumpIfNotEqual { src1, src2, offset } => Instruction::JumpIfNotEqual {
                    src1: replace(src1),
                    src2: replace(src2),
                    offset,
                },
                Instruction::JumpIfNotEqualK { src1, src2, offset } => {
                    Instruction::JumpIfNotEqualK {
                        src1: replace(src1),
                        src2,
                        offset,
                    }
                }
                other => other, // Jump, Nop — no registers
            };
        }
    }

    fn eliminate_dead_code(instructions: &mut [Instruction]) {
        let mut reachable = vec![false; instructions.len()];
        let mut stack = vec![0usize];

        while let Some(index) = stack.pop() {
            if reachable[index] {
                continue;
            }
            reachable[index] = true;

            match instructions[index] {
                Instruction::Jump { offset } => {
                    let target = (index as i32 + offset) as usize;
                    stack.push(target);
                }
                Instruction::JumpIfFalse { offset, .. }
                | Instruction::JumpIfTrue { offset, .. } => {
                    let target = (index as i32 + offset) as usize;
                    stack.push(target);
                    stack.push(index + 1);
                }
                Instruction::Return { .. } => {}
                _ => stack.push(index + 1),
            }
        }

        for index in 0..instructions.len() {
            if !reachable[index] {
                instructions[index] = Instruction::Nop;
            }
        }
    }

    fn coalesce_copies(&mut self, basic_block: &Range<usize>) {
        let instructions = &mut self.instructions[basic_block.start..basic_block.end];

        for i in 0..instructions.len() {
            let (move_dest, src) = match instructions[i] {
                Instruction::Move { dest, src } => (dest, src),
                _ => continue,
            };

            for j in (0..i).rev() {
                let instruction = &mut instructions[j];
                match instruction {
                    Instruction::Add { dest, .. }
                    | Instruction::AddK { dest, .. }
                    | Instruction::Subtract { dest, .. }
                    | Instruction::SubtractRK { dest, .. }
                    | Instruction::SubtractKR { dest, .. }
                    | Instruction::Multiply { dest, .. }
                    | Instruction::MultiplyK { dest, .. }
                    | Instruction::Divide { dest, .. }
                    | Instruction::DivideRK { dest, .. }
                    | Instruction::DivideKR { dest, .. }
                    | Instruction::Modulo { dest, .. }
                    | Instruction::ModuloRK { dest, .. }
                    | Instruction::ModuloKR { dest, .. }
                    | Instruction::Equal { dest, .. }
                    | Instruction::EqualK { dest, .. }
                    | Instruction::NotEqual { dest, .. }
                    | Instruction::NotEqualK { dest, .. }
                    | Instruction::Less { dest, .. }
                    | Instruction::LessK { dest, .. }
                    | Instruction::LessEqual { dest, .. }
                    | Instruction::LessEqualK { dest, .. }
                    | Instruction::Greater { dest, .. }
                    | Instruction::GreaterK { dest, .. }
                    | Instruction::GreaterEqual { dest, .. }
                    | Instruction::GreaterEqualK { dest, .. }
                    | Instruction::Not { dest, .. }
                    | Instruction::Negate { dest, .. }
                    | Instruction::MoveArg { dest, .. }
                    | Instruction::Move { dest, .. }
                    | Instruction::LoadK { dest, .. }
                    | Instruction::CreateDict { dest }
                    | Instruction::GetField { dest, .. }
                    | Instruction::Call { dest, .. } => {
                        let live_range = &self.live_ranges[dest.0 as usize];
                        let register_lives = live_range.contains(&i);

                        if !register_lives && *dest == src {
                            *dest = move_dest;
                            instructions[i] = Instruction::Nop;

                            break;
                        }
                    }
                    Instruction::Nop => {}
                    _ => {
                        break;
                    }
                }
            }
        }
    }

    fn fold_constant(&mut self, basic_block: &Range<usize>) {}

    fn fuse_compare_branch(&mut self, basic_block: &Range<usize>) {
        let instructions = &mut self.instructions[basic_block.start..basic_block.end];

        for index in 1..instructions.len() {
            match instructions[index] {
                Instruction::JumpIfTrue { src, offset } => {
                    let instruction = match instructions[index - 1] {
                        Instruction::Less { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLess { src1, src2, offset })
                        }
                        Instruction::LessK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLessK { src1, src2, offset })
                        }
                        Instruction::LessEqual { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLessEqual { src1, src2, offset })
                        }
                        Instruction::LessEqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLessEqualK { src1, src2, offset })
                        }
                        Instruction::Greater { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreater { src1, src2, offset })
                        }
                        Instruction::GreaterK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreaterK { src1, src2, offset })
                        }
                        Instruction::GreaterEqual { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreaterEqual { src1, src2, offset })
                        }
                        Instruction::GreaterEqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreaterEqualK { src1, src2, offset })
                        }
                        Instruction::Equal { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfEqual { src1, src2, offset })
                        }
                        Instruction::EqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfEqualK { src1, src2, offset })
                        }
                        Instruction::NotEqual { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfNotEqual { src1, src2, offset })
                        }
                        Instruction::NotEqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfNotEqualK { src1, src2, offset })
                        }
                        _ => None,
                    };

                    if let Some(instruction) = instruction {
                        instructions[index - 1] = Instruction::Nop;
                        instructions[index] = instruction;
                    }
                }

                Instruction::JumpIfFalse { src, offset } => {
                    let instruction = match instructions[index - 1] {
                        Instruction::Less { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreaterEqual { src1, src2, offset })
                        }
                        Instruction::LessK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreaterEqualK { src1, src2, offset })
                        }
                        Instruction::LessEqual { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreater { src1, src2, offset })
                        }
                        Instruction::LessEqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfGreaterK { src1, src2, offset })
                        }
                        Instruction::Greater { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLessEqual { src1, src2, offset })
                        }
                        Instruction::GreaterK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLessEqualK { src1, src2, offset })
                        }
                        Instruction::GreaterEqual { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLess { src1, src2, offset })
                        }
                        Instruction::GreaterEqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfLessK { src1, src2, offset })
                        }
                        Instruction::Equal { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfNotEqual { src1, src2, offset })
                        }
                        Instruction::EqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfNotEqualK { src1, src2, offset })
                        }
                        Instruction::NotEqual { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfEqual { src1, src2, offset })
                        }
                        Instruction::NotEqualK { dest, src1, src2 } if dest == src => {
                            Some(Instruction::JumpIfEqualK { src1, src2, offset })
                        }
                        _ => None,
                    };

                    if let Some(instruction) = instruction {
                        instructions[index - 1] = Instruction::Nop;
                        instructions[index] = instruction;
                    }
                }

                _ => {}
            }
        }
    }

    fn eliminate_nops(&mut self) {
        let mut instructions_map = vec![0usize; self.instructions.len()];

        let mut index = 0;

        for i in 0..self.instructions.len() {
            if let Instruction::Nop = self.instructions[i] {
                instructions_map[i] = index;
            } else {
                instructions_map[i] = index;
                index += 1;
            }
        }

        let mut index = 0;

        for i in 0..self.instructions.len() {
            match &mut self.instructions[i] {
                Instruction::Jump { offset }
                | Instruction::JumpIfFalse { offset, .. }
                | Instruction::JumpIfTrue { offset, .. }
                | Instruction::JumpIfLess { offset, .. }
                | Instruction::JumpIfLessK { offset, .. }
                | Instruction::JumpIfLessEqual { offset, .. }
                | Instruction::JumpIfLessEqualK { offset, .. }
                | Instruction::JumpIfGreater { offset, .. }
                | Instruction::JumpIfGreaterK { offset, .. }
                | Instruction::JumpIfGreaterEqual { offset, .. }
                | Instruction::JumpIfGreaterEqualK { offset, .. }
                | Instruction::JumpIfEqual { offset, .. }
                | Instruction::JumpIfEqualK { offset, .. }
                | Instruction::JumpIfNotEqual { offset, .. }
                | Instruction::JumpIfNotEqualK { offset, .. } => {
                    let target = (i as i32 + *offset) as usize;
                    let target = instructions_map[target];
                    *offset = target as i32 - index as i32;
                    self.instructions[index] = self.instructions[i];
                    index += 1;
                }

                Instruction::Nop => {}

                _ => {
                    self.instructions[index] = self.instructions[i];
                    index += 1;
                }
            }
        }

        self.instructions.truncate(index);
    }
}
