use foldhash::HashMap;

use super::value::Value;

#[derive(Default)]
pub struct Gc {
    vecs: Vec<Vec<Value>>,
    maps: Vec<HashMap<Value, Value>>,
    closures: Vec<Vec<Value>>,
    cells: Vec<Value>,

    free_vecs: Vec<usize>,
    free_maps: Vec<usize>,
    free_closures: Vec<usize>,
    free_cells: Vec<usize>,
}

impl Gc {
    #[inline(always)]
    fn alloc<T>(objects: &mut Vec<T>, free_list: &mut Vec<usize>, object: T) -> usize {
        if let Some(index) = free_list.pop() {
            objects[index] = object;
            index
        } else {
            let index = objects.len();
            objects.push(object);
            index
        }
    }

    #[inline(always)]
    pub fn allocate_vec(&mut self) -> Value {
        Value::vec(Self::alloc(&mut self.vecs, &mut self.free_vecs, Vec::new()))
    }

    #[inline(always)]
    pub fn allocate_map(&mut self) -> Value {
        Value::map(Self::alloc(
            &mut self.maps,
            &mut self.free_maps,
            HashMap::default(),
        ))
    }

    #[inline(always)]
    pub fn allocate_closure(&mut self, closure: Vec<Value>) -> Value {
        Value::closure(Self::alloc(
            &mut self.closures,
            &mut self.free_closures,
            closure,
        ))
    }

    #[inline(always)]
    pub fn allocate_cell(&mut self, value: Value) -> Value {
        Value::cell(Self::alloc(&mut self.cells, &mut self.free_cells, value))
    }

    #[inline(always)]
    pub fn get_vec(&self, index: usize) -> &Vec<Value> {
        unsafe { self.vecs.get_unchecked(index) }
    }

    #[inline(always)]
    pub fn get_mut_vec(&mut self, index: usize) -> &mut Vec<Value> {
        unsafe { self.vecs.get_unchecked_mut(index) }
    }

    #[inline(always)]
    pub fn get_map(&self, index: usize) -> &HashMap<Value, Value> {
        unsafe { self.maps.get_unchecked(index) }
    }

    #[inline(always)]
    pub fn get_mut_map(&mut self, index: usize) -> &mut HashMap<Value, Value> {
        unsafe { self.maps.get_unchecked_mut(index) }
    }

    #[inline(always)]
    pub fn get_closure(&self, index: usize) -> &Vec<Value> {
        unsafe { self.closures.get_unchecked(index) }
    }

    #[inline(always)]
    pub fn get_mut_closure(&mut self, index: usize) -> &mut Vec<Value> {
        unsafe { self.closures.get_unchecked_mut(index) }
    }

    #[inline(always)]
    pub fn get_cell(&self, index: usize) -> Value {
        unsafe { *self.cells.get_unchecked(index) }
    }

    #[inline(always)]
    pub fn set_cell(&mut self, index: usize, value: Value) {
        unsafe { *self.cells.get_unchecked_mut(index) = value }
    }
}
