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

    pub fn allocate_register(&mut self) -> Operand {
        for index in 0..self.registers.len() {
            if self.registers[index] {
                self.registers[index] = true;
                return Operand::Register(index as u8);
            }
        }

        panic!("Exceed limited of registers")
    }

    pub fn free_register(&mut self, index: usize) {
        self.registers[index] = false;
    }
}
