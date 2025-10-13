use std::collections::HashMap;

use super::{hir_decl::HirDecl, hir_id::HirId, type_def::TypeDef};

pub struct HirIr {
    pub declarations: Vec<HirDecl>,
    pub types_table: HashMap<HirId, TypeDef>,
}

impl HirIr {
    pub fn new(declarations: Vec<HirDecl>, types_table: HashMap<HirId, TypeDef>) -> Self {
        Self {
            declarations,
            types_table,
        }
    }

    pub fn get_type(&self, id: &HirId) -> &TypeDef {
        self.types_table.get(id).unwrap()
    }
}
