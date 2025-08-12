use crate::frontend::syntax::r#type::Type;

pub enum Declaration {
    Function {
        id: usize,
        name: String,
        type_annotation: Type,
    },
    Variable {
        name: String,
        type_annotation: Type,
    },
}

impl Declaration {
    pub fn function(id: usize, name: String, type_annotation: Type) -> Declaration {
        Declaration::Function {
            id,
            name,
            type_annotation,
        }
    }

    pub fn variable(name: String, type_annotation: Type) -> Declaration {
        Declaration::Variable {
            name,
            type_annotation,
        }
    }
}
