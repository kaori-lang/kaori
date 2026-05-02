use std::hint::unreachable_unchecked;

use foldhash::HashMap;

use super::value::Value;

enum Object {
    Vec(Vec<Value>),
    Dict(HashMap<Value, Value>),
    Closure {
        function: usize,
        captured: Vec<Value>,
    },
}

#[derive(Default)]
pub struct Gc {
    objects: Vec<Object>,
    free_list: Vec<usize>,
}

impl Gc {
    fn alloc(&mut self, object: Object) -> usize {
        if let Some(index) = self.free_list.pop() {
            self.objects[index] = object;

            index
        } else {
            let index = self.objects.len();
            self.objects.push(object);
            index
        }
    }

    pub fn allocate_dict(&mut self) -> Value {
        let object = Object::Dict(HashMap::default());
        let index = self.alloc(object);

        Value::dict(index)
    }

    pub fn allocate_vec(&mut self) -> Value {
        let object = Object::Vec(Vec::new());
        let index = self.alloc(object);

        Value::vec(index)
    }

    pub fn get_mut_vec(&mut self, value: Value) -> &mut Vec<Value> {
        let index = value.as_index();

        match &mut self.objects[index] {
            Object::Vec(v) => v,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn get_mut_dict(&mut self, value: Value) -> &mut HashMap<Value, Value> {
        let index = value.as_index();

        match &mut self.objects[index] {
            Object::Dict(d) => d,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn get_vec(&self, value: Value) -> &Vec<Value> {
        let index = value.as_index();

        match &self.objects[index] {
            Object::Vec(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn get_dict(&self, value: Value) -> &HashMap<Value, Value> {
        let index = value.as_index();

        match &self.objects[index] {
            Object::Dict(d) => d,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
