use std::hint::unreachable_unchecked;

#[derive(Clone, PartialEq)]
pub enum Value {
    Str(String),
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

    pub fn str(value: String) -> Value {
        Value::Str(value)
    }

    pub fn as_number(self) -> f64 {
        match self {
            Value::Number(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::Str(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::boolean(false)
    }
}
