use crate::util::string_interner::Symbol;

pub const NANISH: u64 = 0b0_111111111111100_00_0000000000000000000000000000000000000000000000;
pub const PTR_MASK: u64 = 0b0_000000000000000_00_1111111111111111111111111111111111111111111111;
pub const TAG_MASK: u64 = 0b1_111111111111111_11_0000000000000000000000000000000000000000000000;
pub const BOOL_MASK: u64 = 0b1_111111111111111_11_0000000000000000000000000000000000000000000001;

pub const TAG_NIL: u64 = 0b1_111111111111100_00_0000000000000000000000000000000000000000000000;
pub const TAG_FALSE: u64 = 0b1_111111111111101_00_0000000000000000000000000000000000000000000000;
pub const TAG_TRUE: u64 = 0b1_111111111111101_00_0000000000000000000000000000000000000000000001;
pub const TAG_CLOSURE: u64 = 0b1_111111111111110_00_0000000000000000000000000000000000000000000000;
pub const TAG_STRING: u64 = 0b1_111111111111111_00_0000000000000000000000000000000000000000000000;
pub const TAG_MAP: u64 = 0b0_111111111111101_00_0000000000000000000000000000000000000000000000;
pub const TAG_VEC: u64 = 0b0_111111111111110_00_0000000000000000000000000000000000000000000000;
pub const TAG_CELL: u64 = 0b0_111111111111111_00_0000000000000000000000000000000000000000000000;
pub const TAG_FUNCTION: u64 = 0b1_111111111111100_10_0000000000000000000000000000000000000000000000;
pub const TAG_NATIVE: u64 = 0b1_111111111111101_10_0000000000000000000000000000000000000000000000;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct Value(pub u64);

impl Default for Value {
    fn default() -> Self {
        Value::nil()
    }
}

#[allow(clippy::should_implement_trait)]
impl Value {
    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }
    pub fn nil() -> Self {
        Self(TAG_NIL)
    }
    pub fn bool(value: bool) -> Self {
        if value {
            Self(TAG_TRUE)
        } else {
            Self(TAG_FALSE)
        }
    }
    pub fn string(index: Symbol) -> Self {
        Self(TAG_STRING | index.0 as u64)
    }
    pub fn closure(index: usize) -> Self {
        Self(TAG_CLOSURE | index as u64)
    }
    pub fn function(index: usize) -> Self {
        Self(TAG_FUNCTION | index as u64)
    }
    pub fn native(index: usize) -> Self {
        Self(TAG_NATIVE | index as u64)
    }
    pub fn map(index: usize) -> Self {
        Self(TAG_MAP | index as u64)
    }
    pub fn vec(index: usize) -> Self {
        Self(TAG_VEC | index as u64)
    }
    pub fn cell(index: usize) -> Self {
        Self(TAG_CELL | index as u64)
    }

    pub fn is_number(self) -> bool {
        (self.0 & NANISH) != NANISH
    }
    pub fn is_nil(self) -> bool {
        self.0 == TAG_NIL
    }
    pub fn is_true(self) -> bool {
        self.0 == TAG_TRUE
    }
    pub fn is_false(self) -> bool {
        self.0 == TAG_FALSE
    }
    pub fn is_bool(self) -> bool {
        (self.0 & BOOL_MASK) == TAG_FALSE
    }
    pub fn is_closure(self) -> bool {
        self.is_tag(TAG_CLOSURE)
    }
    pub fn is_function(self) -> bool {
        self.is_tag(TAG_FUNCTION)
    }
    pub fn is_native(self) -> bool {
        self.is_tag(TAG_NATIVE)
    }
    pub fn is_string(self) -> bool {
        self.is_tag(TAG_STRING)
    }
    pub fn is_vec(self) -> bool {
        self.is_tag(TAG_VEC)
    }
    pub fn is_map(self) -> bool {
        self.is_tag(TAG_MAP)
    }
    pub fn is_cell(self) -> bool {
        self.is_tag(TAG_CELL)
    }

    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }
    pub fn as_bool(self) -> bool {
        self.0 == TAG_TRUE
    }
    pub fn as_index(self) -> usize {
        (self.0 & PTR_MASK) as usize
    }
    pub fn tag(self) -> u64 {
        self.0 & TAG_MASK
    }
    pub fn is_tag(self, tag: u64) -> bool {
        (self.0 & TAG_MASK) == tag
    }

    // OPS
    pub fn add(self, rhs: Self) -> f64 {
        self.as_number() + rhs.as_number()
    }
    pub fn sub(self, rhs: Self) -> f64 {
        self.as_number() - rhs.as_number()
    }
    pub fn mul(self, rhs: Self) -> f64 {
        self.as_number() * rhs.as_number()
    }
    pub fn div(self, rhs: Self) -> f64 {
        self.as_number() / rhs.as_number()
    }
    pub fn rem(self, rhs: Self) -> f64 {
        self.as_number() % rhs.as_number()
    }
    pub fn neg(self) -> f64 {
        -self.as_number()
    }

    pub fn equal(self, rhs: Self) -> bool {
        self == rhs
    }
    pub fn not_equal(self, rhs: Self) -> bool {
        self != rhs
    }
    pub fn less(self, rhs: Self) -> bool {
        self.as_number() < rhs.as_number()
    }
    pub fn less_equal(self, rhs: Self) -> bool {
        self.as_number() <= rhs.as_number()
    }
    pub fn greater(self, rhs: Self) -> bool {
        self.as_number() > rhs.as_number()
    }
    pub fn greater_equal(self, rhs: Self) -> bool {
        self.as_number() >= rhs.as_number()
    }

    pub fn not(self) -> bool {
        self.is_false()
    }
}
