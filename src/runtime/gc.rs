use std::{hint::unreachable_unchecked, rc::Rc};

use super::value::Value;
use ahash::AHashMap;

struct GcNode {
    object: GcObject,
    next: Option<Box<GcNode>>,
}

pub enum GcObject {
    String(Rc<str>),
    Vec(Vec<Value>),
    Dict(AHashMap<Value, Value>),
}

impl GcObject {
    pub fn new_string(s: Rc<str>) -> Self {
        Self::String(s)
    }

    pub fn new_vec() -> Self {
        Self::Vec(Vec::new())
    }

    pub fn new_dict() -> Self {
        Self::Dict(AHashMap::new())
    }

    #[inline(always)]
    pub fn as_string(&self) -> &Rc<str> {
        let Self::String(s) = self else {
            unsafe { unreachable_unchecked() }
        };
        s
    }

    #[inline(always)]
    pub fn as_vec(&mut self) -> &mut Vec<Value> {
        let Self::Vec(v) = self else {
            unsafe { unreachable_unchecked() }
        };
        v
    }

    #[inline(always)]
    pub fn as_dict(&mut self) -> &mut AHashMap<Value, Value> {
        let Self::Dict(d) = self else {
            unsafe { unreachable_unchecked() }
        };
        d
    }
}

pub struct Gc {
    head: Option<Box<GcNode>>,
    strings_interned: AHashMap<Rc<str>, *mut GcObject>,
}

impl Default for Gc {
    fn default() -> Self {
        Self {
            head: None,
            strings_interned: AHashMap::new(),
        }
    }
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

        let rc: Rc<str> = Rc::from(s);
        let ptr = self.alloc(GcObject::new_string(rc.clone()));
        self.strings_interned.insert(rc, ptr);

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
