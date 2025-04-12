use std::collections::HashMap;

use super::{data::Data, runtime_error::RuntimeError};

pub struct Environment {
    pub stack: Vec<HashMap<String, Data>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub fn get_symbol(&self, symbol: &str) -> Result<Data, RuntimeError> {
        let mut ptr = self.stack.len();

        while ptr > 0 {
            ptr -= 1;

            if let Some(data) = self.stack[ptr].get(symbol) {
                return Ok(data.clone());
            }
        }

        return Err(RuntimeError::NotFound);
    }

    pub fn create_symbol(&mut self, symbol: String, data: Data) -> Result<(), RuntimeError> {
        self.stack.last_mut().unwrap().insert(symbol, data);

        return Ok(());
    }

    fn stack_pop(&mut self) {
        self.stack.pop();
    }

    fn stack_push(&mut self) {
        self.stack.push(HashMap::new());
    }
}
