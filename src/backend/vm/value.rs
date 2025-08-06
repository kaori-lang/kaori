use std::mem::ManuallyDrop;

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

    pub fn str(value: String) -> Value {
        Value {
            tag: ValueTag::String,
            value: ValueUnion {
                str: ManuallyDrop::new(value),
            },
        }
    }

    pub fn as_number(&self) -> f64 {
        unsafe { self.value.number }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { self.value.boolean }
    }

    pub fn as_str(&self) -> &str {
        unsafe { &self.value.str }
    }

    pub fn equal(&self, other: &Value) -> bool {
        match (&self.tag, &other.tag) {
            (ValueTag::Number, ValueTag::Number) => self.as_number() == other.as_number(),
            (ValueTag::Boolean, ValueTag::Boolean) => self.as_bool() == other.as_bool(),
            (ValueTag::String, ValueTag::String) => self.as_str() == other.as_str(),
            _ => false,
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

impl Drop for Value {
    fn drop(&mut self) {
        match self.tag {
            ValueTag::String => unsafe { ManuallyDrop::drop(&mut self.value.str) },
            _ => (),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self.tag {
            ValueTag::Number => Value::number(self.as_number()),
            ValueTag::Boolean => Value::boolean(self.as_bool()),
            ValueTag::String => Value::str(self.as_str().to_owned()),
        }
    }
}

pub enum ValueTag {
    Number,
    Boolean,
    String,
}

pub union ValueUnion {
    number: f64,
    boolean: bool,
    str: ManuallyDrop<String>,
}
