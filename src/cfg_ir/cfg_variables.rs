use std::collections::HashMap;

use crate::semantic::hir_id::HirId;

use super::variable::Variable;

pub struct CfgVariables {
    pub variables: HashMap<HirId, Variable>,
    pub next_variable: i16,
}

impl CfgVariables {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next_variable: 0,
        }
    }

    pub fn create_variable(&mut self, id: HirId) -> Variable {
        let variable = Variable(self.next_variable);
        self.next_variable += 1;

        self.variables.insert(id, variable);
        variable
    }

    pub fn get_variable(&self, id: HirId) -> Variable {
        *self
            .variables
            .get(&id)
            .expect("Variable not found for HirId")
    }

    pub fn reset_variables(&mut self) {
        self.next_variable = 0;
    }
}
