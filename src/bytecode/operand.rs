#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Register(u8),
    BoolImm(bool),
    NilImm,
    NumberImm(u16),
}

impl Operand {
    const SCALE: f64 = 256.0;

    pub fn from_bool(value: bool) -> Self {
        Self::BoolImm(value)
    }

    pub fn nil() -> Self {
        Self::NilImm
    }

    #[inline(always)]
    pub fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let scaled = value * Self::SCALE;
        if scaled < i16::MIN as f64 || scaled > i16::MAX as f64 {
            return None;
        }
        let fixed = scaled.round() as i16;
        Some(Self::NumberImm(fixed as u16))
    }

    pub fn unwrap_register(self) -> u8 {
        if let Operand::Register(value) = self {
            value
        } else {
            unreachable!("Expected a register to be unwrapped")
        }
    }

    pub fn unwrap_number(self) -> f64 {
        if let Operand::NumberImm(bits) = self {
            bits as i16 as f64 / Self::SCALE
        } else {
            unreachable!("Expected a number to be unwrapped")
        }
    }

    pub fn unwrap_bool(self) -> bool {
        if let Operand::BoolImm(value) = self {
            value
        } else {
            unreachable!("Expected a bool to be unwrapped")
        }
    }

    pub fn operand_to_imm(&self) -> u16 {
        match *self {
            Operand::NumberImm(bits) => bits,
            Operand::BoolImm(true) => 1,
            Operand::BoolImm(false) => 0,
            Operand::NilImm => u16::MAX,
            Operand::Register(_) => unreachable!("operand_to_imm called with a register"),
        }
    }

    pub fn is_nil(self) -> bool {
        matches!(self, Operand::NilImm)
    }
}
