#[derive(Clone, Copy)]
pub enum Operand {
    Constant(u16),
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

    pub fn unwrap_constant(self) -> u32 {
        if let Operand::Constant(value) = self {
            value as u32
        } else {
            unreachable!("Expected a constant to be unwrapped")
        }
    }
}
