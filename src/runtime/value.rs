use crate::program::INTERNER;

const QNAN: u64 = 0x7FFC_0000_0000_0000;
const PTR_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
const TAG_FUNCTION: u64 = QNAN | 0x0003_0000_0000_0000;
const TAG_STRING: u64 = QNAN | 0x0004_0000_0000_0000;
const TAG_DICT: u64 = QNAN | 0x0005_0000_0000_0000;
const TAG_VEC: u64 = QNAN | 0x0006_0000_0000_0000;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct Value(u64);

impl Default for Value {
    fn default() -> Self {
        Value::number(0.0)
    }
}

impl Value {
    #[inline(always)]
    pub fn is_number(self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    #[inline(always)]
    fn is_tag(self, tag: u64) -> bool {
        (self.0 & !PTR_MASK) == tag
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
    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }

    #[inline(always)]
    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }

    #[inline(always)]
    pub fn is_truthy(self) -> bool {
        self.is_number() && self.as_number() != 0.0
    }

    #[inline(always)]
    pub fn string(index: usize) -> Self {
        Self(TAG_STRING | (index as u64))
    }

    #[inline(always)]
    pub fn function(index: usize) -> Self {
        Self(TAG_FUNCTION | (index as u64))
    }

    #[inline(always)]
    pub fn dict(index: usize) -> Self {
        Self(TAG_DICT | (index as u64))
    }

    #[inline(always)]
    pub fn vec(index: usize) -> Self {
        Self(TAG_VEC | (index as u64))
    }

    pub fn as_string(self) -> &'static str {
        INTERNER
            .lock()
            .unwrap()
            .resolve((self.0 & PTR_MASK) as usize)
    }

    pub fn as_index(self) -> usize {
        (self.0 & PTR_MASK) as usize
    }
}
