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
    pub fn to_register(self) -> Register {
        match self {
            Self::Register(register) => register,
            Self::Variable(Variable(value)) => Register(value as u8),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Register(pub u8);

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(pub usize);
