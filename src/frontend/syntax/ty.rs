#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<Ty>,
        return_type: Box<Ty>,
    },
}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_type: Ty) -> Ty {
        Ty::Function {
            parameters,
            return_type: Box::new(return_type),
        }
    }
}
