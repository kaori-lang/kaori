use std::{cmp::Reverse, collections::BinaryHeap};

use crate::mir::{
    function::Function,
    instruction::{Instruction, Register},
};

use crate::mir::instruction as mir_instruction;
use crate::runtime::function as runtime_function;
use crate::runtime::instruction as runtime_instruction;

impl Function {
    pub fn run_optimization_passes(self) -> runtime_function::Function {
        let registers_map = self.allocate_registers();

        //Self::eliminate_dead_code(&mut self.instructions);

        let max_register: u8 = match registers_map.iter().copied().max().unwrap_or(0).try_into() {
            Ok(value) => value,
            Err(err) => panic!("Max registers per function exceeded!"),
        };
        let frame_size = max_register + 1;
        //self.eliminate_nops();

        self.into_runtime_function(&registers_map, frame_size)
    }

    fn into_runtime_function(
        self,
        registers_map: &[usize],
        frame_size: u8,
    ) -> runtime_function::Function {
        let Function {
            instructions,
            constants,
            arity,
            ..
        } = self;

        fn map_register(
            register: Register,
            map: &[usize],
            frame_size: u8,
        ) -> runtime_instruction::Register {
            let value = if register.0 >= 0 {
                map[register.0 as usize] as u8
            } else {
                frame_size + (-register.0 - 1) as u8
            };

            runtime_instruction::Register(value)
        }

        let instructions = instructions
            .into_iter()
            .map(|instruction| match instruction {
                mir_instruction::Instruction::Add { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Add {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::AddK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::AddK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::Subtract { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Subtract {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::SubtractRK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::SubtractRK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::SubtractKR { dest, src1, src2 } => {
                    runtime_instruction::Instruction::SubtractKR {
                        dest: map_register(dest, registers_map, frame_size),
                        src1,
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Multiply { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Multiply {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::MultiplyK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::MultiplyK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::Divide { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Divide {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::DivideRK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::DivideRK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::DivideKR { dest, src1, src2 } => {
                    runtime_instruction::Instruction::DivideKR {
                        dest: map_register(dest, registers_map, frame_size),
                        src1,
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Modulo { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Modulo {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::ModuloRK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::ModuloRK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::ModuloKR { dest, src1, src2 } => {
                    runtime_instruction::Instruction::ModuloKR {
                        dest: map_register(dest, registers_map, frame_size),
                        src1,
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Equal { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Equal {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::EqualK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::EqualK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::NotEqual { dest, src1, src2 } => {
                    runtime_instruction::Instruction::NotEqual {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::NotEqualK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::NotEqualK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::Less { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Less {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::LessK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::LessK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::LessEqual { dest, src1, src2 } => {
                    runtime_instruction::Instruction::LessEqual {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::LessEqualK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::LessEqualK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::Greater { dest, src1, src2 } => {
                    runtime_instruction::Instruction::Greater {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::GreaterK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::GreaterK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::GreaterEqual { dest, src1, src2 } => {
                    runtime_instruction::Instruction::GreaterEqual {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::GreaterEqualK { dest, src1, src2 } => {
                    runtime_instruction::Instruction::GreaterEqualK {
                        dest: map_register(dest, registers_map, frame_size),
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                    }
                }

                mir_instruction::Instruction::Not { dest, src } => {
                    runtime_instruction::Instruction::Not {
                        dest: map_register(dest, registers_map, frame_size),
                        src: map_register(src, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Negate { dest, src } => {
                    runtime_instruction::Instruction::Negate {
                        dest: map_register(dest, registers_map, frame_size),
                        src: map_register(src, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Move { dest, src } => {
                    runtime_instruction::Instruction::Move {
                        dest: map_register(dest, registers_map, frame_size),
                        src: map_register(src, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::LoadK { dest, src } => {
                    runtime_instruction::Instruction::LoadK {
                        dest: map_register(dest, registers_map, frame_size),
                        src,
                    }
                }

                mir_instruction::Instruction::CreateDict { dest } => {
                    runtime_instruction::Instruction::CreateDict {
                        dest: map_register(dest, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::SetField { object, key, value } => {
                    runtime_instruction::Instruction::SetField {
                        object: map_register(object, registers_map, frame_size),
                        key: map_register(key, registers_map, frame_size),
                        value: map_register(value, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::GetField { dest, object, key } => {
                    runtime_instruction::Instruction::GetField {
                        dest: map_register(dest, registers_map, frame_size),
                        object: map_register(object, registers_map, frame_size),
                        key: map_register(key, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::CreateClosure { dest, src } => {
                    runtime_instruction::Instruction::CreateClosure {
                        dest: map_register(dest, registers_map, frame_size),
                        src,
                    }
                }

                mir_instruction::Instruction::CaptureValue { dest, src } => {
                    runtime_instruction::Instruction::CaptureValue {
                        dest: map_register(dest, registers_map, frame_size),
                        src: map_register(src, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Call { dest, src, arity } => {
                    runtime_instruction::Instruction::Call {
                        dest: map_register(dest, registers_map, frame_size),
                        src: map_register(src, registers_map, frame_size),
                        arity,
                    }
                }

                mir_instruction::Instruction::Return { src } => {
                    runtime_instruction::Instruction::Return {
                        src: map_register(src, registers_map, frame_size),
                    }
                }

                mir_instruction::Instruction::Jump { offset } => {
                    runtime_instruction::Instruction::Jump { offset }
                }

                mir_instruction::Instruction::JumpIfFalse { src, offset } => {
                    runtime_instruction::Instruction::JumpIfFalse {
                        src: map_register(src, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfTrue { src, offset } => {
                    runtime_instruction::Instruction::JumpIfTrue {
                        src: map_register(src, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfLess { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfLess {
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfLessK { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfLessK {
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfLessEqual { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfLessEqual {
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfLessEqualK { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfLessEqualK {
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfGreater { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfGreater {
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfGreaterK { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfGreaterK {
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfGreaterEqual { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfGreaterEqual {
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfGreaterEqualK { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfGreaterEqualK {
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfEqual { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfEqual {
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfEqualK { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfEqualK {
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfNotEqual { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfNotEqual {
                        src1: map_register(src1, registers_map, frame_size),
                        src2: map_register(src2, registers_map, frame_size),
                        offset,
                    }
                }

                mir_instruction::Instruction::JumpIfNotEqualK { src1, src2, offset } => {
                    runtime_instruction::Instruction::JumpIfNotEqualK {
                        src1: map_register(src1, registers_map, frame_size),
                        src2,
                        offset,
                    }
                }

                mir_instruction::Instruction::Nop => runtime_instruction::Instruction::Nop,
            })
            .collect();

        runtime_function::Function {
            instructions,
            constants,
            frame_size,
            arity,
        }
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
