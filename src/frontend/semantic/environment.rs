use super::{resolved_ty::ResolvedTy, symbol::Symbol};

pub struct Environment {
    pub globals: Vec<Symbol>,
    pub locals: Vec<Symbol>,
    pub locals_scopes_ptr: Vec<usize>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            globals: Vec::new(),
            locals: Vec::new(),
            locals_scopes_ptr: vec![0],
        }
    }
}

impl Environment {
    pub fn enter_scope(&mut self) {
        let ptr = self.locals.len();

        self.locals_scopes_ptr.push(ptr);
    }

    pub fn exit_scope(&mut self) {
        let ptr = self.locals_scopes_ptr.pop().unwrap();

        while self.locals.len() > ptr {
            self.locals.pop();
        }
    }

    pub fn declare_local_variable(&mut self, name: String, ty: ResolvedTy) -> usize {
        let offset = self.locals.len();
        let declaration = Symbol::new(offset, name, ty);

        self.locals.push(declaration);

        offset
    }

    pub fn declare_global_variable(&mut self, name: String, ty: ResolvedTy) -> usize {
        let offset = self.globals.len();
        let declaration = Symbol::new(offset, name, ty);

        self.locals.push(declaration);

        offset
    }

    pub fn search_current_scope(&self, name_: &str) -> Option<&Symbol> {
        let ptr = *self.locals_scopes_ptr.last().unwrap();

        self.locals[ptr..]
            .iter()
            .find(|symbol| symbol.name == name_)
    }

    pub fn search(&self, name_: &str) -> Option<&Symbol> {
        self.locals.iter().rev().find(|symbol| symbol.name == name_)
    }
}
