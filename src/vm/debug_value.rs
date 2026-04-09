use std::fmt;

use crate::bytecode::value::{Value, ValueKind};

use super::gc::Gc;

pub struct DebugValue<'a> {
    pub value: Value,
    pub gc: &'a Gc,
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
                let s = self.gc.get_string(self.value);

                write!(f, "{}", s)
            }
            ValueKind::Dict => {
                let dict = self.gc.get_dict(self.value);

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
                            gc: self.gc
                        },
                        DebugValue {
                            value: *value,
                            gc: self.gc
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
