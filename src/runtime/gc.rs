use foldhash::HashMap;

use crate::runtime::instruction::Instruction;

use super::value::Value;

pub struct Closure {
    pub captured: Vec<Value>,
    pub instructions: *const Instruction,
    pub constants: *const Value,
    pub arity: u8,
    pub frame_size: u8,
}

#[derive(Default)]
pub struct Gc {
    vecs: Vec<Vec<Value>>,
    dicts: Vec<HashMap<Value, Value>>,
    closures: Vec<Closure>,

    free_vecs: Vec<usize>,
    free_dicts: Vec<usize>,
    free_closures: Vec<usize>,
}

impl Gc {
    #[inline(always)]
    fn alloc_vec(&mut self, object: Vec<Value>) -> usize {
        if let Some(index) = self.free_vecs.pop() {
            self.vecs[index] = object;
            index
        } else {
            let index = self.vecs.len();

            self.vecs.push(object);

            index
        }
    }

    #[inline(always)]
    fn alloc_dict(&mut self, object: HashMap<Value, Value>) -> usize {
        if let Some(index) = self.free_dicts.pop() {
            self.dicts[index] = object;
            index
        } else {
            let index = self.dicts.len();

            self.dicts.push(object);

            index
        }
    }

    #[inline(always)]
    fn alloc_closure(&mut self, object: Closure) -> usize {
        if let Some(index) = self.free_closures.pop() {
            self.closures[index] = object;
            index
        } else {
            let index = self.closures.len();

            self.closures.push(object);

            index
        }
    }

    #[inline(always)]
    pub fn allocate_vec(&mut self) -> Value {
        let index = self.alloc_vec(Vec::new());

        Value::vec(index)
    }

    #[inline(always)]
    pub fn allocate_dict(&mut self) -> Value {
        let index = self.alloc_dict(HashMap::default());

        Value::dict(index)
    }

    #[inline(always)]
    pub fn allocate_closure(&mut self, closure: Closure) -> Value {
        let index = self.alloc_closure(closure);

        Value::closure(index)
    }

    #[inline(always)]
    pub fn get_vec(&self, value: Value) -> &Vec<Value> {
        unsafe { self.vecs.get_unchecked(value.as_index()) }
    }

    #[inline(always)]
    pub fn get_mut_vec(&mut self, value: Value) -> &mut Vec<Value> {
        unsafe { self.vecs.get_unchecked_mut(value.as_index()) }
    }

    #[inline(always)]
    pub fn get_dict(&self, value: Value) -> &HashMap<Value, Value> {
        unsafe { self.dicts.get_unchecked(value.as_index()) }
    }

    #[inline(always)]
    pub fn get_mut_dict(&mut self, value: Value) -> &mut HashMap<Value, Value> {
        unsafe { self.dicts.get_unchecked_mut(value.as_index()) }
    }

    #[inline(always)]
    pub fn get_closure(&self, value: Value) -> &Closure {
        unsafe { self.closures.get_unchecked(value.as_index()) }
    }

    #[inline(always)]
    pub fn get_mut_closure(&mut self, value: Value) -> &mut Closure {
        unsafe { self.closures.get_unchecked_mut(value.as_index()) }
    }

    #[inline(always)]
    pub fn free_vec(&mut self, value: Value) {
        let index = value.as_index();

        self.vecs[index].clear();

        self.free_vecs.push(index);
    }

    #[inline(always)]
    pub fn free_dict(&mut self, value: Value) {
        let index = value.as_index();

        self.dicts[index].clear();

        self.free_dicts.push(index);
    }

    #[inline(always)]
    pub fn free_closure(&mut self, value: Value) {
        let index = value.as_index();

        self.closures[index].captured.clear();

        self.free_closures.push(index);
    }
}
