use crate::frontend::hir::node_id::NodeId;

use super::symbol::{Symbol, SymbolKind};

pub struct Environment {
    pub symbols: Vec<Symbol>,
    pub scopes_ptr: Vec<usize>,
    pub variable_offset: usize,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            symbols: Vec::new(),
            scopes_ptr: vec![0],
            variable_offset: 0,
        }
    }
}

impl Environment {
    pub fn enter_scope(&mut self) {
        let ptr = self.symbols.len();

        self.scopes_ptr.push(ptr);
    }

    pub fn exit_scope(&mut self) {
        let ptr = self.scopes_ptr.pop().unwrap();

        while self.symbols.len() > ptr {
            if let Some(symbol) = self.symbols.last()
                && let SymbolKind::Variable = symbol.kind
            {
                self.variable_offset -= 1;
            }

            self.symbols.pop();
        }
    }

    pub fn declare_variable(&mut self, id: NodeId, name: String) -> usize {
        let offset = self.variable_offset;
        let symbol = Symbol::variable(id, name);

        self.variable_offset += 1;

        self.symbols.push(symbol);

        offset
    }

    pub fn declare_function(&mut self, id: NodeId, name: String) {
        let symbol = Symbol::function(id, name);

        self.symbols.push(symbol);
    }

    pub fn declare_struct(&mut self, id: NodeId, name: String) {
        let symbol = Symbol::struct_(id, name);

        self.symbols.push(symbol);
    }

    pub fn search_current_scope(&self, name: &str) -> Option<&Symbol> {
        let ptr = *self.scopes_ptr.last().unwrap();

        self.symbols[ptr..]
            .iter()
            .find(|symbol| symbol.name == name)
    }

    pub fn search(&self, name: &str) -> Option<&Symbol> {
        self.symbols.iter().rev().find(|symbol| symbol.name == name)
    }
}
