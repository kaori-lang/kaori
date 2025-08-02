use super::value::Value;

pub struct Callstack {
    index: usize,
    declarations: [Value; 1024],
    scopes_pointer: Vec<usize>,
}

impl Callstack {
    pub fn declare(&mut self, value: Value) {
        self.declarations[self.index] = value;

        self.index += 1;
    }

    pub fn load_global(&self, offset: usize) -> Value {
        unsafe { *self.declarations.get_unchecked(offset) }
    }

    pub fn store_global(&mut self, value: Value, offset: usize) {
        unsafe { *self.declarations.get_unchecked_mut(offset) = value }
    }

    pub fn enter_scope(&mut self) {
        self.scopes_pointer.push(self.index);
    }

    pub fn exit_scope(&mut self) {
        let top = self.scopes_pointer.pop().unwrap();

        self.index = top;
    }
}

impl Default for Callstack {
    fn default() -> Self {
        Self {
            index: 0,
            declarations: [Value::default(); 1024],
            scopes_pointer: Vec::new(),
        }
    }
}
