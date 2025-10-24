use std::collections::HashMap;

use super::{hir_decl::HirDecl, hir_id::HirId, type_def::Type};

pub struct HirIr {
    pub declarations: Vec<HirDecl>,
    pub types_table: HashMap<HirId, Type>,
}

impl HirIr {
    pub fn new(declarations: Vec<HirDecl>, types_table: HashMap<HirId, Type>) -> Self {
        Self {
            declarations,
            types_table,
        }
    }
}
