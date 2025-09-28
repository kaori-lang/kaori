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

#[derive(Debug, Clone, Copy)]
pub struct Register(pub u8);

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(pub usize);
