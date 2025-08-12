use crate::frontend::syntax::r#type::Type;

use super::symbol::Symbol;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Environment {
    pub symbols: Vec<Symbol>,
    pub scopes_ptr: Vec<usize>,
    pub variable_offset: usize,
    pub next_id: AtomicUsize,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            symbols: Vec::new(),
            scopes_ptr: vec![0],
            variable_offset: 0,
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Environment {
    pub fn generate_id(&self) -> usize {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn enter_scope(&mut self) {
        let ptr = self.symbols.len();

        self.scopes_ptr.push(ptr);
    }

    pub fn exit_scope(&mut self) {
        let ptr = self.scopes_ptr.pop().unwrap();

        while self.symbols.len() > ptr {
            if let Some(Symbol::Variable { .. }) = self.symbols.last() {
                self.variable_offset -= 1;
            }

            self.symbols.pop();
        }
    }

    pub fn declare_variable(&mut self, name: String, type_annotation: Type) {
        let offset = self.variable_offset;
        let declaration = Symbol::variable(offset, name, type_annotation);

        self.variable_offset += 1;

        self.symbols.push(declaration);
    }

    pub fn declare_function(&mut self, name: String, type_annotation: Type) {
        let id = self.generate_id();
        let declaration = Symbol::function(id, name, type_annotation);

        self.symbols.push(declaration);
    }

    pub fn search_current_scope(&self, name_: &str) -> Option<&Symbol> {
        let ptr = *self.scopes_ptr.last().unwrap();

        let declaration = self.symbols[ptr..]
            .iter()
            .find(|declaration| match declaration {
                Symbol::Function { name, .. } => name == name_,
                Symbol::Variable { name, .. } => name == name_,
            });

        declaration
    }

    pub fn search(&self, name_: &str) -> Option<&Symbol> {
        let declaration = self
            .symbols
            .iter()
            .rev()
            .find(|declaration| match declaration {
                Symbol::Function { name, .. } => name == name_,
                Symbol::Variable { name, .. } => name == name_,
            });

        declaration
    }
}
