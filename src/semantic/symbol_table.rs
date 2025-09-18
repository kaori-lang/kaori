use super::{hir_id::HirId, symbol::Symbol};

pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub scopes_ptr: Vec<usize>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            symbols: Vec::new(),
            scopes_ptr: vec![0],
        }
    }
}

impl SymbolTable {
    pub fn enter_scope(&mut self) {
        let ptr = self.symbols.len();

        self.scopes_ptr.push(ptr);
    }

    pub fn exit_scope(&mut self) {
        let ptr = self.scopes_ptr.pop().unwrap();

        while self.symbols.len() > ptr {
            self.symbols.pop();
        }
    }

    pub fn declare_variable(&mut self, id: HirId, name: String) {
        let symbol = Symbol::variable(id, name);

        self.symbols.push(symbol);
    }

    pub fn declare_function(&mut self, id: HirId, name: String) {
        let symbol = Symbol::function(id, name);

        self.symbols.push(symbol);
    }

    pub fn declare_struct(&mut self, id: HirId, name: String) {
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
