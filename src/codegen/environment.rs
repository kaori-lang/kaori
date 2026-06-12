use crate::util::string_interner::Symbol;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    Temp(usize),
    Local(usize),
}

#[derive(Default)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub locals: Vec<(Symbol, Register)>,
    pub scopes: Vec<usize>,
    pub registers: BinaryHeap<Reverse<usize>>,
    pub frame_size: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self { parent: None, locals: Vec::new(), scopes: vec![0], registers: (0..=255).map(Reverse).collect(), frame_size: 0 }
    }
    pub fn with_parent(parent: Environment) -> Self {
        Self { parent: Some(Box::new(parent)), locals: Vec::new(), scopes: vec![0], registers: (0..=255).map(Reverse).collect(), frame_size: 0 }
    }
    pub fn push_scope(&mut self) {
        let index = self.locals.len();

        self.scopes.push(index);
    }

    pub fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1, "tried to pop a scope with empty array");

        let index = self.scopes.pop().unwrap();

        while self.locals.len() > index {
            let (_, register) = self.locals.pop().unwrap();

            if let Register::Local(register) = register {
                self.registers.push(Reverse(register));
            }
        }
    }

    pub fn declare_local(&mut self, name: Symbol, register: Register) {
        self.locals.push((name, register));
    }

    pub fn lookup(&self, name: Symbol) -> Option<(Symbol, Register)> {
        for (symbol, register) in self.locals.iter().copied().rev() {
            if symbol == name {
                return Some((symbol, register));
            }
        }

        if let Some(parent) = &self.parent { parent.lookup(name) } else { None }
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

    fn pop(&mut self) -> usize {
        let register = self.registers.pop().expect("exceeded register limit").0;

        self.frame_size = (register + 1).max(self.frame_size);

        register
    }

    pub fn free_temp(&mut self, register: Register) {
        if let Register::Temp(register) = register {
            self.registers.push(Reverse(register));
        }
    }
}
