use crate::bytecode::{function::Function, instruction::Instruction, value::Value};

pub struct FunctionFrame {
    pub registers_count: u8,
    pub registers_ptr: *mut Value,
    pub constant_pool_ptr: *const Value,
    pub return_address: *const Instruction,
    pub return_register: u16,
}

impl FunctionFrame {
    pub fn new(
        registers_count: u8,
        registers_ptr: *mut Value,
        constant_pool_ptr: *const Value,
        return_address: *const Instruction,
        return_register: u16,
    ) -> Self {
        Self {
            registers_count,
            registers_ptr,
            constant_pool_ptr,
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
    pub constant_pool_ptr: *const Value,
}

impl<'a> VMContext<'a> {
    pub fn new(
        functions: &'a [Function],
        registers: Vec<Value>,
        registers_ptr: *mut Value,
        constant_pool_ptr: *const Value,
        main_frame: FunctionFrame,
    ) -> Self {
        Self {
            functions,
            call_stack: vec![main_frame],
            registers,
            registers_ptr,
            constant_pool_ptr,
        }
    }

    #[inline(always)]
    pub fn get_value(&self, index: i16) -> Value {
        unsafe {
            if index < 0 {
                *self.constant_pool_ptr.add(-(index + 1) as usize)
            } else {
                *self.registers_ptr.add(index as usize)
            }
        }
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
            self.constant_pool_ptr = frame.constant_pool_ptr;
        }

        frame
    }

    #[inline(always)]
    pub fn push_frame(
        &mut self,
        return_register: u16,
        return_address: *const Instruction,
        registers_count: u8,
        constant_pool_ptr: *const Value,
    ) {
        let registers_count = self.call_stack.last().unwrap().registers_count;

        let registers_ptr = unsafe { self.registers_ptr.add(registers_count as usize) };

        let frame = FunctionFrame::new(
            registers_count,
            registers_ptr,
            constant_pool_ptr,
            return_address,
            return_register,
        );

        self.registers_ptr = registers_ptr;
        self.constant_pool_ptr = constant_pool_ptr;

        self.call_stack.push(frame);
    }
}
