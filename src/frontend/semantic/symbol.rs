use super::resolved_ty::ResolvedTy;

pub struct Symbol {
    pub offset: usize,
    pub name: String,
    pub ty: ResolvedTy,
}

impl Symbol {
    pub fn new(offset: usize, name: String, ty: ResolvedTy) -> Symbol {
        Symbol { offset, name, ty }
    }
}
