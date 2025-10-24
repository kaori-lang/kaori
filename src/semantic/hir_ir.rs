use super::{hir_decl::HirDecl, r#type::Types};

pub struct HirIr {
    pub declarations: Vec<HirDecl>,
    pub types: Types,
}

impl HirIr {
    pub fn new(declarations: Vec<HirDecl>, types: Types) -> Self {
        Self {
            declarations,
            types,
        }
    }
}
