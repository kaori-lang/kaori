#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeDef {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<TypeDef>,
        return_ty: Box<TypeDef>,
    },
    Struct {
        fields: Vec<TypeDef>,
    },
}

impl Default for TypeDef {
    fn default() -> Self {
        TypeDef::Void
    }
}

impl TypeDef {
    pub fn function(parameters: Vec<TypeDef>, return_ty: TypeDef) -> TypeDef {
        TypeDef::Function {
            parameters,
            return_ty: Box::new(return_ty),
        }
    }

    pub fn struct_(fields: Vec<TypeDef>) -> TypeDef {
        TypeDef::Struct { fields }
    }
}
