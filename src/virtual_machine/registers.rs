use crate::{bytecode::value::Value, cfg_ir::operand::Register};

pub struct Registers {
    registers: [Value; 1024],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: [Value::default(); 1024],
        }
    }

    pub fn get_value(&self, register: Register) -> Value {
        self.registers[usize::from(register.0)]
    }

    pub fn set_value(&mut self, register: Register, value: Value) {
        self.registers[usize::from(register.0)] = value;
    }
}
