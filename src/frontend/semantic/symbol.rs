use super::resolved_ty::ResolvedTy;

pub enum Symbol {
    Global {
        id: usize,
        name: String,
        ty: ResolvedTy,
    },
    Local {
        offset: usize,
        name: String,
        ty: ResolvedTy,
    },
}

impl Symbol {
    pub fn global(id: usize, name: String, ty: ResolvedTy) -> Symbol {
        Symbol::Global { id, name, ty }
    }

    pub fn local(offset: usize, name: String, ty: ResolvedTy) -> Symbol {
        Symbol::Local { offset, name, ty }
    }
}
