#[derive(Clone, Copy)]
pub struct Value {
    tag: ValueTag,
    value: ValueUnion,
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value {
            tag: ValueTag::Number,
            value: ValueUnion { number: value },
        }
    }

    pub fn boolean(value: bool) -> Value {
        Value {
            tag: ValueTag::Boolean,
            value: ValueUnion { boolean: value },
        }
    }

    pub fn as_number(&self) -> f64 {
        unsafe { self.value.number }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { self.value.boolean }
    }

    pub fn equal(&self, other: &Value) -> bool {
        match (self.tag, other.tag) {
            (ValueTag::Number, ValueTag::Number) => self.as_number() == other.as_number(),
            (ValueTag::Boolean, ValueTag::Boolean) => self.as_bool() == other.as_bool(),
            _ => true,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value {
            tag: ValueTag::Boolean,
            value: ValueUnion { boolean: false },
        }
    }
}

#[derive(Clone, Copy)]
pub enum ValueTag {
    Number,
    Boolean,
    String,
}

#[derive(Clone, Copy)]
pub union ValueUnion {
    number: f64,
    boolean: bool,
    str: [char; 10],
}
