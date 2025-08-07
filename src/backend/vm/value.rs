use std::{hint::unreachable_unchecked, rc::Rc};

#[derive(Clone, PartialEq)]

pub enum Value {
    Number(f64),
    Bool(bool),
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value::Number(value)
    }

    pub fn boolean(value: bool) -> Value {
        Value::Bool(value)
    }

    /*  */
    /// # Safety
    /// Caller must ensure that `self` is `Value::Number`.
    /// Calling this on any other variant results in **undefined behavior**.

    pub unsafe fn as_number(self) -> f64 {
        match self {
            Value::Number(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    /// # Safety
    /// Caller must ensure that `self` is `Value::Bool`.
    /// Calling this on any other variant results in **undefined behavior**.

    pub unsafe fn as_bool(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::boolean(false)
    }
}
