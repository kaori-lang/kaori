use std::fmt::{self, Display, Formatter};

use ordered_float::OrderedFloat;

use super::function::FunctionId;

#[derive(Default)]
pub struct ConstantPool {
    pub constants: Vec<Constant>,
}

impl ConstantPool {
    fn push_constant(&mut self, constant: Constant) -> ConstantIndex {
        let constant_index = self
            .constants
            .iter()
            .position(|constant_| constant == *constant_);

        if let Some(index) = constant_index {
            ConstantIndex(index)
        } else {
            let index = self.constants.len();

            self.constants.push(constant);

            ConstantIndex(index)
        }
    }

    pub fn push_function(&mut self, value: FunctionId) -> ConstantIndex {
        self.push_constant(Constant::Function(value))
    }

    pub fn push_string(&mut self, value: String) -> ConstantIndex {
        self.push_constant(Constant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> ConstantIndex {
        self.push_constant(Constant::Number(OrderedFloat(value)))
    }

    pub fn push_boolean(&mut self, value: bool) -> ConstantIndex {
        self.push_constant(Constant::Boolean(value))
    }

    pub fn push_nil(&mut self) -> ConstantIndex {
        self.push_constant(Constant::Nil)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    Function(FunctionId),
    Nil,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstantIndex(pub usize);

impl Display for ConstantIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "k{}", self.0)
    }
}

impl ConstantIndex {
    #[inline(always)]
    pub fn to_u8(self) -> u8 {
        self.0 as u8
    }
}
