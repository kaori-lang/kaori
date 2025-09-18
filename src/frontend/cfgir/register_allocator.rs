use super::register::Register;

#[derive(Debug)]
pub struct RegisterAllocator {
    registers: [bool; 256],
}

impl RegisterAllocator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registers: [true; 256],
        }
    }

    pub fn free_all_registers(&mut self) {
        for i in 0..self.registers.len() {
            self.registers[i] = true;
        }
    }

    pub fn is_register_free(&self, register: usize) -> bool {
        self.registers[register]
    }

    pub fn free_register(&mut self, register: Register) {
        self.registers[register.0 as usize] = true;
    }

    pub fn alloc_register(&mut self) -> Register {
        for i in 0..self.registers.len() {
            if self.is_register_free(i) {
                self.registers[i] = false;

                return Register::new(i as u8);
            }
        }

        unreachable!()
    }

    pub fn max_allocated_register(&self) -> u8 {
        let mut max_register = 0;

        for i in (0..self.registers.len()).rev() {
            if !self.is_register_free(i) {
                max_register = i;
                break;
            }
        }

        max_register as u8
    }
}
