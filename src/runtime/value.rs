use ordered_float::OrderedFloat;

use crate::util::string_interner::Symbol;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Value {
    Number(OrderedFloat<f64>),
    Nil,
    Bool(bool),
    String(Symbol),
    Closure(u32),
    Function(u32),
    Native(u32),
    Map(u32),
    Vec(u32),
    Cell(u32),
}

#[allow(clippy::derivable_impls)]
impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl Value {
    pub fn number(value: OrderedFloat<f64>) -> Self {
        Value::Number(value)
    }
    pub fn nil() -> Self {
        Value::Nil
    }
    pub fn bool(value: bool) -> Self {
        Value::Bool(value)
    }
    pub fn string(index: Symbol) -> Self {
        Value::String(index)
    }
    pub fn closure(index: u32) -> Self {
        Value::Closure(index)
    }
    pub fn function(index: u32) -> Self {
        Value::Function(index)
    }
    pub fn native(index: u32) -> Self {
        Value::Native(index)
    }
    pub fn map(index: u32) -> Self {
        Value::Map(index)
    }
    pub fn vec(index: u32) -> Self {
        Value::Vec(index)
    }
    pub fn cell(index: u32) -> Self {
        Value::Cell(index)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    pub fn is_closure(&self) -> bool {
        matches!(self, Value::Closure(_))
    }
    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
    }
    pub fn is_native(&self) -> bool {
        matches!(self, Value::Native(_))
    }
    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }
    pub fn is_vec(&self) -> bool {
        matches!(self, Value::Vec(_))
    }
    pub fn is_cell(&self) -> bool {
        matches!(self, Value::Cell(_))
    }

    // ── as ──────────────────────────────────────────────────────────────────

    pub fn as_number(&self) -> OrderedFloat<f64> {
        match self {
            Value::Number(v) => *v,
            // SAFETY: caller guarantees this is Value::Number
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            // SAFETY: caller guarantees this is Value::Bool
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_string(&self) -> Symbol {
        match self {
            Value::String(v) => *v,
            // SAFETY: caller guarantees this is Value::String
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_closure(&self) -> u32 {
        match self {
            Value::Closure(v) => *v,
            // SAFETY: caller guarantees this is Value::Closure
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_function(&self) -> u32 {
        match self {
            Value::Function(v) => *v,
            // SAFETY: caller guarantees this is Value::Function
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_native(&self) -> u32 {
        match self {
            Value::Native(v) => *v,
            // SAFETY: caller guarantees this is Value::Native
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_map(&self) -> u32 {
        match self {
            Value::Map(v) => *v,
            // SAFETY: caller guarantees this is Value::Map
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_vec(&self) -> u32 {
        match self {
            Value::Vec(v) => *v,
            // SAFETY: caller guarantees this is Value::Vec
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub fn as_cell(&self) -> u32 {
        match self {
            Value::Cell(v) => *v,
            // SAFETY: caller guarantees this is Value::Cell
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
