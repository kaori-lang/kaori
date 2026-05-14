use crate::util::string_interner::StringIndex;

#[derive(Default)]
pub struct Scope {
    pub names: Vec<(StringIndex, u8)>,
    pub scopes: Vec<usize>,
}

impl Scope {
    pub fn enter_scope(&mut self) {
        self.scopes.push(self.names.len());
    }

    pub fn exit_scope(&mut self) {
        let size = self.scopes.pop().unwrap();
        self.names.truncate(size);
    }

    pub fn insert_symbol(&mut self, name: StringIndex, register: u8) {
        self.names.push((name, register));
    }

    pub fn lookup(&self, name: StringIndex) -> Option<u8> {
        for (found_name, register) in self.names.iter().copied().rev() {
            if found_name == name {
                return Some(register);
            }
        }

        None
    }
}
