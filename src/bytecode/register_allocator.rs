use core::panic;

pub struct RegisterAllocator {
    registers: [bool; 256],
    highest_register: u8,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            registers: [false; 256],
            highest_register: 0,
        }
    }

    pub fn get_highest_register(&self) -> u8 {
        self.highest_register
    }

    pub fn allocate_register(&mut self) -> u8 {
        for index in 0..self.registers.len() {
            if !self.registers[index] {
                self.registers[index] = true;
                self.highest_register = u8::max(self.highest_register, index as u8);
                return index as u8;
            }
        }

        panic!("Exceed limited of registers")
    }

    pub fn free_register(&mut self, index: u8) {
        self.registers[index as usize] = false;
    }
}
