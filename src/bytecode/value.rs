#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Value {
    Number(f64),
    Bool(bool),
    InstructionIndex(usize),
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value::Number(value)
    }

    pub fn boolean(value: bool) -> Value {
        Value::Bool(value)
    }

    pub fn instruction_index(instruction_index: usize) -> Value {
        Value::InstructionIndex(instruction_index)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::boolean(false)
    }
}
