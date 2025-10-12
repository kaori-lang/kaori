use crate::bytecode::{instruction::Instruction, value::Value};

pub struct FunctionFrame {
    pub registers: *mut Value,
    pub return_address: *const Instruction,
    pub return_register: i16,
}

impl FunctionFrame {
    pub fn new(
        registers: *mut Value,
        return_address: *const Instruction,
        return_register: i16,
    ) -> Self {
        Self {
            registers,
            return_address,
            return_register,
        }
    }
}
