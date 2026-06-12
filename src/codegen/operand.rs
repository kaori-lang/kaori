use crate::{codegen::environment::Register, diagnostics::error::Error, runtime::operands::Reg, util::string_interner::Symbol};

pub enum Operand {
    Register(Register),
    Constant(Constant),
}

impl From<Register> for Operand {
    fn from(value: Register) -> Self {
        Operand::Register(value)
    }
}

impl From<Constant> for Operand {
    fn from(value: Constant) -> Self {
        Operand::Constant(value)
    }
}

pub enum Constant {
    String(Symbol),
    Number(f64),
    Boolean(bool),
    Nil,
}
