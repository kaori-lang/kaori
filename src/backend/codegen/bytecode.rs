use super::{constant_pool::ConstantPool, instruction::Instruction};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constant_pool: ConstantPool,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>, constant_pool: ConstantPool) -> Self {
        Self {
            instructions,
            constant_pool,
        }
    }
}
