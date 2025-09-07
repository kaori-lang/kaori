use super::resolution_table::Resolution;
use crate::frontend::hir::node_id::NodeId;

#[derive(Clone)]
pub struct Symbol {
    pub id: NodeId,
    pub name: String,
    pub kind: SymbolKind,
}

#[derive(Clone)]
pub enum SymbolKind {
    Variable { offset: usize },
    Function,
    Struct,
}

impl Symbol {
    pub fn variable(id: NodeId, name: String, offset: usize) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Variable { offset },
        }
    }

    pub fn function(id: NodeId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Function,
        }
    }

    pub fn struct_(id: NodeId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Struct,
        }
    }

    pub fn as_resolution(&self) -> Resolution {
        match &self.kind {
            SymbolKind::Struct => Resolution::struct_(self.id),
            SymbolKind::Function => Resolution::function(self.id),
            SymbolKind::Variable { .. } => Resolution::variable(self.id),
        }
    }
}
