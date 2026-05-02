use crate::bytecode::{function::Function, instruction::Instruction};

pub fn optimize_bytecode(functions: &mut [Function]) {
    for function in functions {
        let (reachable, leaders) = reachable_and_leaders(&function.instructions);
        eliminate_dead_code(&mut function.instructions, &reachable);
        remove_redundant_moves(&mut function.instructions, &leaders);
        merge_conditional_jumps(&mut function.instructions);
        remove_nop(&mut function.instructions);
    }
}

fn eliminate_dead_code(instructions: &mut [Instruction], reachable: &[bool]) {
    for i in 0..instructions.len() {
        if !reachable[i] {
            instructions[i] = Instruction::Nop;
        }
    }
}

fn reachable_and_leaders(instructions: &[Instruction]) -> (Vec<bool>, Vec<bool>) {
    let mut reachable = vec![false; instructions.len()];
    let mut leaders = vec![false; instructions.len()];
    let mut stack = vec![0usize];

    leaders[0] = true;

    while let Some(index) = stack.pop() {
        if reachable[index] {
            continue;
        }
        reachable[index] = true;

        match instructions[index] {
            Instruction::Jump { offset } => {
                let target = ((index as i32 + offset) as usize).clamp(0, instructions.len() - 1);
                leaders[target] = true;
                stack.push(target);
            }
            Instruction::JumpIfFalse { offset, .. } | Instruction::JumpIfTrue { offset, .. } => {
                let target = ((index as i32 + offset) as usize).clamp(0, instructions.len() - 1);
                leaders[target] = true;
                stack.push(target);
                stack.push(index + 1);
            }
            Instruction::Return { .. } => {
                if index + 1 < instructions.len() {
                    leaders[index + 1] = true;
                }
            }
            _ => stack.push(index + 1),
        }
    }

    (reachable, leaders)
}

fn remove_redundant_moves(instructions: &mut [Instruction], leaders: &[bool]) {
    let mut leader = 0;

    for index in 0..instructions.len() {
        let (move_dest, src) = match instructions[index] {
            Instruction::Move { dest, src } => (dest, src),
            _ => continue,
        };

        if leaders[index] {
            leader = index;
        };

        let mut i = index;

        while i > leader {
            i -= 1;

            match &mut instructions[i] {
                Instruction::Add { dest, .. }
                | Instruction::AddI { dest, .. }
                | Instruction::Subtract { dest, .. }
                | Instruction::SubtractRI { dest, .. }
                | Instruction::SubtractIR { dest, .. }
                | Instruction::Multiply { dest, .. }
                | Instruction::MultiplyI { dest, .. }
                | Instruction::Divide { dest, .. }
                | Instruction::DivideRI { dest, .. }
                | Instruction::DivideIR { dest, .. }
                | Instruction::Modulo { dest, .. }
                | Instruction::ModuloRI { dest, .. }
                | Instruction::ModuloIR { dest, .. }
                | Instruction::Equal { dest, .. }
                | Instruction::EqualI { dest, .. }
                | Instruction::NotEqual { dest, .. }
                | Instruction::NotEqualI { dest, .. }
                | Instruction::Less { dest, .. }
                | Instruction::LessI { dest, .. }
                | Instruction::LessEqual { dest, .. }
                | Instruction::LessEqualI { dest, .. }
                | Instruction::Greater { dest, .. }
                | Instruction::GreaterI { dest, .. }
                | Instruction::GreaterEqual { dest, .. }
                | Instruction::GreaterEqualI { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::Negate { dest, .. }
                | Instruction::MoveArg { dest, .. }
                | Instruction::Move { dest, .. }
                | Instruction::LoadK { dest, .. }
                | Instruction::LoadImm { dest, .. }
                | Instruction::CreateDict { dest }
                | Instruction::GetField { dest, .. }
                | Instruction::Call { dest, .. }
                | Instruction::CallK { dest, .. }
                    if *dest == src =>
                {
                    *dest = move_dest;
                    instructions[index] = Instruction::Nop;

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

fn merge_conditional_jumps(instructions: &mut [Instruction]) {
    for index in 1..instructions.len() {
        match instructions[index] {
            Instruction::JumpIfTrue { src, offset } => {
                let instruction = match instructions[index - 1] {
                    Instruction::Less { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLess { src1, src2, offset })
                    }
                    Instruction::LessI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLessI { src1, src2, offset })
                    }
                    Instruction::LessEqual { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLessEqual { src1, src2, offset })
                    }
                    Instruction::LessEqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLessEqualI { src1, src2, offset })
                    }
                    Instruction::Greater { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreater { src1, src2, offset })
                    }
                    Instruction::GreaterI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreaterI { src1, src2, offset })
                    }
                    Instruction::GreaterEqual { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreaterEqual { src1, src2, offset })
                    }
                    Instruction::GreaterEqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreaterEqualI { src1, src2, offset })
                    }
                    Instruction::Equal { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfEqual { src1, src2, offset })
                    }
                    Instruction::EqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfEqualI { src1, src2, offset })
                    }
                    Instruction::NotEqual { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfNotEqual { src1, src2, offset })
                    }
                    Instruction::NotEqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfNotEqualI { src1, src2, offset })
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
                    Instruction::LessI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreaterEqualI { src1, src2, offset })
                    }
                    Instruction::LessEqual { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreater { src1, src2, offset })
                    }
                    Instruction::LessEqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfGreaterI { src1, src2, offset })
                    }
                    Instruction::Greater { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLessEqual { src1, src2, offset })
                    }
                    Instruction::GreaterI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLessEqualI { src1, src2, offset })
                    }
                    Instruction::GreaterEqual { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLess { src1, src2, offset })
                    }
                    Instruction::GreaterEqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfLessI { src1, src2, offset })
                    }
                    Instruction::Equal { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfNotEqual { src1, src2, offset })
                    }
                    Instruction::EqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfNotEqualI { src1, src2, offset })
                    }
                    Instruction::NotEqual { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfEqual { src1, src2, offset })
                    }
                    Instruction::NotEqualI { dest, src1, src2 } if dest == src => {
                        Some(Instruction::JumpIfEqualI { src1, src2, offset })
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

fn remove_nop(instructions: &mut Vec<Instruction>) {
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
            | Instruction::JumpIfLessI { offset, .. }
            | Instruction::JumpIfLessEqual { offset, .. }
            | Instruction::JumpIfLessEqualI { offset, .. }
            | Instruction::JumpIfGreater { offset, .. }
            | Instruction::JumpIfGreaterI { offset, .. }
            | Instruction::JumpIfGreaterEqual { offset, .. }
            | Instruction::JumpIfGreaterEqualI { offset, .. }
            | Instruction::JumpIfEqual { offset, .. }
            | Instruction::JumpIfEqualI { offset, .. }
            | Instruction::JumpIfNotEqual { offset, .. }
            | Instruction::JumpIfNotEqualI { offset, .. } => {
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
