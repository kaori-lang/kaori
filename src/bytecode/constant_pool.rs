use core::fmt;

use super::value::Value;

#[derive(Default)]
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

#[derive(Debug, Clone, Copy, Default)]
pub struct ConstantIndex(pub u16);

impl fmt::Display for ConstantIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "c{}", self.0)
    }
}
