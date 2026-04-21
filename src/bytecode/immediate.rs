use std::fmt;

const FLAG: u16 = 1 << 15;
const MASK: u16 = !FLAG;
const SCALE: f64 = 256.0;

#[derive(Clone, Copy)]
pub struct Imm(pub u16);

impl Imm {
    #[inline(always)]
    pub fn decode(self) -> f64 {
        if self.0 & (1 << 15) != 0 {
            (self.0 & !(1 << 15)) as f64 / 256.0
        } else {
            self.0 as f64
        }
    }

    #[inline(always)]
    pub fn try_from_f64(value: f64) -> Option<Self> {
        let scaled = (value * 256.0).round();
        if (0.0..=32767.0).contains(&scaled) {
            let as_u16 = scaled as u16;
            if (as_u16 as f64) / 256.0 == value {
                return Some(Self(as_u16 | (1 << 15)));
            }
        }

        None
    }
}

impl fmt::Display for Imm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.decode();

        // Optional: print integers without ".0"
        if value.fract() == 0.0 {
            write!(f, "{}", value as i64)
        } else {
            write!(f, "{}", value)
        }
    }
}
