use std::collections::HashMap;

use super::{data::Data, error::RuntimeError};

pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub symbols: HashMap<String, Data>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
        }
    }

    pub fn get(&self, symbol: &str) -> Result<Data, RuntimeError> {
        let Some(s) = self.symbols.get(symbol) else {
            let Some(parent) = &self.parent else {
                return Err(RuntimeError::NotFound);
            };

            return parent.get(symbol);
        };

        return Ok(s.clone());
    }

    pub fn create(&mut self, symbol: &str, value: Data) -> Result<(), RuntimeError> {
        if let Some(_) = self.symbols.get(symbol) {
            return Err(RuntimeError::NotFound);
        }

        self.symbols.insert(symbol.to_string(), value);

        return Ok(());
    }
}
