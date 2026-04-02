use std::collections::HashMap;

use crate::bytecode::value::Value;

pub enum HeapObject {
    String(String),
    HashMap(HashMap<String, Value>),
}

pub struct Heap {
    objects: Vec<HeapObject>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn allocate(&mut self, object: HeapObject) -> Value {
        let index = self.objects.len();

        self.objects.push(object);

        Value::object(index)
    }
}
