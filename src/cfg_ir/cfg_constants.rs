use ordered_float::OrderedFloat;
use std::collections::HashMap;

use super::{basic_block::BlockId, variable::Variable};

pub struct CfgConstants {
    pub constants: Vec<CfgConstant>,
    pub constants_variable: HashMap<CfgConstant, Variable>,
    pub variable: i16,
}

impl Default for CfgConstants {
    fn default() -> Self {
        Self {
            constants: Vec::new(),
            constants_variable: HashMap::new(),
            variable: -1,
        }
    }
}

impl CfgConstants {
    fn push_constant(&mut self, constant: CfgConstant) -> Variable {
        if let Some(variable) = self.constants_variable.get(&constant) {
            *variable
        } else {
            let variable = Variable(self.variable);

            self.constants_variable
                .insert(constant.to_owned(), variable);
            self.constants.push(constant);

            self.variable -= 1;

            variable
        }
    }

    pub fn push_function(&mut self, value: BlockId) -> Variable {
        self.push_constant(CfgConstant::Function(value))
    }

    pub fn push_string(&mut self, value: String) -> Variable {
        self.push_constant(CfgConstant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> Variable {
        self.push_constant(CfgConstant::Number(OrderedFloat(value)))
    }

    pub fn push_boolean(&mut self, value: bool) -> Variable {
        self.push_constant(CfgConstant::Boolean(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CfgConstant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    Function(BlockId),
}
