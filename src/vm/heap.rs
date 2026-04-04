use std::{collections::HashMap, hint::unreachable_unchecked};

use crate::bytecode::value::Value;

#[derive(Debug)]
pub enum HeapObject {
    String(String),
    Dict(HashMap<String, Value>),
    Vec(Vec<Value>),
}

#[derive(Debug)]
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
    pub fn allocate_dict(&mut self) -> Value {
        let index = self.objects.len();
        let dict = HashMap::new();

        self.objects.push(HeapObject::Dict(dict));

        Value::dict(index)
    }

    #[inline(always)]
    pub fn allocate_vec(&mut self) -> Value {
        let index = self.objects.len();

        let vec = Vec::new();

        self.objects.push(HeapObject::Vec(vec));

        Value::vec(index)
    }

    pub fn get_string(&self, value: Value) -> &str {
        let index = value.expect_string();

        match &self.objects[index] {
            HeapObject::String(object) => object,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn get_dict(&mut self, value: Value) -> &mut HashMap<String, Value> {
        let index = value.expect_dict();

        match &mut self.objects[index] {
            HeapObject::Dict(object) => object,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
