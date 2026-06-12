use crate::{codegen::environment::Register, util::string_interner::Symbol};

pub enum Operand {
    Register(Register),
    Constant(Constant),
}

impl From<Register> for Operand {
    fn from(value: Register) -> Self {
        Operand::Register(value)
    }
}

pub enum Constant {
    String(Symbol),
    Number(f64),
    Nil,
    Boolean(bool),
}
