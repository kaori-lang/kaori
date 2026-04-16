use ahash::AHashMap;
use std::fmt;

use super::function::Function;
use super::gc::GcObject;

const TYPE_MASK: u64 = 0b111;

pub const TYPE_NUMBER: u64 = 0b000;
pub const TYPE_BOOLEAN: u64 = 0b001;
pub const TYPE_FUNCTION: u64 = 0b010;
pub const TYPE_STRING: u64 = 0b011;
pub const TYPE_DICT: u64 = 0b100;
pub const TYPE_VEC: u64 = 0b101;
pub const TYPE_NIL: u64 = 0b110;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(u64);

impl Default for Value {
    fn default() -> Self {
        Value::nil()
    }
}

impl Value {
    #[inline(always)]
    pub fn tag(self) -> u64 {
        self.0 & TYPE_MASK
    }

    #[inline(always)]
    pub fn nil() -> Self {
        Self(TYPE_NIL)
    }

    #[inline(always)]
    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }

    #[inline(always)]
    pub fn boolean(value: bool) -> Self {
        Self(((value as u64) << 3) | TYPE_BOOLEAN)
    }

    #[inline(always)]
    pub fn function(ptr: *const Function) -> Self {
        Self((ptr as u64) | TYPE_FUNCTION)
    }

    #[inline(always)]
    pub fn string(ptr: *mut GcObject<String>) -> Self {
        Self((ptr as u64) | TYPE_STRING)
    }

    #[inline(always)]
    pub fn dict(ptr: *mut GcObject<AHashMap<Value, Value>>) -> Self {
        Self((ptr as u64) | TYPE_DICT)
    }

    #[inline(always)]
    pub fn vec(ptr: *mut GcObject<Vec<Value>>) -> Self {
        Self((ptr as u64) | TYPE_VEC)
    }

    #[inline(always)]
    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }

    #[inline(always)]
    pub fn as_boolean(self) -> bool {
        ((self.0 >> 3) & 1) != 0
    }

    #[inline(always)]
    pub fn as_function(self) -> *const Function {
        (self.0 & !TYPE_MASK) as *const Function
    }

    #[inline(always)]
    pub fn as_string<'a>(self) -> &'a String {
        unsafe { (*((self.0 & !TYPE_MASK) as *mut GcObject<String>)).get() }
    }

    #[inline(always)]
    pub fn as_vec<'a>(self) -> &'a mut Vec<Value> {
        unsafe { (*((self.0 & !TYPE_MASK) as *mut GcObject<Vec<Value>>)).get_mut() }
    }

    #[inline(always)]
    pub fn as_dict<'a>(self) -> &'a mut AHashMap<Value, Value> {
        unsafe { (*((self.0 & !TYPE_MASK) as *mut GcObject<AHashMap<Value, Value>>)).get_mut() }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tag() {
            TYPE_NIL => write!(f, "nil"),
            TYPE_NUMBER => write!(f, "{}", self.as_number()),
            TYPE_BOOLEAN => write!(f, "{}", self.as_boolean()),
            TYPE_FUNCTION => write!(f, "<function {:p}>", self.as_function()),
            TYPE_STRING => write!(f, "{:?}", self.as_string()),
            TYPE_VEC => f.debug_list().entries(self.as_vec().iter()).finish(),
            TYPE_DICT => f.debug_map().entries(self.as_dict().iter()).finish(),
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
