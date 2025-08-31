use crate::frontend::{hir::node_id::NodeId, syntax::ty::Ty};

pub enum Symbol {
    Global { id: NodeId, name: String, ty: Ty },
    Local { offset: usize, name: String, ty: Ty },
}

impl Symbol {
    pub fn global(id: NodeId, name: String, ty: Ty) -> Symbol {
        Symbol::Global { id, name, ty }
    }

    pub fn local(offset: usize, name: String, ty: Ty) -> Symbol {
        Symbol::Local { offset, name, ty }
    }
}
