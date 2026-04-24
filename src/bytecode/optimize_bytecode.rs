use crate::bytecode::{Function, instruction::Instruction};

pub fn optimize_bytecode(functions: &mut [Function]) {
    for function in functions {
        remove_redundant_move(&mut function.instructions);
        merge_conditional_jumps(&mut function.instructions);
        remove_nop(&mut function.instructions);
    }
}

fn remove_redundant_move(instructions: &mut [Instruction]) {
    for index in 0..instructions.len() {
        match instructions[index] {
            Instruction::Move {
                dest: move_dest,
                src,
            }
            | Instruction::MoveArg {
                dest: move_dest,
                src,
            } => {
                let mut i = index;

                while i > 0 {
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
                        | Instruction::Move { dest, .. }
                        | Instruction::MoveArg { dest, .. }
                        | Instruction::LoadK { dest, .. }
                        | Instruction::LoadImm { dest, .. }
                        | Instruction::CreateDict { dest }
                        | Instruction::GetField { dest, .. }
                        | Instruction::Call { dest, .. }
                        | Instruction::CallK { dest, .. } => {
                            if *dest == src {
                                *dest = move_dest;
                                instructions[index] = Instruction::Nop;
                            }

                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
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
    let mut instructions_map = vec![0; instructions.len()];

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
                let target = i as i32 + *offset;

                if target < 0 || target >= instructions_map.len() as i32 {
                    continue;
                };

                let target = instructions_map[target as usize];
                *offset = target as i32 - index as i32;

                instructions[index] = instructions[i];

                if index != i {
                    instructions[i] = Instruction::Nop;
                }

                index += 1;
            }
            Instruction::Nop => {}
            _ => {
                instructions[index] = instructions[i];
                if index != i {
                    instructions[i] = Instruction::Nop;
                }

                index += 1;
            }
        }
    }

    instructions.truncate(index);
}
