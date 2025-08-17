use super::value::Value;

pub struct Callstack {
    declarations: [Value; 1024],
}

impl Callstack {
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
            declarations: [Value::default(); 1024],
        }
    }
}
