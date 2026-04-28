use ahash::{HashMap, HashMapExt};

use crate::bytecode::{instruction::Instruction, operand::Operand};

#[derive(Clone, Copy)]
pub enum Symbol {
    Function { register: u8, index: usize },
    Variable { register: u8 },
}

pub struct FunctionScope {
    pub block_scopes: Vec<HashMap<String, Symbol>>,
    pub constants: Vec<Constant>,
    pub instructions: Vec<Instruction>,
    pub registers: [bool; 256],
}

impl FunctionScope {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            instructions: Vec::new(),
            block_scopes: Vec::new(),
            registers: [false; 256],
        }
    }

    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        index
    }

    pub fn enter_scope(&mut self) {
        self.block_scopes.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        let registers = self
            .block_scopes
            .last()
            .unwrap()
            .values()
            .map(|symbol| match symbol {
                Symbol::Function { register, .. } => register,
                Symbol::Variable { register, .. } => register,
            })
            .copied()
            .collect::<Vec<u8>>();

        for register in registers {
            self.free_register(register);
        }

        self.block_scopes.pop();
    }

    pub fn insert_function_symbol(&mut self, name: &str, register: u8, index: usize) {
        let symbol = Symbol::Function { register, index };

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
