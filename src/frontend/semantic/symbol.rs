use crate::frontend::syntax::ty::Ty;

pub enum Symbol {
    Function { id: usize, name: String, ty: Ty },
    Variable { offset: usize, name: String, ty: Ty },
    Struct { name: String, ty: Ty },
}

impl Symbol {
    pub fn function(id: usize, name: String, ty: Ty) -> Symbol {
        Symbol::Function { id, name, ty }
    }

    pub fn variable(offset: usize, name: String, ty: Ty) -> Symbol {
        Symbol::Variable { offset, name, ty }
    }
}
