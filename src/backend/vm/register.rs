use super::value::Value;

pub struct Register {
    registers: [Value; 1024],
}

impl Register {
    pub fn load_local(&self, offset: usize) -> &Value {
        unsafe { self.registers.get_unchecked(offset) }
    }

    pub fn store_local(&mut self, value: Value, offset: usize) {
        unsafe { *self.registers.get_unchecked_mut(offset) = value }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            registers: [Value::default(); 1024],
        }
    }
}
