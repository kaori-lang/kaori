use std::collections::HashMap;

use super::{hir_decl::HirDecl, hir_id::HirId, r#type::Type};

pub struct HirIr {
    pub declarations: Vec<HirDecl>,
    pub types: HashMap<HirId, Type>,
}

impl HirIr {
    pub fn new(declarations: Vec<HirDecl>, types: HashMap<HirId, Type>) -> Self {
        Self {
            declarations,
            types,
        }
    }
}
