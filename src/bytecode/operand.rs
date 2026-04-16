#[derive(Clone, Copy)]
pub enum Operand {
    Constant(u8),
    Register(u8),
}

impl Operand {
    pub fn unwrap_register(self) -> u8 {
        if let Operand::Register(value) = self {
            value
        } else {
            unreachable!("Expected a register to be unwrapped")
        }
    }

    pub fn unwrap_constant(self) -> u8 {
        if let Operand::Constant(value) = self {
            value
        } else {
            unreachable!("Expected a constant to be unwrapped")
        }
    }
}
