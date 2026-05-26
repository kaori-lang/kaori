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
    pub fn allocate_local(&mut self) -> Register {
        Register::Local(self.pop())
    }

    pub fn allocate_temp(&mut self) -> Register {
        Register::Temp(self.pop())
    }

    fn pop(&mut self) -> u8 {
        self.registers.pop().expect("exceeded register limit").0
    }

    pub fn free_temp(&mut self, register: Register) {
        if let Register::Temp(register) = register {
            self.registers.push(Reverse(register));
        }
    }

    pub fn free_local(&mut self, register: Register) {
        if let Register::Local(register) = register {
            self.registers.push(Reverse(register));
        }
    }
}
