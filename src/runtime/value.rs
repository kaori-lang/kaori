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
    Closure(usize),
    Function(usize),
    Native(usize),
    Map(usize),
    Vec(usize),
    Cell(usize),
}

#[allow(clippy::should_implement_trait)]
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
    pub fn closure(index: usize) -> Self {
        Value::Closure(index)
    }
    pub fn function(index: usize) -> Self {
        Value::Function(index)
    }
    pub fn native(index: usize) -> Self {
        Value::Native(index)
    }
    pub fn map(index: usize) -> Self {
        Value::Map(index)
    }
    pub fn vec(index: usize) -> Self {
        Value::Vec(index)
    }
    pub fn cell(index: usize) -> Self {
        Value::Cell(index)
    }
}
