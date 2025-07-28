pub struct Environment<T> {
    pub stack: Vec<T>,
    pub scopes_start: Vec<usize>,
    pub function_start: usize,
}

impl<T> Environment<T> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            scopes_start: Vec::new(),
            function_start: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        let current = self.stack.len();

        self.scopes_start.push(current);
    }

    pub fn exit_scope(&mut self) {
        let start = self.scopes_start.pop().unwrap();

        while self.stack.len() > start {
            self.stack.pop();
        }
    }

    pub fn enter_function(&mut self) {
        self.function_start = self.stack.len();
        self.enter_scope();
    }

    pub fn exit_function(&mut self) {
        self.function_start = 0;
        self.exit_scope();
    }

    pub fn declare(&mut self, value: T) {
        self.stack.push(value);
    }
}
