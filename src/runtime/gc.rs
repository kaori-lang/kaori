use std::hint::unreachable_unchecked;

use super::value::Value;
use ahash::AHashMap;

struct GcNode {
    object: GcObject,
    next: Option<Box<GcNode>>,
}

pub enum GcObject {
    String(Box<str>),
    Vec(Box<Vec<Value>>),
    Dict(Box<AHashMap<Value, Value>>),
}

impl GcObject {
    pub fn new_string(value: &str) -> Self {
        Self::String(Box::from(value))
    }

    pub fn new_vec() -> Self {
        Self::Vec(Box::default())
    }

    pub fn new_dict() -> Self {
        Self::Dict(Box::default())
    }

    #[inline(always)]
    pub fn as_string(&self) -> &str {
        let Self::String(value) = self else {
            unsafe { unreachable_unchecked() }
        };

        value
    }

    #[inline(always)]
    pub fn as_vec(&mut self) -> &mut Vec<Value> {
        let Self::Vec(value) = self else {
            unsafe { unreachable_unchecked() }
        };

        value
    }

    #[inline(always)]
    pub fn as_dict(&mut self) -> &mut AHashMap<Value, Value> {
        let Self::Dict(value) = self else {
            unsafe { unreachable_unchecked() }
        };

        value
    }
}

#[derive(Default)]
pub struct Gc {
    head: Option<Box<GcNode>>,
    strings_interned: AHashMap<String, *mut GcObject>,
}

impl Gc {
    pub fn allocate_dict(&mut self) -> Value {
        Value::dict(self.alloc(GcObject::new_dict()))
    }

    pub fn allocate_vec(&mut self) -> Value {
        Value::vec(self.alloc(GcObject::new_vec()))
    }

    pub fn allocate_string(&mut self, s: &str) -> Value {
        if let Some(&ptr) = self.strings_interned.get(s) {
            return Value::string(ptr);
        }

        let ptr = self.alloc(GcObject::new_string(s));
        self.strings_interned.insert(s.to_owned(), ptr);

        Value::string(ptr)
    }

    fn alloc(&mut self, object: GcObject) -> *mut GcObject {
        let mut node = Box::new(GcNode {
            object,
            next: self.head.take(),
        });
        let ptr: *mut GcObject = &mut node.object;
        self.head = Some(node);
        ptr
    }
}
