use super::register::Register;

#[derive(Debug)]
pub struct RegisterAllocator {
    registers: [bool; 256],
    frame_stack: Vec<Register>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            registers: [true; 256],
            frame_stack: Vec::new(),
        }
    }
    pub fn push_scope(&mut self) {
        let cursor = self.frame_stack.last().unwrap();

        self.frame_stack.push(*cursor);
    }

    pub fn pop_scope(&mut self) {
        self.frame_stack.pop();
    }

    pub fn free_all_registers(&mut self) {
        for i in 0..self.registers.len() {
            self.registers[i] = false;
        }
    }

    pub fn is_register_free(&self, register: usize) -> bool {
        self.registers[register] == true
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
}
