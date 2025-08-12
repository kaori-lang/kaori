use crate::frontend::syntax::declaration::Decl;

use super::declaration::Declaration;

pub struct Environment {
    pub declarations: Vec<Declaration>,
    pub scopes_ptr: Vec<usize>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            declarations: Vec::new(),
            scopes_ptr: Vec::new(),
        }
    }
}

impl Environment {
    pub fn enter_scope(&mut self) {
        let ptr = self.declarations.len();

        self.scopes_ptr.push(ptr);
    }

    pub fn exit_scope(&mut self) {
        let ptr = self.scopes_ptr.pop().unwrap();

        while self.declarations.len() > ptr {
            self.declarations.pop();
        }
    }

    pub fn declare(&mut self, declaration: Declaration) {
        self.declarations.push(declaration);
    }

    pub fn search_current_scope(&self, name_: &str) -> Option<&Declaration> {
        let ptr = *self.scopes_ptr.last().unwrap();

        let declaration = self.declarations[ptr..]
            .iter()
            .find(|declaration| match declaration {
                Declaration::Function { name, .. } => name == name_,
                Declaration::Variable { name, .. } => name == name_,
            });

        declaration
    }

    pub fn search(&self, name_: &str) -> Option<&Declaration> {
        let declaration = self
            .declarations
            .iter()
            .rev()
            .find(|declaration| match declaration {
                Declaration::Function { name, .. } => name == name_,
                Declaration::Variable { name, .. } => name == name_,
            });

        declaration
    }
}
