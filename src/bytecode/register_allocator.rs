use core::panic;

use crate::bytecode::operand::Operand;

pub struct RegisterAllocator {
    registers: [bool; 256],
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            registers: [false; 256],
        }
    }

    pub fn allocate_register(&mut self) -> u8 {
        for index in 0..self.registers.len() {
            if self.registers[index] {
                self.registers[index] = true;
                return index as u8;
            }
        }

        panic!("Exceed limited of registers")
    }

    pub fn free_register(&mut self, index: u8) {
        self.registers[index as usize] = false;
    }
}
