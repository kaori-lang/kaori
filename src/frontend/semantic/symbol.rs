use super::hir_id::HirId;

#[derive(Clone)]
pub struct Symbol {
    pub id: HirId,
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
    pub fn variable(name: String, offset: usize) -> Symbol {
        Symbol {
            id: HirId::default(),
            name,
            kind: SymbolKind::Variable { offset },
        }
    }

    pub fn function(name: String) -> Symbol {
        Symbol {
            id: HirId::default(),
            name,
            kind: SymbolKind::Function,
        }
    }

    pub fn struct_(name: String) -> Symbol {
        Symbol {
            id: HirId::default(),
            name,
            kind: SymbolKind::Struct,
        }
    }
}
