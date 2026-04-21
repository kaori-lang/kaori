use std::fmt;

#[derive(Clone, Copy)]
pub struct Imm(pub u16);

impl Imm {
    #[inline(always)]
    pub fn decode(self) -> f64 {
        self.0 as f64
    }

    #[inline(always)]
    pub fn try_to_encode(value: f64) -> Option<Self> {
        let as_u16 = value as u16;
        if as_u16 as f64 == value && value >= 0.0 {
            Some(Self(as_u16))
        } else {
            None
        }
    }
}
impl fmt::Display for Imm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.decode())
    }
}
