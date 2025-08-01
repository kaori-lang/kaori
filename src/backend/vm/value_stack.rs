use crate::frontend::codegen::value::Value;

pub struct ValueStack {
    index: usize,
    values: [Value; 1024],
}

impl ValueStack {
    pub fn new() -> Self {
        Self {
            index: 0,
            values: [Value::default(); 1024],
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values[self.index] = value;

        self.index += 1;
    }

    pub fn pop(&mut self) -> Value {
        let value = self.values[self.index];

        self.index -= 1;

        value
    }
}
