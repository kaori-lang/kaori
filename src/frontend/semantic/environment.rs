use crate::frontend::syntax::node_id::NodeId;

use super::{resolved_ty::ResolvedTy, symbol::Symbol};

pub struct Environment {
    pub symbols: Vec<Symbol>,
    pub scopes_ptr: Vec<usize>,
    pub local_offset: usize,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            symbols: Vec::new(),
            scopes_ptr: vec![0],
            local_offset: 0,
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
            if let Some(Symbol::Local { .. }) = self.symbols.last() {
                self.local_offset -= 1;
            }

            self.symbols.pop();
        }
    }

    pub fn declare_local(&mut self, name: String, ty: ResolvedTy) -> usize {
        let offset = self.local_offset;
        let declaration = Symbol::local(offset, name, ty);

        self.local_offset += 1;

        self.symbols.push(declaration);

        offset
    }

    pub fn declare_global(&mut self, id: NodeId, name: String, ty: ResolvedTy) {
        let declaration = Symbol::global(id, name, ty);

        self.symbols.push(declaration);
    }

    pub fn search_current_scope(&self, name_: &str) -> Option<&Symbol> {
        let ptr = *self.scopes_ptr.last().unwrap();

        self.symbols[ptr..]
            .iter()
            .find(|declaration| match declaration {
                Symbol::Global { name, .. } => name == name_,
                Symbol::Local { name, .. } => name == name_,
            })
    }

    pub fn search(&self, name_: &str) -> Option<&Symbol> {
        self.symbols
            .iter()
            .rev()
            .find(|declaration| match declaration {
                Symbol::Global { name, .. } => name == name_,
                Symbol::Local { name, .. } => name == name_,
            })
    }
}
