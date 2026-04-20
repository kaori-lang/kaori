#[derive(Clone, Copy)]
pub enum Operand {
    Constant(u8),
    Register(u8),
    Immediate(u32),
}

impl Operand {
    pub fn try_f32(value: f64) -> Option<Self> {
        if (value as f32) as f64 == value {
            Some(Self::Immediate((value as f32).to_bits()))
        } else {
            None
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self::Immediate(value as u32)
    }

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
