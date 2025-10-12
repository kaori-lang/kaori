use ordered_float::OrderedFloat;
use std::collections::HashMap;

use super::variable::Variable;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

pub struct CfgConstants {
    pub number_constants: Vec<f64>,
    pub number_variable: HashMap<OrderedFloat<f64>, Variable>,

    pub string_constants: Vec<String>,
    pub string_variable: HashMap<String, Variable>,

    pub boolean_constants: Vec<bool>,
    pub boolean_variable: HashMap<bool, Variable>,

    pub function_constants: Vec<BlockId>,
    pub function_variable: HashMap<BlockId, Variable>,

    pub variable: isize,
}

impl Default for CfgConstants {
    fn default() -> Self {
        Self {
            number_constants: Vec::new(),
            number_variable: HashMap::new(),
            string_constants: Vec::new(),
            string_variable: HashMap::new(),
            boolean_constants: Vec::new(),
            boolean_variable: HashMap::new(),
            function_constants: Vec::new(),
            function_variable: HashMap::new(),
            variable: 0,
        }
    }
}

impl CfgConstants {
    #[inline]
    fn next_variable<T>(constants: &Vec<T>) -> i16 {
        -((constants.len() + 1) as i16)
    }

    pub fn push_number(&mut self, value: f64) -> Variable {
        let number = OrderedFloat(value);

        match self.number_variable.get(&number) {
            Some(variable) => *variable,
            None => {
                let variable = Variable::Number(Self::next_variable(&self.number_constants));
                self.number_constants.push(*number);
                self.number_variable.insert(number, variable);
                variable
            }
        }
    }

    pub fn push_string(&mut self, value: String) -> Variable {
        if let Some(variable) = self.string_variable.get(&value) {
            *variable
        } else {
            let variable = Variable::String(Self::next_variable(&self.string_constants));
            self.string_constants.push(value.clone());
            self.string_variable.insert(value, variable);
            variable
        }
    }

    pub fn push_boolean(&mut self, value: bool) -> Variable {
        if let Some(variable) = self.boolean_variable.get(&value) {
            *variable
        } else {
            let variable = Variable::Boolean(Self::next_variable(&self.boolean_constants));
            self.boolean_constants.push(value);
            self.boolean_variable.insert(value, variable);
            variable
        }
    }

    pub fn push_function_ref(&mut self, func: BlockId) -> Variable {
        if let Some(variable) = self.function_variable.get(&func) {
            *variable
        } else {
            let variable = Variable::Function(Self::next_variable(&self.function_constants));
            self.function_constants.push(func);
            self.function_variable.insert(func, variable);
            variable
        }
    }
}
