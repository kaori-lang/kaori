use crate::util::string_interner::Symbol;
use std::hash::{Hash, Hasher};

const TAG_NIL: u32 = 0xFFFF_0001;
const TAG_BOOL: u32 = 0xFFFF_0002;
const TAG_STRING: u32 = 0xFFFF_0003;
const TAG_CLOSURE: u32 = 0xFFFF_0004;
const TAG_MAP: u32 = 0xFFFF_0005;
const TAG_VEC: u32 = 0xFFFF_0006;
const TAG_CELL: u32 = 0xFFFF_0007;
const TAG_NATIVE_FUNCTION: u32 = 0xFFFF_0008;

#[derive(Clone, Copy)]
#[repr(C)]
pub union Value {
    float: f64,
    parts: (u32, u32), // (.0 = payload/low, .1 = tag/high)
    bits: u64,
}

impl Default for Value {
    fn default() -> Self {
        Self::nil()
    }
}

impl PartialEq for Value {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.bits == other.bits }
    }
}

impl Eq for Value {}

impl Hash for Value {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.bits.hash(state) }
    }
}

impl Value {
    #[inline(always)]
    pub fn number(value: f64) -> Self {
        Self { float: value }
    }

    #[inline(always)]
    pub fn nil() -> Self {
        Self { parts: (0, TAG_NIL) }
    }

    #[inline(always)]
    pub fn bool(value: bool) -> Self {
        Self { parts: (value as u32, TAG_BOOL) }
    }

    #[inline(always)]
    pub fn string(index: Symbol) -> Self {
        Self { parts: (index.0, TAG_STRING) }
    }

    #[inline(always)]
    pub fn closure(index: u32) -> Self {
        Self { parts: (index, TAG_CLOSURE) }
    }

    #[inline(always)]
    pub fn native_function(index: u32) -> Self {
        Self { parts: (index, TAG_NATIVE_FUNCTION) }
    }

    #[inline(always)]
    pub fn map(index: u32) -> Self {
        Self { parts: (index, TAG_MAP) }
    }

    #[inline(always)]
    pub fn vec(index: u32) -> Self {
        Self { parts: (index, TAG_VEC) }
    }

    #[inline(always)]
    pub fn cell(index: u32) -> Self {
        Self { parts: (index, TAG_CELL) }
    }

    #[inline(always)]
    pub fn is_number(&self) -> bool {
        unsafe { !self.float.is_nan() }
    }

    #[inline(always)]
    pub fn is_nil(&self) -> bool {
        unsafe { self.parts.1 == TAG_NIL }
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        unsafe { self.parts.1 == TAG_BOOL }
    }

    #[inline(always)]
    pub fn is_string(&self) -> bool {
        unsafe { self.parts.1 == TAG_STRING }
    }

    #[inline(always)]
    pub fn is_closure(&self) -> bool {
        unsafe { self.parts.1 == TAG_CLOSURE }
    }

    #[inline(always)]
    pub fn is_native_function(&self) -> bool {
        unsafe { self.parts.1 == TAG_NATIVE_FUNCTION }
    }

    #[inline(always)]
    pub fn is_map(&self) -> bool {
        unsafe { self.parts.1 == TAG_MAP }
    }

    #[inline(always)]
    pub fn is_vec(&self) -> bool {
        unsafe { self.parts.1 == TAG_VEC }
    }

    #[inline(always)]
    pub fn is_cell(&self) -> bool {
        unsafe { self.parts.1 == TAG_CELL }
    }

    #[inline(always)]
    pub fn is_true(&self) -> bool {
        unsafe { self.parts.0 == 1 }
    }

    #[inline(always)]
    pub fn is_false(&self) -> bool {
        unsafe { self.parts.0 == 0 }
    }

    #[inline(always)]
    pub fn as_number(&self) -> f64 {
        unsafe { self.float }
    }

    #[inline(always)]
    pub fn as_bool(&self) -> bool {
        unsafe { self.parts.0 != 0 }
    }

    #[inline(always)]
    pub fn as_string(&self) -> Symbol {
        unsafe { Symbol(self.parts.0) }
    }

    #[inline(always)]
    pub fn index(&self) -> u32 {
        unsafe { self.parts.0 }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_number() {
            write!(f, "{}", self.as_number())
        } else if self.is_nil() {
            write!(f, "Nil")
        } else if self.is_bool() {
            write!(f, "{}", self.as_bool())
        } else if self.is_string() {
            write!(f, "{:?}", self.as_string())
        } else if self.is_closure() {
            write!(f, "Closure({})", self.index())
        } else if self.is_native_function() {
            write!(f, "NativeFunction({})", self.index())
        } else if self.is_map() {
            write!(f, "Map({})", self.index())
        } else if self.is_vec() {
            write!(f, "Array({})", self.index())
        } else if self.is_cell() {
            write!(f, "Cell({})", self.index())
        } else {
            write!(f, "Unknown(0x{:016x})", unsafe { self.bits })
        }
    }
}
