use super::{constant_pool::ConstantPool, instruction::Instruction};

#[derive(Default)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: ConstantPool,
    pub entry_ptr: Option<usize>,
}
