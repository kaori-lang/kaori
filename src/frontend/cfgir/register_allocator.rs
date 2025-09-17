use super::register::Register;

#[derive(Debug, Default)]
pub struct RegisterAllocator {
    frame_stack: Vec<Register>,
}

impl RegisterAllocator {
    pub fn push_scope(&mut self) {
        let cursor = self.frame_stack.last().unwrap();

        self.frame_stack.push(*cursor);
    }

    pub fn pop_scope(&mut self) {
        self.frame_stack.pop();
    }

    pub fn enter_frame(&mut self) {
        let cursor = Register::new(0);
        self.frame_stack.push(cursor);
    }

    pub fn exit_frame(&mut self) {
        self.frame_stack.pop();
    }

    pub fn alloc_register(&mut self) -> Register {
        let register = self.frame_stack.last_mut().unwrap();
        let allocated = *register;

        register.increment();

        allocated
    }
}
