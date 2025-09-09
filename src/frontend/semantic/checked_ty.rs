use crate::frontend::hir::node_id::NodeId;

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
    Identifier(NodeId),
}

impl CheckedTy {
    pub fn function(parameters: Vec<CheckedTy>, return_ty: CheckedTy) -> CheckedTy {
        CheckedTy::Function {
            parameters,
            return_ty: Box::new(return_ty),
        }
    }
}
