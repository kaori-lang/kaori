#[derive(Clone, Copy)]
pub union Value {
    number: f64,
    boolean: bool,
    str: *mut String,
}

impl Value {
    pub fn as_number(&self) -> f64 {
        unsafe { self.number }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { self.boolean }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value { boolean: false }
    }
}

pub enum ConstValue {
    Bool(bool),
    Number(f64),
}

impl ConstValue {
    pub fn equal(&self, other: &ConstValue) -> bool {
        match (self, other) {
            (ConstValue::Bool(l), ConstValue::Bool(r)) => l == r,
            (ConstValue::Number(l), ConstValue::Number(r)) => l == r,
            _ => false,
        }
    }
}
