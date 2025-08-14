use super::{constant_pool::ConstantPool, instruction::Instruction};

#[derive(Default, Debug)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: ConstantPool,
    pub entry_point: Option<usize>,
}

impl Bytecode {
    pub fn set_entry_point(&mut self, instruction_ptr: usize) {
        self.entry_point = Some(instruction_ptr);
    }
}
