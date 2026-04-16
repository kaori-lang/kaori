use ahash::AHashMap;
use std::fmt;

use super::function::Function;
use super::gc::GcObject;

const QNAN: u64 = 0x7FFC_0000_0000_0000;
const PTR_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

pub const TAG_NIL: u64 = QNAN | 0x0001_0000_0000_0000;
pub const TAG_BOOL: u64 = QNAN | 0x0002_0000_0000_0000;
pub const TAG_FUNCTION: u64 = QNAN | 0x0003_0000_0000_0000;
pub const TAG_STRING: u64 = QNAN | 0x0004_0000_0000_0000;
pub const TAG_DICT: u64 = QNAN | 0x0005_0000_0000_0000;
pub const TAG_VEC: u64 = QNAN | 0x0006_0000_0000_0000;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Value(u64);

impl Default for Value {
    fn default() -> Self {
        Value::nil()
    }
}

impl Value {
    #[inline(always)]
    pub fn is_nan(self) -> bool {
        (self.0 & QNAN) == QNAN
    }

    #[inline(always)]
    pub fn is_number(self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    #[inline(always)]
    fn is_tag(self, tag: u64) -> bool {
        (self.0 & !PTR_MASK) == tag
    }

    #[inline(always)]
    pub fn is_nil(self) -> bool {
        self.0 == TAG_NIL
    }

    #[inline(always)]
    pub fn is_boolean(self) -> bool {
        self.is_tag(TAG_BOOL)
    }

    #[inline(always)]
    pub fn is_function(self) -> bool {
        self.is_tag(TAG_FUNCTION)
    }

    #[inline(always)]
    pub fn is_string(self) -> bool {
        self.is_tag(TAG_STRING)
    }

    #[inline(always)]
    pub fn is_vec(self) -> bool {
        self.is_tag(TAG_VEC)
    }

    #[inline(always)]
    pub fn is_dict(self) -> bool {
        self.is_tag(TAG_DICT)
    }

    #[inline(always)]
    pub fn tag(self) -> u64 {
        self.0 & !PTR_MASK
    }

    #[inline(always)]
    pub fn nil() -> Self {
        Self(TAG_NIL)
    }

    #[inline(always)]
    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }

    #[inline(always)]
    pub fn boolean(value: bool) -> Self {
        Self(TAG_BOOL | value as u64)
    }

    #[inline(always)]
    pub fn function(ptr: *const Function) -> Self {
        Self(TAG_FUNCTION | (ptr as u64 & PTR_MASK))
    }

    #[inline(always)]
    pub fn string(ptr: *mut GcObject<String>) -> Self {
        Self(TAG_STRING | (ptr as u64 & PTR_MASK))
    }

    #[inline(always)]
    pub fn dict(ptr: *mut GcObject<AHashMap<Value, Value>>) -> Self {
        Self(TAG_DICT | (ptr as u64 & PTR_MASK))
    }

    #[inline(always)]
    pub fn vec(ptr: *mut GcObject<Vec<Value>>) -> Self {
        Self(TAG_VEC | (ptr as u64 & PTR_MASK))
    }

    #[inline(always)]
    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }

    #[inline(always)]
    pub fn as_boolean(self) -> bool {
        self.0 & 1 != 0
    }

    #[inline(always)]
    pub fn as_function(self) -> *const Function {
        (self.0 & PTR_MASK) as *const Function
    }

    #[inline(always)]
    pub fn as_string<'a>(self) -> &'a String {
        unsafe { (*((self.0 & PTR_MASK) as *mut GcObject<String>)).get() }
    }

    #[inline(always)]
    pub fn as_vec<'a>(self) -> &'a mut Vec<Value> {
        unsafe { (*((self.0 & PTR_MASK) as *mut GcObject<Vec<Value>>)).get_mut() }
    }

    #[inline(always)]
    pub fn as_dict<'a>(self) -> &'a mut AHashMap<Value, Value> {
        unsafe { (*((self.0 & PTR_MASK) as *mut GcObject<AHashMap<Value, Value>>)).get_mut() }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_number() {
            return write!(f, "{}", self.as_number());
        }
        if self.is_nil() {
            return write!(f, "nil");
        }
        if self.is_boolean() {
            return write!(f, "{}", self.as_boolean());
        }
        if self.is_function() {
            return write!(f, "Function({:p})", self.as_function());
        }
        if self.is_string() {
            return write!(f, "{:?}", self.as_string());
        }
        if self.is_vec() {
            return write!(f, "{:?}", self.as_vec());
        }
        if self.is_dict() {
            return write!(f, "{:?}", self.as_dict());
        }
        unsafe { std::hint::unreachable_unchecked() }
    }
}
