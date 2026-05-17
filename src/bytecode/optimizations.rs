use crate::bytecode::{function::Function, instruction::Instruction};

impl Function {
    pub fn run_optimization_passes(&mut self) {
        let basic_blocks = self.build_cfg();

        Self::eliminate_dead_code(&mut self.instructions);

        for basic_block in basic_blocks {
            self.coalesce_copies(basic_block);
            self.fuse_compare_branch(basic_block);
        }

        Self::eliminate_nops(&mut self.instructions);
    }

    fn build_cfg(&self) -> Vec<(usize, usize)> {
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
                basic_blocks.push((start, end));
                start = end;
            }
        }

        let end = self.instructions.len() - 1;
        basic_blocks.push((start, end));

        basic_blocks
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

    fn coalesce_copies(&mut self, basic_block: (usize, usize)) {
        let (start, end) = basic_block;
        let instructions = &mut self.instructions[start..end];

        for i in 0..instructions.len() {
            let (move_dest, src) = match instructions[i] {
                Instruction::Move { dest, src } | Instruction::MoveArg { dest, src } => (dest, src),
                _ => continue,
            };

            for j in (0..i).rev() {
                match &mut instructions[j] {
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
                    | Instruction::Call { dest, .. }
                        if *dest == src =>
                    {
                        *dest = move_dest;
                        instructions[i] = Instruction::Nop;

                        break;
                    }
                    Instruction::Nop => {}
                    _ => {
                        break;
                    }
                }
            }
        }
    }

    fn fuse_compare_branch(&mut self, basic_block: (usize, usize)) {
        let (start, end) = basic_block;
        let instructions = &mut self.instructions[start..end];

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

    fn eliminate_nops(instructions: &mut Vec<Instruction>) {
        let mut instructions_map = vec![0usize; instructions.len()];

        let mut index = 0;

        for i in 0..instructions.len() {
            if let Instruction::Nop = instructions[i] {
                instructions_map[i] = index;
            } else {
                instructions_map[i] = index;
                index += 1;
            }
        }

        let mut index = 0;

        for i in 0..instructions.len() {
            match &mut instructions[i] {
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
                    instructions[index] = instructions[i];
                    index += 1;
                }

                Instruction::Nop => {}

                _ => {
                    instructions[index] = instructions[i];
                    index += 1;
                }
            }
        }

        instructions.truncate(index);
    }
}
