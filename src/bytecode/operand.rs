#[derive(Clone, Copy)]
pub enum Operand {
    Constant(u8),
    Register(u8),
    Immediate(u16),
}

impl Operand {
    pub fn try_f64(value: f64) -> Option<Self> {
        let scaled = (value * 256.0).round();
        if (0.0..=32767.0).contains(&scaled) {
            let as_u16 = scaled as u16;
            if (as_u16 as f64) / 256.0 == value {
                return Some(Self::Immediate(as_u16 | (1 << 15)));
            }
        }

        None
    }

    pub fn boolean(value: bool) -> Self {
        Self::Immediate(value as u16)
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
