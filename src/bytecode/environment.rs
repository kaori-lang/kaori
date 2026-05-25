use crate::{bytecode::register_allocator::Register, util::string_interner::Symbol};

#[derive(Clone, Copy)]
pub struct Local {
    pub name: Symbol,
    pub register: Register,
    pub kind: LocalKind,
}

#[derive(Clone, Copy)]
pub enum LocalKind {
    Variable,
    Mut,
    Constant,
}

#[derive(Default)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    scopes: Vec<Vec<Local>>,
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

    pub fn insert_local(&mut self, local: Local) {
        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .push(local);
    }

    pub fn lookup(&self, name: Symbol) -> Option<Local> {
        for scope in self.scopes.iter().rev() {
            for local in scope.iter().copied().rev() {
                if local.name == name {
                    return Some(local);
                }
            }
        }

        if let Some(parent) = &self.parent {
            if let Some(mut local) = parent.lookup(name) {
                if let LocalKind::Variable = local.kind {
                    local.kind = LocalKind::Constant;
                }
                Some(local)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn lookup_in_parent(&self, name: Symbol) -> Option<Local> {
        self.parent.as_ref()?.lookup(name)
    }
}
