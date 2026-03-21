use super::{decl::Decl, r#type::Types};

pub struct Hir {
    pub declarations: Vec<Decl>,
    pub types: Types,
}

impl Hir {
    pub fn new(declarations: Vec<Decl>, types: Types) -> Self {
        Self {
            declarations,
            types,
        }
    }
}
