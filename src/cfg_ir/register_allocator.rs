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

    pub fn free_register(&mut self, register: usize) {
        self.registers[register] = true;
    }

    pub fn alloc_register(&mut self) -> usize {
        for i in 0..self.registers.len() {
            if self.is_register_free(i) {
                self.registers[i] = false;

                return i;
            }
        }

        unreachable!()
    }

    pub fn max_allocated_register(&self) -> usize {
        let mut max_register = 0;

        for i in (0..self.registers.len()).rev() {
            if !self.is_register_free(i) {
                max_register = i;
                break;
            }
        }

        max_register
    }
}
