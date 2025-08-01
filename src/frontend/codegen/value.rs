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
    Bool(Value),
    Number(Value),
}

impl ConstValue {
    pub fn bool(value: bool) -> ConstValue {
        ConstValue::Bool(Value { boolean: value })
    }

    pub fn number(value: f64) -> ConstValue {
        ConstValue::Bool(Value { number: value })
    }

    pub fn equal(&self, other: &ConstValue) -> bool {
        match (self, other) {
            (ConstValue::Bool(l), ConstValue::Bool(r)) => l.as_bool() == r.as_bool(),
            (ConstValue::Number(l), ConstValue::Number(r)) => l.as_number() == r.as_number(),
            _ => false,
        }
    }

    pub fn to_value(&self) -> Value {
        match self {
            ConstValue::Bool(value) => *value,
            ConstValue::Number(value) => *value,
        }
    }
}
