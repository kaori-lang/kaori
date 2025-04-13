use std::collections::HashMap;

use crate::yf_error::{RuntimeError, YFError};

use super::data::Data;

pub struct Environment {
    pub stack: Vec<HashMap<String, Data>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn get_symbol(&self, symbol: &str) -> Result<Data, YFError> {
        let mut ptr = self.stack.len();

        while ptr > 0 {
            ptr -= 1;

            if let Some(data) = self.stack[ptr].get(symbol) {
                return Ok(data.clone());
            }
        }

        return Err(YFError::RuntimeError(RuntimeError::NotFound));
    }

    pub fn create_symbol(&mut self, symbol: String, data: Data) -> Result<(), YFError> {
        self.stack.last_mut().unwrap().insert(symbol, data);

        return Ok(());
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }
}
