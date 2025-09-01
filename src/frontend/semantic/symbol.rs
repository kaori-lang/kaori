use crate::frontend::hir::node_id::NodeId;

pub enum Symbol {
    Global { id: NodeId, name: String },
    Local { offset: usize, name: String },
}

impl Symbol {
    pub fn global(id: NodeId, name: String) -> Symbol {
        Symbol::Global { id, name }
    }

    pub fn local(offset: usize, name: String) -> Symbol {
        Symbol::Local { offset, name }
    }
}
