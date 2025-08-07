use crate::backend::vm::value::Value;

#[derive(Default)]
pub struct ConstantPool {
    constants: Vec<Value>,
}

impl ConstantPool {
    pub fn add_constant(&mut self, other: Value) -> isize {
        for (index, current) in self.constants.iter().enumerate() {
            if *current == other {
                return index as isize;
            }
        }

        let index = self.constants.len();

        self.constants.push(other);

        index as isize
    }

    pub fn get_constant(&self, offset: u16) -> Value {
        unsafe { self.constants.get_unchecked(offset as usize).to_owned() }
    }
}
