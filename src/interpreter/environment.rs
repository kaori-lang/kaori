use std::collections::HashMap;

use super::data::Data;

#[derive(Debug)]
pub enum EnvironmentError {
    NotFound,
    VariableAlreadyDeclared,
}

pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub symbols: HashMap<String, Data>,
}

impl Environment {
    pub fn new(parent: Box<Environment>) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }

    pub fn get(&self, symbol: &str) -> Result<&Data, EnvironmentError> {
        let Some(s) = self.symbols.get(symbol) else {
            let Some(parent) = &self.parent else {
                return Err(EnvironmentError::NotFound);
            };

            return parent.get(symbol);
        };

        return Ok(s);
    }

    pub fn create(&mut self, symbol: &str, value: Data) -> Result<(), EnvironmentError> {
        if let Some(_) = self.symbols.get(symbol) {
            return Err(EnvironmentError::VariableAlreadyDeclared);
        }

        self.symbols.insert(symbol.to_string(), value);

        return Ok(());
    }
}
