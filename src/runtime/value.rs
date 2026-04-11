use std::{collections::HashMap, rc::Rc};

use ahash::AHashMap;

use super::{function::Function, gc::GcObject};

const TAG_MASK: u64 = 0b111;

const TAG_NUMBER: u64 = 0b000;
const TAG_BOOLEAN: u64 = 0b001;
const TAG_FUNCTION: u64 = 0b010;
const TAG_STRING: u64 = 0b011;
const TAG_DICT: u64 = 0b100;
const TAG_VEC: u64 = 0b101;
const TAG_NIL: u64 = 0b110;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Number,
    Boolean,
    Function,
    String,
    Dict,
    Vec,
    Nil,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(u64);

impl Value {
    #[inline(always)]
    pub fn nil() -> Self {
        Self(TAG_NIL)
    }

    #[inline(always)]
    pub fn number(value: f64) -> Self {
        let bits = value.to_bits();
        let shifted = bits >> 3;
        Self((shifted << 3) | TAG_NUMBER)
    }

    #[inline(always)]
    pub fn boolean(value: bool) -> Self {
        Self(((value as u64) << 3) | TAG_BOOLEAN)
    }

    #[inline(always)]
    pub fn function(ptr: *const Function) -> Self {
        debug_assert!((ptr as u64) & TAG_MASK == 0);
        Self((ptr as u64) | TAG_FUNCTION)
    }

    #[inline(always)]
    pub fn string(ptr: *mut GcObject) -> Self {
        debug_assert!((ptr as u64) & TAG_MASK == 0, "GcObject pointer not aligned");
        Self((ptr as u64) | TAG_STRING)
    }

    #[inline(always)]
    pub fn dict(ptr: *mut GcObject) -> Self {
        debug_assert!((ptr as u64) & TAG_MASK == 0, "GcObject pointer not aligned");
        Self((ptr as u64) | TAG_DICT)
    }

    #[inline(always)]
    pub fn vec(ptr: *mut GcObject) -> Self {
        debug_assert!((ptr as u64) & TAG_MASK == 0, "GcObject pointer not aligned");
        Self((ptr as u64) | TAG_VEC)
    }

    #[inline(always)]
    pub fn kind(self) -> ValueKind {
        match self.0 & TAG_MASK {
            TAG_NUMBER => ValueKind::Number,
            TAG_BOOLEAN => ValueKind::Boolean,
            TAG_FUNCTION => ValueKind::Function,
            TAG_STRING => ValueKind::String,
            TAG_DICT => ValueKind::Dict,
            TAG_VEC => ValueKind::Vec,
            TAG_NIL => ValueKind::Nil,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    pub fn is_nil(self) -> bool {
        self.0 & TAG_MASK == TAG_NIL
    }

    #[inline(always)]
    pub fn is_number(self) -> bool {
        self.0 & TAG_MASK == TAG_NUMBER
    }

    #[inline(always)]
    pub fn is_boolean(self) -> bool {
        self.0 & TAG_MASK == TAG_BOOLEAN
    }

    #[inline(always)]
    pub fn is_function(self) -> bool {
        self.0 & TAG_MASK == TAG_FUNCTION
    }

    #[inline(always)]
    pub fn is_string(self) -> bool {
        self.0 & TAG_MASK == TAG_STRING
    }

    #[inline(always)]
    pub fn is_dict(self) -> bool {
        self.0 & TAG_MASK == TAG_DICT
    }

    #[inline(always)]
    pub fn is_vec(self) -> bool {
        self.0 & TAG_MASK == TAG_VEC
    }

    #[inline(always)]
    pub fn expect_number(self) -> f64 {
        assert!(self.is_number(), "expected Number, got {:?}", self.kind());
        self.as_number()
    }

    #[inline(always)]
    pub fn expect_boolean(self) -> bool {
        assert!(self.is_boolean(), "expected Boolean, got {:?}", self.kind());
        self.as_boolean()
    }

    #[inline(always)]
    pub fn expect_function(self) -> *const Function {
        assert!(
            self.is_function(),
            "expected Function, got {:?}",
            self.kind()
        );
        self.as_function()
    }

    #[inline(always)]
    pub fn expect_string(self) -> *const Rc<str> {
        assert!(self.is_string(), "expected String, got {:?}", self.kind());
        self.as_string()
    }

    #[inline(always)]
    pub fn expect_dict(self) -> *mut AHashMap<Value, Value> {
        assert!(self.is_dict(), "expected Dict, got {:?}", self.kind());
        self.as_dict()
    }

    #[inline(always)]
    pub fn expect_vec(self) -> *mut Vec<Value> {
        assert!(self.is_vec(), "expected Vec, got {:?}", self.kind());
        self.as_vec()
    }

    #[inline(always)]
    pub fn as_number(self) -> f64 {
        let bits = self.0 >> 3;
        f64::from_bits(bits << 3)
    }

    #[inline(always)]
    pub fn as_boolean(self) -> bool {
        (self.0 >> 3) != 0
    }

    #[inline(always)]
    pub fn as_function(self) -> *const Function {
        (self.0 & !TAG_MASK) as *const Function
    }

    #[inline(always)]
    pub fn as_gc_object(self) -> *mut GcObject {
        (self.0 & !TAG_MASK) as *mut GcObject
    }

    #[inline(always)]
    pub fn as_string(self) -> *const Rc<str> {
        unsafe { (*self.as_gc_object()).as_string() }
    }

    #[inline(always)]
    pub fn as_vec(self) -> *mut Vec<Value> {
        unsafe { (*self.as_gc_object()).as_vec() }
    }

    #[inline(always)]
    pub fn as_dict(self) -> *mut AHashMap<Value, Value> {
        unsafe { (*self.as_gc_object()).as_dict() }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            ValueKind::Nil => write!(f, "nil"),
            ValueKind::Number => write!(f, "{}", self.as_number()),
            ValueKind::Boolean => write!(f, "{}", self.as_boolean()),
            ValueKind::Function => write!(f, "<function {:p}>", self.as_function()),
            ValueKind::String => unsafe { write!(f, "{:?}", &*self.as_string()) },
            ValueKind::Vec => unsafe {
                let vec = &*self.as_vec();
                f.debug_list().entries(vec.iter()).finish()
            },
            ValueKind::Dict => unsafe {
                let map = &*self.as_dict();
                f.debug_map().entries(map.iter()).finish()
            },
        }
    }
}
