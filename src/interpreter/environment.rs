use std::collections::HashMap;

use super::expr::ExprEval;

pub enum Symbol {
    Variable(ExprEval),
}

pub enum EnvironmentError {
    NotFound,
}

pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub symbols: HashMap<String, Symbol>,
}

impl Environment {
    pub fn new(parent: Box<Environment>) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }

    pub fn get(&self, symbol: &str) -> Result<&Symbol, EnvironmentError> {
        let Some(s) = self.symbols.get(symbol) else {
            let Some(parent) = &self.parent else {
                return Err(EnvironmentError::NotFound);
            };

            return parent.get(symbol);
        };

        return Ok(s);
    }

    pub fn create(&mut self, symbol: &str) {
        let a = self.symbols.insert(k, v);
    }
}
