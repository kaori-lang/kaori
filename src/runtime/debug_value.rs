use std::fmt;

use crate::runtime::{gc::Gc, value::Value};

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
        if self.value.is_closure() {
            return write!(f, "Closure({:p})", self.gc.get_closure(self.value));
        }
        if self.value.is_string() {
            return write!(f, "{}", self.value.as_string());
        }
        if self.value.is_vec() {
            let mut list = f.debug_list();
            for &value in self.gc.get_vec(self.value) {
                list.entry(&DebugValue::new(value, self.gc));
            }
            return list.finish();
        }
        if self.value.is_dict() {
            let mut map = f.debug_map();

            for (&key, &val) in self.gc.get_dict(self.value) {
                map.entry(
                    &DebugValue::new(key, self.gc),
                    &DebugValue::new(val, self.gc),
                );
            }
            return map.finish();
        }
        unsafe { std::hint::unreachable_unchecked() }
    }
}
