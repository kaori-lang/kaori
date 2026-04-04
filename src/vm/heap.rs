use std::collections::HashMap;

use crate::bytecode::value::Value;

#[derive(Debug)]
pub enum HeapObject {
    String(String),
    Dict(HashMap<String, Value>),
    Vec(Vec<Value>),
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

    #[inline(always)]
    pub fn allocate_string(&mut self, s: String) -> Value {
        let index = self.objects.len();
        self.objects.push(HeapObject::String(s));

        Value::string(index)
    }

    #[inline(always)]
    pub fn allocate_dict(&mut self, d: HashMap<String, Value>) -> Value {
        let index = self.objects.len();
        self.objects.push(HeapObject::Dict(d));

        Value::dict(index)
    }

    #[inline(always)]
    pub fn allocate_vec(&mut self, v: Vec<Value>) -> Value {
        let index = self.objects.len();
        self.objects.push(HeapObject::Vec(v));

        Value::vec(index)
    }

    pub fn get_string(&self, index: usize) -> &str {
        match &self.objects[index] {
            HeapObject::String(s) => s,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
