use std::collections::HashMap;

use crate::semantic::{hir_id::HirId, type_def::TypeDef};

use super::variable::Variable;

pub struct CfgVariables {
    pub types_table: HashMap<HirId, TypeDef>,
    pub number_variable: i16,

    pub string_variable: i16,

    pub boolean_variable: i16,

    pub function_variable: i16,

    pub variables: HashMap<HirId, Variable>,
}

impl CfgVariables {
    pub fn new(types_table: HashMap<HirId, TypeDef>) -> Self {
        Self {
            types_table,
            number_variable: 0,
            string_variable: 0,
            boolean_variable: 0,
            function_variable: 0,
        }
    }
    pub fn create_variable(&mut self, id: HirId) {
        let ty = self.types_table.get(id).unwrap();
    }

    pub fn get_variable(&self, id: HirId) -> Variable {
        self.variables.get(id)
    }
}
