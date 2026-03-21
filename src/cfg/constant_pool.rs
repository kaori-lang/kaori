use ordered_float::OrderedFloat;
use std::collections::HashMap;

use super::operand::Operand;

#[derive(Default)]
pub struct ConstantPool {
    pub constants: Vec<Constant>,
    pub constants_index: HashMap<Constant, usize>,
}

impl ConstantPool {
    fn push_constant(&mut self, constant: Constant) -> Operand {
        if let Some(index) = self.constants_index.get(&constant) {
            Operand::Constant(*index)
        } else {
            let index = self.constants_index.len();

            self.constants_index.insert(constant.to_owned(), index);

            self.constants.push(constant);

            Operand::Constant(index)
        }
    }

    pub fn push_function(&mut self, value: usize) -> Operand {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    Function(usize),
}
