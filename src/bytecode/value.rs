#[derive(Default, Clone, Copy, Debug)]
pub struct Value(pub u64);

impl Value {
    #[inline(always)]
    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }

    #[inline(always)]
    pub fn boolean(value: bool) -> Self {
        Self(value as u64)
    }

    #[inline(always)]
    pub fn function(value: usize) -> Self {
        Self(value as u64)
    }

    #[inline(always)]
    pub fn as_number(&self) -> f64 {
        f64::from_bits(self.0)
    }

    #[inline(always)]
    pub fn as_boolean(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub fn as_function(&self) -> usize {
        self.0 as usize
    }
}
