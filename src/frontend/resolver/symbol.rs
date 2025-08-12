use crate::frontend::syntax::r#type::Type;

pub enum Symbol {
    Function {
        id: usize,
        name: String,
        type_annotation: Type,
    },
    Variable {
        offset: usize,
        name: String,
        type_annotation: Type,
    },
}

impl Symbol {
    pub fn function(id: usize, name: String, type_annotation: Type) -> Symbol {
        Symbol::Function {
            id,
            name,
            type_annotation,
        }
    }

    pub fn variable(offset: usize, name: String, type_annotation: Type) -> Symbol {
        Symbol::Variable {
            offset,
            name,
            type_annotation,
        }
    }
}
