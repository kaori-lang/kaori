use std::fmt;

use crate::{
    compiler::INTERNER,
    runtime::{gc::Gc, value::Value},
    util::string_interner::Symbol,
};

pub struct DebugValue<'a> {
    value: Value,
    gc: &'a Gc,
}

impl<'a> DebugValue<'a> {
    pub fn new(value: Value, gc: &'a Gc) -> Self {
        Self { value, gc }
    }
}

impl<'a> fmt::Debug for DebugValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_number() {
            return write!(f, "{}", self.value.as_number());
        }
        if self.value.is_bool() {
            return write!(f, "{}", self.value.as_bool());
        }
        if self.value.is_nil() {
            return write!(f, "nil");
        }
        if self.value.is_cell() {
            return write!(f, "Cell({:p})", &self.value);
        }
        if self.value.is_closure() {
            return write!(f, "Closure({:p})", self.gc.get_closure(self.value));
        }
        if self.value.is_string() {
            let index = Symbol(self.value.as_index() as u32);

            return write!(f, "{}", INTERNER.lock().unwrap().resolve(index));
        }
        if self.value.is_vec() {
            let mut list = f.debug_list();
            for &value in self.gc.get_vec(self.value) {
                list.entry(&DebugValue::new(value, self.gc));
            }
            return list.finish();
        }
        if self.value.is_map() {
            let mut map = f.debug_map();

            for (&key, &val) in self.gc.get_map(self.value) {
                map.entry(
                    &DebugValue::new(key, self.gc),
                    &DebugValue::new(val, self.gc),
                );
            }
            return map.finish();
        }

        unreachable!("Should not be reached, tried to debug invalid tag value")
    }
}
