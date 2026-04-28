use ahash::{HashMap, HashMapExt};

use crate::bytecode::{instruction::Instruction, operand::Operand};

#[derive(Clone, Copy)]
pub enum Symbol {
    Function { index: usize },
    Variable { register: u8 },
    Closure { register: u8, index: usize },
}

pub struct FunctionScope {
    pub block_scopes: Vec<HashMap<String, Symbol>>,
    pub constants: Vec<Constant>,
    pub instructions: Vec<Instruction>,
    pub registers: [bool; 256],
    pub size: u8,
}

impl Default for FunctionScope {
    fn default() -> Self {
        Self {
            block_scopes: Vec::new(),
            constants: Vec::new(),
            instructions: Vec::new(),
            registers: [false; 256],
            size: 0,
        }
    }
}

impl FunctionScope {
    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        index
    }

    pub fn enter_scope(&mut self) {
        self.block_scopes.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        let scope = self.block_scopes.pop().unwrap();

        for symbol in scope.values().copied() {
            match symbol {
                Symbol::Variable { register } => self.free_register(register),
                Symbol::Closure { register, index } => self.free_register(register),
                _ => {}
            }
        }
    }

    pub fn insert_function_symbol(&mut self, name: &str, index: usize) {
        let symbol = Symbol::Function { index };

        self.block_scopes
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), symbol);
    }

    pub fn insert_variable_symbol(&mut self, name: &str, register: u8) {
        let symbol = Symbol::Variable { register };

        self.block_scopes
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), symbol);
    }

    pub fn insert_closure_symbol(&mut self, name: &str, register: u8, index: usize) {
        let symbol = Symbol::Closure { register, index };

        self.block_scopes
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), symbol);
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<Symbol> {
        self.block_scopes
            .iter()
            .rev()
            .find_map(|table| table.get(name).copied())
    }

    fn push_constant(&mut self, constant: Constant) -> Operand {
        let index = if let Some(index) = self.constants.iter().position(|c| *c == constant) {
            index
        } else {
            let index = self.constants.len();
            assert!(index < 256, "constant pool overflow (u8)");

            self.constants.push(constant);

            index
        };

        Operand::Constant(index as u8)
    }

    pub fn push_function_index(&mut self, value: usize) -> Operand {
        self.push_constant(Constant::FunctionIndex(value))
    }

    pub fn push_string(&mut self, value: String) -> Operand {
        self.push_constant(Constant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> Operand {
        self.push_constant(Constant::Number(value))
    }

    pub fn allocate_register(&mut self) -> u8 {
        for index in 0..self.registers.len() {
            if !self.registers[index] {
                self.size = u8::max(index as u8, self.size);
                self.registers[index] = true;
                return index as u8;
            }
        }

        panic!("Exceed limited of registers")
    }

    pub fn free_register(&mut self, index: u8) {
        self.registers[index as usize] = false;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    String(String),
    Number(f64),
    FunctionIndex(usize),
}
