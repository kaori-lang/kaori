use core::fmt;
use std::fmt::{Display, Formatter};

use super::function::Function;
use crate::bytecode::op_code::Opcode;

pub struct Bytecode {
    pub bytes: Vec<i16>,
    pub functions: Vec<Function>,
}

impl Bytecode {
    pub fn new(bytes: Vec<i16>, functions: Vec<Function>) -> Self {
        Self { bytes, functions }
    }
}

fn fmt_operand(op: i16) -> String {
    if op >= 0 {
        format!("r{}", op)
    } else {
        format!("k{}", op)
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut index = 0;

        while index < self.bytes.len() {
            let opcode = Opcode::from(self.bytes[index] as u16);

            index += 1;

            match opcode {
                Opcode::Add
                | Opcode::Subtract
                | Opcode::Multiply
                | Opcode::Divide
                | Opcode::Modulo
                | Opcode::Equal
                | Opcode::NotEqual
                | Opcode::Greater
                | Opcode::GreaterEqual
                | Opcode::Less
                | Opcode::LessEqual => {
                    let dst = self.bytes[index];
                    let lhs = self.bytes[index + 1];
                    let rhs = self.bytes[index + 2];

                    writeln!(
                        f,
                        "{:?} {}, {}, {}",
                        opcode,
                        fmt_operand(dst),
                        fmt_operand(lhs),
                        fmt_operand(rhs)
                    )?;

                    index += 3;
                }

                Opcode::Negate | Opcode::Not => {
                    let dst = self.bytes[index];
                    let src = self.bytes[index + 1];

                    writeln!(f, "{:?} {}, {}", opcode, fmt_operand(dst), fmt_operand(src))?;

                    index += 2;
                }

                Opcode::Move => {
                    let dst = self.bytes[index];
                    let src = self.bytes[index + 1];

                    writeln!(f, "{:?} {}, {}", opcode, fmt_operand(dst), fmt_operand(src))?;

                    index += 2;
                }

                Opcode::Call => {
                    let dst = self.bytes[index];
                    let func = self.bytes[index + 1];

                    writeln!(
                        f,
                        "{:?} {}, {}",
                        opcode,
                        fmt_operand(dst),
                        fmt_operand(func)
                    )?;

                    index += 2;
                }

                Opcode::Return => {
                    let value = self.bytes[index];
                    writeln!(f, "{:?} {}", opcode, fmt_operand(value))?;
                    index += 1;
                }

                Opcode::ReturnVoid => {
                    writeln!(f, "{:?}", opcode)?;
                }

                Opcode::Jump => {
                    let offset = self.bytes[index];
                    writeln!(f, "{:?} {}", opcode, offset)?;
                    index += 1;
                }

                Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                    let cond = self.bytes[index];
                    let offset = self.bytes[index + 1];

                    writeln!(f, "{:?} {}, {}", opcode, fmt_operand(cond), offset)?;

                    index += 2;
                }

                Opcode::Print => {
                    let value = self.bytes[index];
                    writeln!(f, "{:?} {}", opcode, fmt_operand(value))?;
                    index += 1;
                }

                Opcode::Halt => {
                    writeln!(f, "Halt")?;
                }
            }
        }

        Ok(())
    }
}
