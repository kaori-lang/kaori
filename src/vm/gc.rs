use std::{collections::HashMap, rc::Rc};

use crate::bytecode::value::Value;

#[derive(Default)]
pub struct Gc {
    objects: Vec<Option<GcObject>>,
    strings_interned: HashMap<Rc<str>, usize>,
    free_list: Vec<usize>,
}

impl Gc {
    pub fn allocate_dict(&mut self) -> Value {
        let object = GcObject::create_dict();
        let index = self.alloc(object);

        Value::dict(index)
    }

    pub fn allocate_vec(&mut self) -> Value {
        let object = GcObject::create_vec();
        let index = self.alloc(object);

        Value::vec(index)
    }

    pub fn allocate_string(&mut self, s: &str) -> Value {
        if let Some(index) = self.strings_interned.get(s) {
            return Value::string(*index);
        }

        let rc: Rc<str> = Rc::from(s);

        let object = GcObject::create_string(rc.clone());
        let index = self.alloc(object);
        self.strings_interned.insert(rc, index);

        Value::string(index)
    }

    fn alloc(&mut self, object: GcObject) -> usize {
        match self.free_list.pop() {
            Some(index) => {
                self.objects[index] = Some(object);
                index
            }
            None => {
                let index = self.objects.len();
                self.objects.push(Some(object));
                index
            }
        }
    }

    #[inline(always)]
    pub fn get_string(&self, value: Value) -> &str {
        let index = value.expect_string();

        match &self.objects[index] {
            Some(GcObject {
                marked: _,
                data: Object::String(rc),
            }) => rc,
            _ => unreachable!("Value tagged as string but not a string"),
        }
    }

    #[inline(always)]
    pub fn get_dict(&self, value: Value) -> &HashMap<Value, Value> {
        let index = value.expect_dict();

        match &self.objects[index] {
            Some(GcObject {
                marked: _,
                data: Object::Dict(map),
            }) => map,
            _ => unreachable!("Value tagged as dict but not a dict"),
        }
    }

    #[inline(always)]
    pub fn get_mut_dict(&mut self, value: Value) -> &mut HashMap<Value, Value> {
        let index = value.expect_dict();

        match &mut self.objects[index] {
            Some(GcObject {
                marked: _,
                data: Object::Dict(map),
            }) => map,
            _ => unreachable!("Value tagged as dict but not a dict"),
        }
    }

    #[inline(always)]
    pub fn get_vec(&self, value: Value) -> &Vec<Value> {
        let index = value.expect_vec();

        match &self.objects[index] {
            Some(GcObject {
                marked: _,
                data: Object::Vec(vec),
            }) => vec,
            _ => unreachable!("Value tagged as vec but not a vec"),
        }
    }

    #[inline(always)]
    pub fn get_mut_vec(&mut self, value: Value) -> &mut Vec<Value> {
        let index = value.expect_vec();

        match &mut self.objects[index] {
            Some(GcObject {
                marked: _,
                data: Object::Vec(vec),
            }) => vec,
            _ => unreachable!("Value tagged as vec but not a vec"),
        }
    }
}

struct GcObject {
    pub marked: bool,
    pub data: Object,
}

impl GcObject {
    pub fn create_dict() -> Self {
        GcObject {
            marked: false,
            data: Object::Dict(HashMap::new()),
        }
    }

    pub fn create_vec() -> Self {
        GcObject {
            marked: false,
            data: Object::Vec(Vec::new()),
        }
    }

    pub fn create_string(s: Rc<str>) -> Self {
        Self {
            marked: false,
            data: Object::String(s),
        }
    }
}
enum Object {
    String(Rc<str>),
    Vec(Vec<Value>),
    Dict(HashMap<Value, Value>),
}
