use super::hir_id::HirId;

#[derive(Clone)]
pub struct Symbol {
    pub id: HirId,
    pub name: String,
    pub kind: SymbolKind,
}

#[derive(Clone)]
pub enum SymbolKind {
    Variable,
    Function,
    Struct,
}

impl Symbol {
    pub fn variable(id: HirId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Variable,
        }
    }

    pub fn function(id: HirId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Function,
        }
    }

    pub fn struct_(id: HirId, name: String) -> Symbol {
        Symbol {
            id,
            name,
            kind: SymbolKind::Struct,
        }
    }
}
