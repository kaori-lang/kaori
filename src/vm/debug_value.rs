use std::fmt;

use crate::bytecode::value::{Value, ValueKind};

use super::heap::Heap;

pub struct DebugValue<'a> {
    pub value: Value,
    pub heap: &'a Heap,
}

impl<'a> fmt::Debug for DebugValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value.kind() {
            ValueKind::Number => {
                write!(f, "{}", self.value.as_number())
            }
            ValueKind::Boolean => {
                write!(f, "{}", self.value.as_boolean())
            }
            ValueKind::Function => {
                write!(f, "Function({})", self.value.as_function())
            }
            ValueKind::String => {
                let s = self.heap.get_string(self.value);

                write!(f, "{}", s)
            }
            ValueKind::Dict => {
                let dict = self.heap.get_dict(self.value);

                write!(f, "{{")?;

                let mut first = true;
                for (key, value) in dict {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;

                    write!(
                        f,
                        "{:?}: {:?}",
                        DebugValue {
                            value: *key,
                            heap: self.heap
                        },
                        DebugValue {
                            value: *value,
                            heap: self.heap
                        },
                    )?;
                }

                write!(f, "}}")
            }
            ValueKind::Vec => {
                write!(f, "")
            }
        }
    }
}
