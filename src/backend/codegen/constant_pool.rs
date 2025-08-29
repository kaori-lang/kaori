use std::collections::HashMap;

use crate::backend::vm::value::Value;

#[derive(Default, Debug)]
pub struct ConstantPool {
    pub constants: Vec<Value>,
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
}
