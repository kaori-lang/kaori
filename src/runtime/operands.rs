use std::fmt;

use crate::codegen::environment::Register;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Const(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reg(pub u8);

impl From<Register> for Reg {
    fn from(register: Register) -> Self {
        Reg(match register {
            Register::Local(register) => register as u8,
            Register::Temp(register) => register as u8,
        })
    }
}

impl From<usize> for Reg {
    fn from(value: usize) -> Self {
        Reg(value as u8)
    }
}

impl From<usize> for Const {
    fn from(value: usize) -> Self {
        Const(value as u16)
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}", self.0)
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}
