use std::{cmp::Reverse, collections::BinaryHeap};

use crate::mir::{
    function::Function,
    instruction::{Instruction, Register},
};

impl Function {
    pub fn run_optimization_passes(&mut self) {
        let registers_map = self.allocate_registers();
        self.rewrite_instructions(&registers_map);

        //Self::eliminate_dead_code(&mut self.instructions);

        let frame_size = registers_map.iter().copied().max().unwrap_or(0);

        //self.eliminate_nops();

        println!("{}", self);
    }

    fn allocate_registers(&self) -> Vec<usize> {
        let mut sorted_ranges = Vec::new();

        for (register, range) in self.live_ranges.iter() {
            sorted_ranges.push((*register, range.start..range.end));
        }

        sorted_ranges.sort_by_key(|(register, range)| (range.start, *register));

        let mut registers_map = vec![0usize; self.next_register];
        let mut active_registers: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
        let mut next_register = 0usize;

        for (register, range) in sorted_ranges {
            match active_registers.peek().copied() {
                Some(Reverse((end, active_register))) if range.start >= end => {
                    active_registers.pop();
                    active_registers.push(Reverse((range.end, active_register)));
                    registers_map[register.0 as usize] = active_register;
                }
                _ => {
                    let active_register = next_register;
                    next_register += 1;
                    active_registers.push(Reverse((range.end, active_register)));
                    registers_map[register.0 as usize] = active_register;
                }
            }
        }

        registers_map
    }

    fn rewrite_instructions(&mut self, registers_map: &[usize]) {
        let replace = |register: Register| {
            let register = if register.0 >= 0 {
                registers_map[register.0 as usize] as i16
            } else {
                register.0
            };

            Register(register)
        };

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
                other => other,
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

    fn eliminate_nops(&mut self) {
        let mut instructions_map = vec![0usize; self.instructions.len()];
        let mut index = 0;

        for (i, instruction) in self.instructions.iter().enumerate() {
            instructions_map[i] = index;
            if !matches!(instruction, Instruction::Nop) {
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
                    let new_target = instructions_map[target];
                    *offset = new_target as i32 - index as i32;
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
