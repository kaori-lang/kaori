use crate::semantic::hir_id::HirId;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

use super::operand::Variable;

pub struct CfgConstantPool {
    pub constants: Vec<CfgConstant>,
    pub constants_variable: HashMap<CfgConstant, Variable>,
    pub variable: isize,
}

impl CfgConstantPool {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            constants_variable: HashMap::new(),
            variable: -1,
        }
    }
    pub fn insert_constant(constant: CfgConstant) -> Variable {}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CfgConstant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    FunctionRef(HirId),
}

impl CfgConstant {
    pub fn number(value: f64) -> Self {
        Self::Number(Ordered)
    }
}
