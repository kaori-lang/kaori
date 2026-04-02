use super::node_id::NodeId;

#[derive(Clone)]
pub struct Symbol {
    pub id: NodeId,
    pub name: String,
    pub kind: SymbolKind,
}

#[derive(Clone)]
pub enum SymbolKind {
    Variable,
    Function,
}

impl Symbol {
    pub fn variable(id: NodeId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Variable,
        }
    }

    pub fn function(id: NodeId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Function,
        }
    }
}
