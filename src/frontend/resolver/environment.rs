pub struct Environment {
    pub variables: Vec<String>,
    pub functions: Vec<String>,
    pub scopes_ptr: Vec<(usize, usize)>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            variables: Vec::new(),
            functions: Vec::new(),
            scopes_ptr: Vec::new(),
        }
    }
}

impl Environment {
    pub fn enter_scope(&mut self) {
        let i = self.variables.len();
        let j = self.functions.len();

        self.scopes_ptr.push((i, j));
    }

    pub fn exit_scope(&mut self) {
        let (i, j) = self.scopes_ptr.pop().unwrap();

        while self.variables.len() > i {
            self.variables.pop();
        }

        while self.functions.len() > j {
            self.functions.pop();
        }
    }

    pub fn declare_variable(&mut self, value: String) {
        self.variables.push(value);
    }

    pub fn declare_function(&mut self, value: String) {
        self.functions.push(value);
    }

    pub fn is_variable_declared(&self, value: &String) -> bool {
        let (i, _) = *self.scopes_ptr.last().unwrap();
        self.variables[i..].contains(value)
    }

    pub fn is_function_declared(&self, value: &String) -> bool {
        let (_, j) = *self.scopes_ptr.last().unwrap();

        self.functions[j..].contains(value)
    }
}
