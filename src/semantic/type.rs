use super::hir_id::HirId;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Type {
    Boolean,
    String,
    Number,
    #[default]
    Void,
    Function {
        parameters: Vec<HirId>,
        return_ty: HirId,
    },
    Struct {
        fields: Vec<HirId>,
    },
    TypeRef(HirId),
}

impl Type {
    pub fn function(parameters: Vec<HirId>, return_ty: HirId) -> Type {
        Type::Function {
            parameters,
            return_ty,
        }
    }

    pub fn struct_(fields: Vec<HirId>) -> Type {
        Type::Struct { fields }
    }

    pub fn type_ref(id: HirId) -> Type {
        Type::TypeRef(id)
    }
}
