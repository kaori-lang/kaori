use super::resolution::Resolution;

pub struct Environment<T> {
    pub declarations: Vec<T>,
    pub scopes_pointer: Vec<usize>,
    pub frame_pointer: usize,
}

impl<T> Environment<T> {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            scopes_pointer: Vec::new(),
            frame_pointer: 0,
        }
    }

    pub fn get(&mut self, resolution: Resolution) -> &T {
        let mut index = self.frame_pointer + resolution.offset;

        if resolution.global {
            index -= self.frame_pointer;
        }

        return self.declarations.get(index).unwrap();
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

    pub fn enter_function(&mut self) {
        self.frame_pointer = self.declarations.len();
        self.enter_scope();
    }

    pub fn exit_function(&mut self) {
        self.frame_pointer = 0;
        self.exit_scope();
    }

    pub fn declare(&mut self, value: T) {
        self.declarations.push(value);
    }
}
