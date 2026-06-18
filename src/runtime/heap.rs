use super::value::Value;
use crate::runtime::function::Function;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
struct Object<T> {
    marked: bool,
    data: T,
}

impl<T> Object<T> {
    fn new(data: T) -> Self {
        Self { marked: false, data }
    }
}

#[derive(Default)]
struct Arena<T> {
    objects: Vec<Object<T>>,
    free_list: Vec<u32>,
}

impl<T> Arena<T> {
    fn alloc(&mut self, data: T) -> u32 {
        if let Some(index) = self.free_list.pop() {
            self.objects[index as usize] = Object::new(data);

            index
        } else {
            let index = self.objects.len() as u32;

            self.objects.push(Object::new(data));

            index
        }
    }

    fn get(&self, index: u32) -> &T {
        unsafe { &self.objects.get_unchecked(index as usize).data }
    }

    fn get_mut(&mut self, index: u32) -> &mut T {
        unsafe { &mut self.objects.get_unchecked_mut(index as usize).data }
    }
}

#[derive(Debug, Default)]
pub struct Closure {
    pub function: *const Function,
    pub captures: Box<[Value]>,
}

impl Closure {
    pub fn new(function: *const Function, captures: Box<[Value]>) -> Self {
        Self { function, captures }
    }
}

#[derive(Default)]
pub struct Heap {
    maps: Arena<HashMap<Value, Value, FxBuildHasher>>,
    arrays: Arena<Vec<Value>>,
    closures: Arena<Closure>,
    cells: Arena<Value>,
}

impl Heap {
    pub fn alloc_map(&mut self) -> u32 {
        self.maps.alloc(HashMap::with_hasher(FxBuildHasher))
    }

    pub fn alloc_array(&mut self) -> u32 {
        self.arrays.alloc(Vec::new())
    }

    pub fn alloc_closure(&mut self, closure: Closure) -> u32 {
        self.closures.alloc(closure)
    }

    pub fn alloc_cell(&mut self, value: Value) -> u32 {
        self.cells.alloc(value)
    }

    pub fn get_map(&self, index: u32) -> &HashMap<Value, Value, FxBuildHasher> {
        self.maps.get(index)
    }

    pub fn get_map_mut(
        &mut self,
        index: u32,
    ) -> &mut HashMap<Value, Value, FxBuildHasher> {
        self.maps.get_mut(index)
    }

    pub fn get_array(&self, index: u32) -> &[Value] {
        self.arrays.get(index)
    }

    pub fn get_array_mut(&mut self, index: u32) -> &mut Vec<Value> {
        self.arrays.get_mut(index)
    }

    pub fn get_closure(&self, index: u32) -> &Closure {
        self.closures.get(index)
    }

    pub fn get_closure_mut(&mut self, index: u32) -> &mut Closure {
        self.closures.get_mut(index)
    }

    pub fn get_cell(&self, index: u32) -> &Value {
        self.cells.get(index)
    }

    pub fn get_cell_mut(&mut self, index: u32) -> &mut Value {
        self.cells.get_mut(index)
    }
}
