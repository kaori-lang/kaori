#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum TypeDef {
    Boolean,
    String,
    Number,
    #[default]
    Void,
    Function {
        parameters: Vec<TypeDef>,
        return_ty: Box<TypeDef>,
    },
    Struct {
        fields: Vec<TypeDef>,
    },
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
