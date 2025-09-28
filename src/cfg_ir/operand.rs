pub enum Operand {
    Register(Register),
    Variable(Variable),
}

#[derive(Debug, Clone, Copy)]
pub struct Register(pub u8);

impl Register {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Variable(pub usize);

impl Variable {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}
