#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResolvedTy {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<ResolvedTy>,
        return_ty: Box<ResolvedTy>,
    },
    Struct {
        fields: Vec<ResolvedTy>,
    },
}

impl ResolvedTy {
    pub fn function(parameters: Vec<ResolvedTy>, return_ty: ResolvedTy) -> ResolvedTy {
        Ty::Function {
            parameters,
            return_ty: Box::new(return_ty),
        }
    }
}
