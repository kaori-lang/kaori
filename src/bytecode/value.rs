#[derive(Clone, Copy)]
pub union Value {
    number: f64,
    boolean: bool,
    instruction_index: usize,
}

impl Default for Value {
    fn default() -> Value {
        Value { boolean: false }
    }
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value { number: value }
    }

    pub fn boolean(value: bool) -> Value {
        Value { boolean: value }
    }

    pub fn instruction(instruction_index: usize) -> Value {
        Value { instruction_index }
    }

    pub unsafe fn as_number(self) -> f64 {
        unsafe { self.number }
    }

    pub unsafe fn as_boolean(self) -> bool {
        unsafe { self.boolean }
    }

    pub unsafe fn as_instruction_index(self) -> usize {
        unsafe { self.instruction_index }
    }
}
