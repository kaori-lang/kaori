use super::value::Value;

pub struct ValueStack {
    index: usize,
    values: [Value; 1024],
}

impl ValueStack {
    pub fn push(&mut self, value: Value) {
        self.values[self.index] = value;

        self.index += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.index -= 1;

        self.values[self.index]
    }
}

impl Default for ValueStack {
    fn default() -> Self {
        Self {
            index: 0,
            values: [Value::default(); 1024],
        }
    }
}
