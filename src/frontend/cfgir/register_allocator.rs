use super::register::Register;

#[derive(Debug, Default)]
pub struct RegisterAllocator {
    register_stack: Vec<Register>,
}

impl RegisterAllocator {
    pub fn enter_scope(&mut self) {
        let register = self.register_stack.last().unwrap();
        self.register_stack.push(*register);
    }

    pub fn exit_scope(&mut self) {
        self.register_stack.pop();
    }

    pub fn enter_function(&mut self) {
        let register = Register::new(0);

        self.register_stack.push(register);
    }

    pub fn exit_function(&mut self) {
        self.register_stack.pop();
    }

    pub fn create_register(&mut self) -> Register {
        let register = self.register_stack.last().unwrap();

        *register
    }
}
