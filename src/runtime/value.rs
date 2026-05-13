use crate::util::string_interner::StringIndex;

const QNAN: u64 = 0x7FFC_0000_0000_0000;
const PTR_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
const TAG_CLOSURE: u64 = QNAN | 0x0003_0000_0000_0000;
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
    pub fn is_number(self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    fn is_tag(self, tag: u64) -> bool {
        (self.0 & !PTR_MASK) == tag
    }

    pub fn is_closure(self) -> bool {
        self.is_tag(TAG_CLOSURE)
    }

    pub fn is_string(self) -> bool {
        self.is_tag(TAG_STRING)
    }

    pub fn is_vec(self) -> bool {
        self.is_tag(TAG_VEC)
    }

    pub fn is_dict(self) -> bool {
        self.is_tag(TAG_DICT)
    }

    pub fn tag(self) -> u64 {
        self.0 & !PTR_MASK
    }

    pub fn number(value: f64) -> Self {
        Self(value.to_bits())
    }

    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }

    pub fn string(index: StringIndex) -> Self {
        Self(TAG_STRING | (index.0 as u64))
    }

    pub fn closure(index: usize) -> Self {
        Self(TAG_CLOSURE | (index as u64))
    }

    pub fn dict(index: usize) -> Self {
        Self(TAG_DICT | (index as u64))
    }

    pub fn vec(index: usize) -> Self {
        Self(TAG_VEC | (index as u64))
    }

    pub fn as_index(self) -> usize {
        (self.0 & PTR_MASK) as usize
    }
}
