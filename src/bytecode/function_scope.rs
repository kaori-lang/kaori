use std::collections::HashMap;

use crate::{
    bytecode::{instruction::Instruction, operand::Operand},
    util::string_interner::StringIndex,
};

pub struct FunctionScope {
    pub block_scopes: Vec<HashMap<StringIndex, Operand>>,
    pub instructions: Vec<Instruction>,
    pub registers: [bool; 256],
    pub last_register: u8,
}

impl Default for FunctionScope {
    fn default() -> Self {
        Self {
            block_scopes: Vec::new(),
            instructions: Vec::new(),
            registers: [false; 256],
            last_register: 0,
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
            let register = symbol.unwrap_register();
            self.free_register(register);
        }
    }

    pub fn insert_symbol(&mut self, name: StringIndex, register: u8) {
        let symbol = Operand::Register(register);

        self.block_scopes.last_mut().unwrap().insert(name, symbol);
    }

    pub fn lookup_symbol(&self, name: StringIndex) -> Option<Operand> {
        self.block_scopes
            .iter()
            .rev()
            .find_map(|table| table.get(&name).copied())
    }

    pub fn allocate_register(&mut self) -> u8 {
        for index in 0..self.registers.len() {
            if !self.registers[index] {
                self.last_register = u8::max(index as u8, self.last_register);
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
