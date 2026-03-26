use ordered_float::OrderedFloat;

use super::{function::FunctionId, operand::Operand};

#[derive(Default)]
pub struct ConstantPool {
    pub constants: Vec<Constant>,
}

impl ConstantPool {
    fn push_constant(&mut self, constant: Constant) -> Operand {
        let constant_index = self
            .constants
            .iter()
            .position(|constant_| constant == *constant_);

        if let Some(index) = constant_index {
            Operand::Constant(index)
        } else {
            let index = self.constants.len();

            self.constants.push(constant);

            Operand::Constant(index)
        }
    }

    pub fn push_function(&mut self, value: FunctionId) -> Operand {
        self.push_constant(Constant::Function(value))
    }

    pub fn push_string(&mut self, value: String) -> Operand {
        self.push_constant(Constant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> Operand {
        self.push_constant(Constant::Number(OrderedFloat(value)))
    }

    pub fn push_boolean(&mut self, value: bool) -> Operand {
        self.push_constant(Constant::Boolean(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    Function(FunctionId),
}
