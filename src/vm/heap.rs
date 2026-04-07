use std::collections::HashMap;

use crate::bytecode::value::Value;

#[derive(Default)]
pub struct Heap {
    strings: Vec<String>,
    strings_interned: HashMap<String, usize>,
    dicts: Vec<HashMap<Value, Value>>,
    vecs: Vec<Vec<Value>>,
}

impl Heap {
    #[inline(always)]
    pub fn allocate_string(&mut self, s: String) -> Value {
        if let Some(index) = self.strings_interned.get(&s) {
            return Value::string(*index);
        }

        let index = self.strings.len();
        self.strings.push(s.to_owned());
        self.strings_interned.insert(s, index);

        Value::string(index)
    }

    #[inline(always)]
    pub fn allocate_dict(&mut self) -> Value {
        let index = self.dicts.len();
        self.dicts.push(HashMap::new());

        Value::dict(index)
    }

    #[inline(always)]
    pub fn allocate_vec(&mut self) -> Value {
        let index = self.vecs.len();
        self.vecs.push(Vec::new());

        Value::vec(index)
    }

    #[inline(always)]
    pub fn get_string(&self, value: Value) -> &str {
        let index = value.expect_string();
        &self.strings[index]
    }

    #[inline(always)]
    pub fn get_mut_dict(&mut self, value: Value) -> &mut HashMap<Value, Value> {
        let index = value.expect_dict();
        &mut self.dicts[index]
    }

    #[inline(always)]
    pub fn get_dict(&self, value: Value) -> &HashMap<Value, Value> {
        let index = value.expect_dict();
        &self.dicts[index]
    }

    #[inline(always)]
    pub fn get_mut_vec(&mut self, value: Value) -> &mut Vec<Value> {
        let index = value.expect_vec();
        &mut self.vecs[index]
    }

    #[inline(always)]
    pub fn get_vec(&self, value: Value) -> &Vec<Value> {
        let index = value.expect_vec();
        &self.vecs[index]
    }
}
