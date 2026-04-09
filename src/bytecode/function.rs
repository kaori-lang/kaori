use super::instruction::Instruction;

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constant_pool: Vec<Constant>,
}

impl Function {
    pub fn new(
        instructions: Vec<Instruction>,
        registers_count: u8,
        constant_pool: Vec<Constant>,
    ) -> Self {
        Self {
            instructions,
            registers_count,
            constant_pool,
        }
    }
}

pub enum Constant {
    String(String),
    Number(f64),
    Boolean(bool),
    Function(usize),
}
