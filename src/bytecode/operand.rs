#[derive(Clone, Copy)]
pub enum Operand {
    Constant(u8),
    Register(u8),
    Immediate(Immediate),
}

#[derive(Clone, Copy)]
pub enum ImmediateType {
    Float,
    Boolean,
}

#[derive(Clone, Copy)]
pub struct Immediate {
    pub ty: ImmediateType,
    pub value: u32,
}

impl Immediate {
    pub fn try_f32(value: f64) -> Option<Self> {
        if (value as f32) as f64 == value {
            Some(Self {
                ty: ImmediateType::Float,
                value: (value as f32).to_bits(),
            })
        } else {
            None
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self {
            ty: ImmediateType::Boolean,
            value: value as u32,
        }
    }
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
