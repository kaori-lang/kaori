#[derive(Debug, Clone, Copy)]
pub struct Register(pub u8);

impl Register {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}
