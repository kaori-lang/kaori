use ordered_float::OrderedFloat;
use std::collections::HashMap;

use super::operand::Operand;

#[derive(Default)]
pub struct CfgConstants {
    pub constants: Vec<CfgConstant>,
    pub constants_index: HashMap<CfgConstant, usize>,
}

impl CfgConstants {
    fn push_constant(&mut self, constant: CfgConstant) -> Operand {
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
        self.push_constant(CfgConstant::Function(value))
    }

    pub fn push_string(&mut self, value: String) -> Operand {
        self.push_constant(CfgConstant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> Operand {
        self.push_constant(CfgConstant::Number(OrderedFloat(value)))
    }

    pub fn push_boolean(&mut self, value: bool) -> Operand {
        self.push_constant(CfgConstant::Boolean(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CfgConstant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    Function(usize),
}
