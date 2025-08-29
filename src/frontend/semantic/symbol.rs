use super::resolved_ty::ResolvedTy;

pub enum Symbol {
    Function {
        id: usize,
        name: String,
        ty: ResolvedTy,
    },
    Variable {
        offset: usize,
        name: String,
        ty: ResolvedTy,
    },
    Struct {
        id: usize,
        name: String,
        ty: ResolvedTy,
    },
}

impl Symbol {
    pub fn function(id: usize, name: String, ty: ResolvedTy) -> Symbol {
        Symbol::Function { id, name, ty }
    }

    pub fn variable(offset: usize, name: String, ty: ResolvedTy) -> Symbol {
        Symbol::Variable { offset, name, ty }
    }

    pub fn struct_(id: usize, name: String, ty: ResolvedTy) -> Symbol {
        Symbol::Struct { id, name, ty }
    }
}
