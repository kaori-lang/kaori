use super::hir_id::HirId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CheckedTy {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<CheckedTy>,
        return_ty: Box<CheckedTy>,
    },
    Struct {
        fields: Vec<CheckedTy>,
    },
    Identifier(HirId),
}

impl CheckedTy {
    pub fn function(parameters: Vec<CheckedTy>, return_ty: CheckedTy) -> CheckedTy {
        CheckedTy::Function {
            parameters,
            return_ty: Box::new(return_ty),
        }
    }
}
