pub struct Environment<T> {
    pub declarations: Vec<T>,
    pub scopes_pointer: Vec<usize>,
    pub frame_pointer: usize,
}

impl<T> Default for Environment<T> {
    fn default() -> Self {
        Self {
            declarations: Vec::new(),
            scopes_pointer: vec![0],
            frame_pointer: 0,
        }
    }
}

impl<T> Environment<T> {
    pub fn get(&mut self, offset: usize) -> &T {
        let index = self.frame_pointer + offset;

        self.declarations.get(index).unwrap()
    }

    pub fn enter_scope(&mut self) {
        let current = self.declarations.len();

        self.scopes_pointer.push(current);
    }

    pub fn exit_scope(&mut self) {
        let start = self.scopes_pointer.pop().unwrap();

        while self.declarations.len() > start {
            self.declarations.pop();
        }
    }

    pub fn declare(&mut self, value: T) {
        self.declarations.push(value);
    }

    fn search_current_scope(&mut self, name: &str) {
        let mut start = self.environment.declarations.len();
        let end = *self.environment.scopes_pointer.last().unwrap();

        while start > end {
            start -= 1;

            if name == self.environment.declarations[start] {
                let global =
                    self.environment.frame_pointer == 0 || start < self.environment.frame_pointer;
                let mut offset = start;

                if !global {
                    offset = start - self.environment.frame_pointer;
                }
            }
        }
    }

    fn search(&mut self, name: &str) {
        let mut start = self.environment.declarations.len();
        let end: usize = 0;

        while start > end {
            start -= 1;

            if name == self.environment.declarations[start] {
                let global =
                    self.environment.frame_pointer == 0 || start < self.environment.frame_pointer;
                let mut offset = start;

                if !global {
                    offset = start - self.environment.frame_pointer;
                }
            }
        }
    }
}
