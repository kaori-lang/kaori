use core::fmt;
use std::fmt::{Display, Formatter};

use super::function::Function;
use crate::bytecode::op_code::Opcode;

pub struct Bytecode {
    pub bytes: Vec<u16>,
    pub functions: Vec<Function>,
}

impl Bytecode {
    pub fn new(bytes: Vec<u16>, functions: Vec<Function>) -> Self {
        Self { bytes, functions }
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut index = 0;

        while index < self.bytes.len() {
            let op_code = Opcode::from(self.bytes[index]);

            match op_code {
                // === Arithmetic ===
                Opcode::AddRR
                | Opcode::SubtractRR
                | Opcode::MultiplyRR
                | Opcode::DivideRR
                | Opcode::ModuloRR
                | Opcode::EqualRR
                | Opcode::NotEqualRR
                | Opcode::GreaterRR
                | Opcode::GreaterEqualRR
                | Opcode::LessRR
                | Opcode::LessEqualRR => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}, r{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2],
                        self.bytes[index + 3]
                    )?;
                    index += 4;
                }

                Opcode::AddRK
                | Opcode::SubtractRK
                | Opcode::MultiplyRK
                | Opcode::DivideRK
                | Opcode::ModuloRK
                | Opcode::EqualRK
                | Opcode::NotEqualRK
                | Opcode::GreaterRK
                | Opcode::GreaterEqualRK
                | Opcode::LessRK
                | Opcode::LessEqualRK => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}, k{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2],
                        self.bytes[index + 3]
                    )?;
                    index += 4;
                }

                Opcode::AddKR
                | Opcode::SubtractKR
                | Opcode::MultiplyKR
                | Opcode::DivideKR
                | Opcode::ModuloKR
                | Opcode::EqualKR
                | Opcode::NotEqualKR
                | Opcode::GreaterKR
                | Opcode::GreaterEqualKR
                | Opcode::LessKR
                | Opcode::LessEqualKR => {
                    writeln!(
                        f,
                        "{:?} r{}, k{}, r{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2],
                        self.bytes[index + 3]
                    )?;
                    index += 4;
                }

                Opcode::AddKK
                | Opcode::SubtractKK
                | Opcode::MultiplyKK
                | Opcode::DivideKK
                | Opcode::ModuloKK
                | Opcode::EqualKK
                | Opcode::NotEqualKK
                | Opcode::GreaterKK
                | Opcode::GreaterEqualKK
                | Opcode::LessKK
                | Opcode::LessEqualKK => {
                    writeln!(
                        f,
                        "{:?} r{}, k{}, k{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2],
                        self.bytes[index + 3]
                    )?;
                    index += 4;
                }

                // === Unary ===
                Opcode::NegateR | Opcode::NotR => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2]
                    )?;
                    index += 3;
                }

                Opcode::NegateK | Opcode::NotK => {
                    writeln!(
                        f,
                        "{:?} r{}, k{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2]
                    )?;
                    index += 3;
                }

                // === Data movement ===
                Opcode::MoveR => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2]
                    )?;
                    index += 3;
                }

                Opcode::MoveK => {
                    writeln!(
                        f,
                        "{:?} r{}, k{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2]
                    )?;
                    index += 3;
                }

                // === Function and return ===
                Opcode::CallR => {
                    writeln!(
                        f,
                        "{:?} r{}, r{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2]
                    )?;
                    index += 3;
                }

                Opcode::CallK => {
                    writeln!(
                        f,
                        "{:?} r{}, k{}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2]
                    )?;
                    index += 3;
                }

                Opcode::ReturnR => {
                    writeln!(f, "{:?} r{}", op_code, self.bytes[index + 1])?;
                    index += 2;
                }

                Opcode::ReturnK => {
                    writeln!(f, "{:?} k{}", op_code, self.bytes[index + 1])?;
                    index += 2;
                }

                Opcode::ReturnVoid => {
                    writeln!(f, "{:?}", op_code)?;
                    index += 1;
                }

                // === Control flow ===
                Opcode::Jump => {
                    writeln!(f, "{:?} {}", op_code, self.bytes[index + 1] as i16)?;
                    index += 2;
                }

                Opcode::JumpIfTrueR | Opcode::JumpIfFalseR => {
                    writeln!(
                        f,
                        "{:?} r{}, {}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2] as i16
                    )?;
                    index += 3;
                }

                Opcode::JumpIfTrueK | Opcode::JumpIfFalseK => {
                    writeln!(
                        f,
                        "{:?} k{}, {}",
                        op_code,
                        self.bytes[index + 1],
                        self.bytes[index + 2] as i16
                    )?;
                    index += 3;
                }

                // === IO ===
                Opcode::PrintR => {
                    writeln!(f, "{:?} r{}", op_code, self.bytes[index + 1])?;
                    index += 2;
                }

                Opcode::PrintK => {
                    writeln!(f, "{:?} k{}", op_code, self.bytes[index + 1])?;
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
