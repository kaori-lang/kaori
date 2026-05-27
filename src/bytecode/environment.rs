use crate::{bytecode::register_allocator::Register, util::string_interner::Symbol};

#[derive(Default)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    scopes: Vec<Vec<(Symbol, Register)>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![Vec::new()],
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![Vec::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    pub fn pop_scope(&mut self) {
        assert!(
            self.scopes.len() > 1,
            "tried to pop a scope with empty array"
        );
        self.scopes.pop();
    }

    pub fn insert_local(&mut self, name: Symbol, register: Register) {
        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .push((name, register));
    }

    pub fn lookup(&self, name: Symbol) -> Option<(Symbol, Register)> {
        for scope in self.scopes.iter().rev() {
            for local in scope.iter().copied().rev() {
                if local.0 == name {
                    return Some(local);
                }
            }
        }

        if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }

    pub fn lookup_in_parent(&self, name: Symbol) -> Option<(Symbol, Register)> {
        self.parent.as_ref()?.lookup(name)
    }
}
