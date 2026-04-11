use std::{collections::HashMap, rc::Rc};

use super::value::Value;

pub struct GcObject {
    tag: ObjectKind,
    pub data: *mut u8,
}

enum ObjectKind {
    String,
    Vec,
    Dict,
}

impl GcObject {
    pub fn new_string(s: Rc<str>) -> Self {
        Self {
            tag: ObjectKind::String,
            data: Box::into_raw(Box::new(s)) as *mut u8,
        }
    }

    pub fn new_vec() -> Self {
        Self {
            tag: ObjectKind::Vec,
            data: Box::into_raw(Box::new(Vec::<Value>::new())) as *mut u8,
        }
    }

    pub fn new_dict() -> Self {
        Self {
            tag: ObjectKind::Dict,
            data: Box::into_raw(Box::new(HashMap::<Value, Value>::new())) as *mut u8,
        }
    }
}

impl Drop for GcObject {
    fn drop(&mut self) {
        unsafe {
            match self.tag {
                ObjectKind::String => drop(Box::from_raw(self.data as *mut Rc<str>)),
                ObjectKind::Vec => drop(Box::from_raw(self.data as *mut Vec<Value>)),
                ObjectKind::Dict => drop(Box::from_raw(self.data as *mut HashMap<Value, Value>)),
            }
        }
    }
}

#[derive(Default)]
pub struct Gc {
    objects: Vec<Box<GcObject>>,
    strings_interned: HashMap<Rc<str>, *mut GcObject>,
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
        let mut boxed = Box::new(object);
        let ptr: *mut GcObject = &mut *boxed;
        self.objects.push(boxed);
        ptr
    }
}
