use crate::backend::vm::value::Value;

#[derive(PartialEq, Debug)]
pub enum ConstValue {
    Bool(bool),
    Number(f64),
}

impl ConstValue {
    pub fn to_union(&self) -> Value {
        match self {
            ConstValue::Bool(value) => Value::boolean(*value),
            ConstValue::Number(value) => Value::number(*value),
        }
    }
}
