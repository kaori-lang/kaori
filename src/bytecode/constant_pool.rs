use std::u16;

use super::value::Value;

#[derive(Debug, Clone, Copy, Default)]
pub struct ConstantIndex(u16);

pub struct ConstantPool {
    pub values: Vec<Value>,
}

impl ConstantPool {
    pub fn insert_value(&mut self, value: Value) -> ConstantIndex {
        let constant_index = ConstantIndex(self.values.len() as u16);

        self.values.push(value);

        constant_index
    }

    pub fn get_value(&self, constant_index: ConstantIndex) -> Value {
        self.values[usize::from(constant_index.0)]
    }
}
