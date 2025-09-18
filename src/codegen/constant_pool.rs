use std::collections::HashMap;

use crate::{backend::vm::value::Value, frontend::syntax::node_id::NodeId};

#[derive(Default, Debug)]
pub struct ConstantPool {
    pub constants: Vec<Value>,
    globals: HashMap<NodeId, usize>,
}

impl ConstantPool {
    pub fn load_const(&mut self, other: Value) -> usize {
        for (index, current) in self.constants.iter().enumerate() {
            if *current == other {
                return index;
            }
        }

        let index = self.constants.len();

        self.constants.push(other);

        index
    }

    pub fn load_global_const(&mut self, id: NodeId) -> usize {
        if let Some(index) = self.globals.get(&id) {
            return *index;
        };

        let index = self.constants.len();

        self.globals.insert(id, index);
        self.constants.push(Value::Null);

        index
    }

    pub fn update_global_const(&mut self, id: NodeId, value: Value) {
        if let Some(index) = self.globals.get(&id) {
            self.constants[*index] = value;
        } else {
            let index = self.constants.len();

            self.globals.insert(id, index);
            self.constants.push(value);
        }
    }
}
