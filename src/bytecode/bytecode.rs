use core::fmt;
use std::fmt::{Display, Formatter};

use crate::bytecode::op_code::Opcode;

use super::function::Function;

pub struct Bytecode {
    pub instructions: Vec<u16>,
    pub functions: Vec<Function>,
}

impl Bytecode {
    pub fn new(instructions: Vec<u16>, functions: Vec<Function>) -> Self {
        Self {
            instructions,
            functions,
        }
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Bytecode ===")?;

        let mut index = 0;
        while index < self.instructions.len() {
            let op_code = Opcode::from(self.instructions[index]);

            match &op_code {
                // === Arithmetic ===
                Opcode::AddRR
                | Opcode::AddRK
                | Opcode::AddKR
                | Opcode::AddKK
                | Opcode::SubtractRR
                | Opcode::SubtractRK
                | Opcode::SubtractKR
                | Opcode::SubtractKK
                | Opcode::MultiplyRR
                | Opcode::MultiplyRK
                | Opcode::MultiplyKR
                | Opcode::MultiplyKK
                | Opcode::DivideRR
                | Opcode::DivideRK
                | Opcode::DivideKR
                | Opcode::DivideKK
                | Opcode::ModuloRR
                | Opcode::ModuloRK
                | Opcode::ModuloKR
                | Opcode::ModuloKK
                | Opcode::EqualRR
                | Opcode::EqualRK
                | Opcode::EqualKR
                | Opcode::EqualKK
                | Opcode::NotEqualRR
                | Opcode::NotEqualRK
                | Opcode::NotEqualKR
                | Opcode::NotEqualKK
                | Opcode::GreaterRR
                | Opcode::GreaterRK
                | Opcode::GreaterKR
                | Opcode::GreaterKK
                | Opcode::GreaterEqualRR
                | Opcode::GreaterEqualRK
                | Opcode::GreaterEqualKR
                | Opcode::GreaterEqualKK
                | Opcode::LessRR
                | Opcode::LessRK
                | Opcode::LessKR
                | Opcode::LessKK
                | Opcode::LessEqualRR
                | Opcode::LessEqualRK
                | Opcode::LessEqualKR
                | Opcode::LessEqualKK => {
                    writeln!(
                        f,
                        "{:?} r{}, {}, {}",
                        op_code,
                        self.instructions[index + 1],
                        self.instructions[index + 2],
                        self.instructions[index + 3]
                    )?;
                    index += 4;
                }
                Opcode::NegateR | Opcode::NegateK | Opcode::NotR | Opcode::NotK => {
                    writeln!(
                        f,
                        "{:?} r{}, {}",
                        op_code,
                        self.instructions[index + 1],
                        self.instructions[index + 2]
                    )?;
                    index += 3;
                }

                // === Data movement ===
                Opcode::MoveR | Opcode::MoveK => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}",
                        op_code,
                        self.instructions[index + 1],
                        self.instructions[index + 2]
                    )?;
                    index += 3;
                }

                // === Function and return ===
                Opcode::CallR | Opcode::CallK => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}",
                        op_code,
                        self.instructions[index + 1],
                        self.instructions[index + 2]
                    )?;
                    index += 3;
                }

                Opcode::ReturnR | Opcode::ReturnK => {
                    writeln!(f, "{:?} r{}", op_code, self.instructions[index + 1])?;
                    index += 2;
                }

                Opcode::ReturnVoid => {
                    writeln!(f, "{:?}", op_code)?;
                    index += 1;
                }

                // === Control flow ===
                Opcode::Jump => {
                    writeln!(f, "{:?} {}", op_code, self.instructions[index + 1])?;
                    index += 2;
                }

                Opcode::JumpIfTrueR
                | Opcode::JumpIfTrueK
                | Opcode::JumpIfFalseR
                | Opcode::JumpIfFalseK => {
                    writeln!(
                        f,
                        "{:?} r{}, {}",
                        op_code,
                        self.instructions[index + 1],
                        self.instructions[index + 2]
                    )?;
                    index += 3;
                }

                // === IO ===
                Opcode::PrintR | Opcode::PrintK => {
                    writeln!(f, "{:?} r{}", op_code, self.instructions[index + 1])?;
                    index += 2;
                }

                // === Program termination ===
                Opcode::Halt => {
                    writeln!(f, "Halt")?;
                    index += 1;
                }
            }
        }

        Ok(())
    }
}
