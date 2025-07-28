pub struct Environment<T> {
    pub stack: Vec<T>,
    pub scopes_pointer: Vec<usize>,
    pub frame_pointer: usize,
}

impl<T> Environment<T> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            scopes_pointer: Vec::new(),
            frame_pointer: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        let current = self.stack.len();

        self.scopes_pointer.push(current);
    }

    pub fn exit_scope(&mut self) {
        let start = self.scopes_pointer.pop().unwrap();

        while self.stack.len() > start {
            self.stack.pop();
        }
    }

    pub fn enter_function(&mut self) {
        self.frame_pointer = self.stack.len();
        self.enter_scope();
    }

    pub fn exit_function(&mut self) {
        self.frame_pointer = 0;
        self.exit_scope();
    }

    pub fn declare(&mut self, value: T) {
        self.stack.push(value);
    }
}
