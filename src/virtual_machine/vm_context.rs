use crate::bytecode::{function::Function, value::Value};

pub struct FunctionFrame {
    pub size: u8,
    pub registers_ptr: *mut Value,
    pub constants_ptr: *const Value,
    pub return_address: *const u16,
    pub return_register: u16,
}

impl FunctionFrame {
    pub fn new(
        size: u8,
        registers_ptr: *mut Value,
        constants_ptr: *const Value,
        return_address: *const u16,
        return_register: u16,
    ) -> Self {
        Self {
            size,
            registers_ptr,
            constants_ptr,
            return_address,
            return_register,
        }
    }
}

pub struct VMContext<'a> {
    pub functions: &'a [Function],
    pub call_stack: Vec<FunctionFrame>,
    pub registers: Vec<Value>,
    pub registers_ptr: *mut Value,
    pub constants_ptr: *const Value,
}

impl<'a> VMContext<'a> {
    pub fn new(
        functions: &'a [Function],
        registers: Vec<Value>,
        registers_ptr: *mut Value,
        constants_ptr: *const Value,
        main_frame: FunctionFrame,
    ) -> Self {
        Self {
            functions,
            call_stack: vec![main_frame],
            registers,
            registers_ptr,
            constants_ptr,
        }
    }

    #[inline(always)]
    pub fn get_constant_value(&self, index: u16) -> Value {
        unsafe { *self.constants_ptr.add(index as usize) }
    }

    #[inline(always)]
    pub fn get_register_value(&self, index: u16) -> Value {
        unsafe { *self.registers_ptr.add(index as usize) }
    }

    #[inline(always)]
    pub fn set_value(&mut self, index: u16, value: Value) {
        unsafe {
            *self.registers_ptr.add(index as usize) = value;
        }
    }

    #[inline(always)]
    pub fn pop_frame(&mut self) -> FunctionFrame {
        let frame = unsafe { self.call_stack.pop().unwrap_unchecked() };

        if let Some(frame) = self.call_stack.last() {
            self.registers_ptr = frame.registers_ptr;
            self.constants_ptr = frame.constants_ptr;
        }

        frame
    }

    #[inline(always)]
    pub fn push_frame(
        &mut self,
        return_register: u16,
        return_address: *const u16,
        frame_size: u8,
        constants_ptr: *const Value,
    ) {
        let size = self.call_stack.last().unwrap().size;

        let registers_ptr = unsafe { self.registers_ptr.add(size as usize) };

        let frame = FunctionFrame::new(
            frame_size,
            registers_ptr,
            constants_ptr,
            return_address,
            return_register,
        );

        self.registers_ptr = registers_ptr;
        self.constants_ptr = constants_ptr;

        self.call_stack.push(frame);
    }
}
