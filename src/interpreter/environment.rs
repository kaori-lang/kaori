use std::collections::HashMap;

use crate::{lexer::data::Data, yf_error::ErrorType};

pub struct Environment {
    pub stack: Vec<HashMap<String, Data>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn get_symbol(&self, symbol: &str) -> Result<Data, ErrorType> {
        let mut ptr = self.stack.len();

        while ptr > 0 {
            ptr -= 1;

            if let Some(data) = self.stack[ptr].get(symbol) {
                return Ok(data.clone());
            }
        }

        return Err(ErrorType::NotFound);
    }

    pub fn create_symbol(&mut self, symbol: String, data: Data) -> Result<(), ErrorType> {
        let Some(current_scope) = self.stack.last_mut() else {
            return Err(ErrorType::NotFound);
        };

        current_scope.insert(symbol, data);

        return Ok(());
    }

    pub fn update_symbol(&mut self, symbol: &str, data: &Data) -> Result<(), ErrorType> {
        let Some(current_scope) = self.stack.last_mut() else {
            return Err(ErrorType::NotFound);
        };

        let Some(found_data) = current_scope.get(symbol) else {
            return Err(ErrorType::NotFound);
        };

        match (&data, found_data) {
            (Data::Number(_), Data::Number(_)) => {
                current_scope.insert(symbol.to_string(), data.clone())
            }
            (Data::Boolean(_), Data::Boolean(_)) => {
                current_scope.insert(symbol.to_string(), data.clone())
            }
            (Data::String(_), Data::String(_)) => {
                current_scope.insert(symbol.to_string(), data.clone())
            }
            _ => return Err(ErrorType::TypeError),
        };

        return Ok(());
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }
}
