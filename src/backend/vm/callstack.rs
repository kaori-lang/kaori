use super::value::Value;

pub struct Callstack {
    declarations: Vec<Value>,
}

impl Callstack {
    pub fn declare(&mut self, value: Value) {
        self.declarations.push(value);
    }

    pub fn load_local(&self, offset: usize) -> &Value {
        unsafe { self.declarations.get_unchecked(offset) }
    }

    pub fn store_local(&mut self, value: Value, offset: usize) {
        unsafe { *self.declarations.get_unchecked_mut(offset) = value }
    }
}

impl Default for Callstack {
    fn default() -> Self {
        Self {
            declarations: Vec::with_capacity(100),
        }
    }
}
