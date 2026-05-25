use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    Temp(u8),
    Local(u8),
}

pub struct RegisterAllocator {
    registers: BinaryHeap<Reverse<u8>>,
}

impl Default for RegisterAllocator {
    fn default() -> Self {
        Self {
            registers: (0..=255).map(Reverse).collect(),
        }
    }
}
impl RegisterAllocator {
    pub fn allocate_register(&mut self) -> Register {
        if let Some(Reverse(register)) = self.registers.pop() {
            Register::Local(register)
        } else {
            panic!("Exceeed the amount of registers per function")
        }
    }

    pub fn allocate_temporary_register(&mut self) -> Register {
        if let Some(Reverse(register)) = self.registers.pop() {
            Register::Temp(register)
        } else {
            panic!("Exceeed the amount of registers per function")
        }
    }

    pub fn free_temporary_register(&mut self, register: Register) {
        if let Register::Temp(register) = register {
            self.registers.push(Reverse(register));
        }
    }
}
