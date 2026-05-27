use foldhash::HashMap;

use crate::bytecode::{function::Function, instruction::Instruction};

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
    maps: Vec<HashMap<Value, Value>>,
    closures: Vec<Closure>,
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
        let index = Self::alloc(&mut self.vecs, &mut self.free_vecs, Vec::new());

        Value::vec(index)
    }

    #[inline(always)]
    pub fn allocate_map(&mut self) -> Value {
        let index = Self::alloc(&mut self.maps, &mut self.free_maps, HashMap::default());

        Value::map(index)
    }

    #[inline(always)]
    pub fn allocate_closure(&mut self, closure: Closure) -> Value {
        let index = Self::alloc(&mut self.closures, &mut self.free_closures, closure);

        Value::closure(index)
    }

    #[inline(always)]
    pub fn allocate_cell(&mut self, value: Value) -> Value {
        let index = Self::alloc(&mut self.cells, &mut self.free_cells, value);

        Value::cell(index)
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
    pub fn get_map(&self, value: Value) -> &HashMap<Value, Value> {
        unsafe { self.maps.get_unchecked(value.as_index()) }
    }

    #[inline(always)]
    pub fn get_mut_map(&mut self, value: Value) -> &mut HashMap<Value, Value> {
        unsafe { self.maps.get_unchecked_mut(value.as_index()) }
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
    pub fn get_cell(&self, value: Value) -> Value {
        unsafe { *self.cells.get_unchecked(value.as_index()) }
    }

    #[inline(always)]
    pub fn set_cell(&mut self, cell: Value, value: Value) {
        unsafe {
            *self.cells.get_unchecked_mut(cell.as_index()) = value;
        }
    }
}
