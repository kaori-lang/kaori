use std::mem::ManuallyDrop;

use foldhash::{HashMap, HashMapExt};

use crate::runtime::function::Function;

use super::value::Value;

struct Object<T> {
    marked: bool,
    data: ManuallyDrop<T>,
}

impl<T> Object<T> {
    fn new(data: T) -> Self {
        Self { marked: false, data: ManuallyDrop::new(data) }
    }
}

#[derive(Default)]
struct Arena<T> {
    objects: Vec<Object<T>>,
    free_list: Vec<usize>,
}

impl<T> Arena<T> {
    fn alloc(&mut self, data: T) -> usize {
        if let Some(index) = self.free_list.pop() {
            self.objects[index] = Object::new(data);

            index
        } else {
            let index = self.objects.len();

            self.objects.push(Object::new(data));

            index
        }
    }

    fn get(&self, index: usize) -> &T {
        unsafe { &self.objects.get_unchecked(index).data }
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        unsafe { &mut self.objects.get_unchecked_mut(index).data }
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
    maps: Arena<HashMap<Value, Value>>,
    arrays: Arena<Vec<Value>>,
    closures: Arena<Closure>,
    cells: Arena<Value>,
}

impl Heap {
    pub fn alloc_map(&mut self) -> usize {
        self.maps.alloc(HashMap::new())
    }

    pub fn alloc_array(&mut self) -> usize {
        self.arrays.alloc(Vec::new())
    }

    pub fn alloc_closure(&mut self, closure: Closure) -> usize {
        self.closures.alloc(closure)
    }

    pub fn alloc_cell(&mut self, value: Value) -> usize {
        self.cells.alloc(value)
    }

    pub fn get_map(&self, index: usize) -> &HashMap<Value, Value> {
        self.maps.get(index)
    }

    pub fn get_map_mut(&mut self, index: usize) -> &mut HashMap<Value, Value> {
        self.maps.get_mut(index)
    }

    pub fn get_array(&self, index: usize) -> &[Value] {
        self.arrays.get(index)
    }

    pub fn get_array_mut(&mut self, index: usize) -> &mut Vec<Value> {
        self.arrays.get_mut(index)
    }

    pub fn get_closure(&self, index: usize) -> &Closure {
        self.closures.get(index)
    }

    pub fn get_closure_mut(&mut self, index: usize) -> &mut Closure {
        self.closures.get_mut(index)
    }

    pub fn get_cell(&self, index: usize) -> &Value {
        self.cells.get(index)
    }

    pub fn get_cell_mut(&mut self, index: usize) -> &mut Value {
        self.cells.get_mut(index)
    }
}
