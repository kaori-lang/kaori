use std::collections::HashMap;

use crate::backend::vm::value::Value;

#[derive(Default, Debug)]
pub struct ConstantPool {
    constants: Vec<Value>,
    functions: HashMap<usize, usize>,
}

impl ConstantPool {
    pub fn load_constant(&mut self, other: Value) -> usize {
        for (index, current) in self.constants.iter().enumerate() {
            if *current == other {
                return index;
            }
        }

        let index = self.constants.len();

        self.constants.push(other);

        index
    }

    pub fn load_function_constant(&mut self, function_id: usize) -> usize {
        if let Some(index) = self.functions.get(&function_id) {
            return *index;
        };

        let index = self.constants.len();

        self.functions.insert(function_id, index);
        self.constants.push(Value::Null);

        index
    }

    pub fn define_function_constant(&mut self, function_id: usize, value: Value) {
        if let Some(index) = self.functions.get(&function_id) {
            self.constants[*index] = value;
        } else {
            let index = self.constants.len();

            self.functions.insert(function_id, index);
            self.constants.push(value);
        }
    }

    pub fn get_constant(&self, index: usize) -> Value {
        unsafe { self.constants.get_unchecked(index).to_owned() }
    }
}
