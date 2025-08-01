pub struct Value {
    tag: u8,
    value_data: ValueData,
}

pub union ValueData {
    number: f64,
    boolean: bool,
    str: *mut String,
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value {
            tag: 0,
            value_data: ValueData { number: value },
        }
    }

    pub fn bool(value: bool) -> Value {
        Value {
            tag: 1,
            value_data: ValueData { boolean: value },
        }
    }

    pub fn as_number(&self) -> f64 {
        unsafe { self.value_data.number }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { self.value_data.boolean }
    }

    pub fn is_number(&self) -> bool {
        self.tag == 0
    }

    pub fn is_bool(&self) -> bool {
        self.tag == 1
    }
}
