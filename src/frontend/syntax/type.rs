#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Boolean,
    String,
    Number,
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl Type {
    pub fn function(parameters: Vec<Type>, return_type: Box<Type>) -> Type {
        Type::Function {
            parameters,
            return_type,
        }
    }
}
