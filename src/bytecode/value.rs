use super::instruction::Instruction;

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
    pub fn instruction(ptr: *const Instruction) -> Self {
        Self(ptr.expose_provenance() as u64)
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
    pub fn as_instruction(&self) -> *const Instruction {
        core::ptr::with_exposed_provenance(self.0 as usize)
    }
}

pub struct Function {
    index: usize,
    frame_size: u8,
}
