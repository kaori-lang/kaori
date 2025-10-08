use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(Register),
    Variable(Variable),
}

impl From<Register> for Operand {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Variable> for Operand {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl Operand {
    pub fn to_register(self) -> i16 {
        match self {
            Self::Register(register) => register.0,
            Self::Variable(Variable(value)) => value as i16,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(pub isize);

#[derive(Debug, Clone, Copy, Default)]
pub struct Register(pub i16);

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            write!(f, "c{}", -self.0)
        } else {
            write!(f, "r{}", self.0)
        }
    }
}
