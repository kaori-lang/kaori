const TAG_MASK: u64 = 0b11;
const TAG_NUMBER: u64 = 0b00;
const TAG_BOOLEAN: u64 = 0b01;
const TAG_FUNCTION: u64 = 0b10;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind {
    #[default]
    Number,
    Boolean,
    Function,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Value(u64);

impl Value {
    #[inline(always)]
    pub fn number(value: f64) -> Self {
        let bits = value.to_bits();

        debug_assert!(bits & TAG_MASK == 0, "f64 bits collide with tag");
        Self(bits | TAG_NUMBER)
    }

    #[inline(always)]
    pub fn boolean(value: bool) -> Self {
        Self((value as u64) << 2 | TAG_BOOLEAN)
    }

    #[inline(always)]
    pub fn function(value: usize) -> Self {
        Self((value as u64) << 2 | TAG_FUNCTION)
    }

    #[inline(always)]
    pub fn kind(self) -> ValueKind {
        match self.0 & TAG_MASK {
            TAG_NUMBER => ValueKind::Number,
            TAG_BOOLEAN => ValueKind::Boolean,
            TAG_FUNCTION => ValueKind::Function,
            _ => unreachable!(),
        }
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
    pub fn expect_function(self) -> usize {
        assert!(
            self.is_function(),
            "expected Function, got {:?}",
            self.kind()
        );
        self.as_function()
    }

    #[inline(always)]
    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0 & !TAG_MASK)
    }

    #[inline(always)]
    pub fn as_boolean(self) -> bool {
        (self.0 >> 2) != 0
    }

    #[inline(always)]
    pub fn as_function(self) -> usize {
        (self.0 >> 2) as usize
    }
}
