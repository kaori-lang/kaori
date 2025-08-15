use super::value::Value;

pub struct Callstack {
    declarations: Vec<Value>,
    scopes_pointer: Vec<usize>,
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

    pub fn enter_scope(&mut self) {
        self.scopes_pointer.push(self.declarations.len());
    }

    pub fn exit_scope(&mut self) {
        let top = self.scopes_pointer.pop().unwrap();

        self.declarations.resize(top, Value::default());
    }
}

impl Default for Callstack {
    fn default() -> Self {
        Self {
            declarations: Vec::with_capacity(1024),
            scopes_pointer: Vec::with_capacity(64),
        }
    }
}
