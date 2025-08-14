use super::{constant_pool::ConstantPool, instruction::Instruction};

#[derive(Default, Debug)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: ConstantPool,
}
