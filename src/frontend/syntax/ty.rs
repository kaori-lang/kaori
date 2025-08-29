#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<Ty>,
        return_ty: Box<Ty>,
    },
    Struct {
        fields: Vec<Ty>,
    },
    Custom {
        name: String,
    },
}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_ty: Ty) -> Ty {
        Ty::Function {
            parameters,
            return_ty: Box::new(return_ty),
        }
    }
}
