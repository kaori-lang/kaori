use crate::{bytecode::instruction::Instruction, util::string_interner::StringIndex};

#[derive(Default)]
pub struct FunctionScope {
    names: Vec<(StringIndex, u8)>,
    scopes: Vec<usize>,
    pub instructions: Vec<Instruction>,
    pub next_register: u8,
}

impl FunctionScope {
    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        index
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(self.names.len());
    }

    pub fn exit_scope(&mut self) {
        let size = self.scopes.pop().unwrap();
        self.names.truncate(size);
    }

    fn insert_symbol(&mut self, name: StringIndex) -> u8 {
        let register = self.allocate_register();

        self.names.push((name, register));

        register
    }

    pub fn lookup_or_declare(&mut self, name: StringIndex) -> u8 {
        for (found_name, register) in self.names.iter().copied().rev() {
            if found_name == name {
                return register;
            }
        }

        self.insert_symbol(name)
    }

    pub fn allocate_register(&mut self) -> u8 {
        let register = self.next_register;
        self.next_register += 1;
        register
    }
}
