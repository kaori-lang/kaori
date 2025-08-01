use crate::backend::vm::value::Value;

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

    pub fn to_union(&self) -> Value {
        match self {
            ConstValue::Bool(value) => Value::boolean(*value),
            ConstValue::Number(value) => Value::number(*value),
        }
    }
}
