use std::ptr;

use super::value::Value;

pub struct ValueStack {
    index: usize,
    values: [Value; 1024],
}

impl ValueStack {
    pub fn push(&mut self, value: Value) {
        unsafe {
            *self.values.get_unchecked_mut(self.index) = value;
        }

        self.index += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.index -= 1;
        unsafe { self.values.get_unchecked(self.index).clone() }
    }
}

impl Default for ValueStack {
    fn default() -> Self {
        Self {
            index: 0,
            values: [(); 1024].map(|_| Value::default()),
        }
    }
}
