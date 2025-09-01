use crate::frontend::hir::node_id::NodeId;

#[derive(Clone)]
pub enum Symbol {
    Variable {
        id: NodeId,
        name: String,
        offset: usize,
    },
    Function {
        id: NodeId,
        name: String,
    },
    Struct {
        id: NodeId,
        name: String,
    },
}

impl Symbol {
    pub fn variable(id: NodeId, name: String, offset: usize) -> Symbol {
        Symbol::Variable { id, name, offset }
    }

    pub fn function(id: NodeId, name: String) -> Symbol {
        Symbol::Function { id, name }
    }

    pub fn struct_(id: NodeId, name: String) -> Symbol {
        Symbol::Struct { id, name }
    }
}
