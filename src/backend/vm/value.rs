#[derive(Clone, Copy)]
pub union Value {
    number: f64,
    boolean: bool,
    str: *mut String,
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value { number: value }
    }

    pub fn boolean(value: bool) -> Value {
        Value { boolean: value }
    }

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
