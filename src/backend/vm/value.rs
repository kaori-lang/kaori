#![allow(clippy::missing_safety_doc)]
use std::hint::unreachable_unchecked;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Value {
    Number(f64),
    Bool(bool),

    Null,
    InstructionPtr(usize),
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value::Number(value)
    }

    pub fn boolean(value: bool) -> Value {
        Value::Bool(value)
    }

    pub fn instruction_ptr(instruction_ptr: usize) -> Value {
        Value::InstructionPtr(instruction_ptr)
    }

    pub unsafe fn as_number(self) -> f64 {
        match self {
            Value::Number(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub unsafe fn as_bool(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub unsafe fn as_instruction_ptr(self) -> usize {
        match self {
            Value::InstructionPtr(value) => value,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::boolean(false)
    }
}
