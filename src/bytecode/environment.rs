use crate::util::string_interner::Symbol;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    Temp(u8),
    Local(u8),
    Nil,
}

#[derive(Default)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    scopes: Vec<Vec<(Symbol, Register)>>,
    registers: BinaryHeap<Reverse<u8>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![Vec::new()],
            registers: (0..=255).map(Reverse).collect(),
        }
    }
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![Vec::new()],
            registers: (0..=255).map(Reverse).collect(),
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
    pub fn declare_local(&mut self, name: Symbol) -> Register {
        let register = self.allocate_local();

        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .push((name, register));

        register
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

    pub fn allocate_local(&mut self) -> Register {
        Register::Local(self.pop())
    }

    pub fn allocate_temp(&mut self) -> Register {
        Register::Temp(self.pop())
    }
    fn pop(&mut self) -> u8 {
        self.registers.pop().expect("exceeded register limit").0
    }
    pub fn free_temp(&mut self, register: Register) {
        if let Register::Temp(r) = register {
            self.registers.push(Reverse(r));
        }
    }
    pub fn free_local(&mut self, register: Register) {
        if let Register::Local(r) = register {
            self.registers.push(Reverse(r));
        }
    }
}
