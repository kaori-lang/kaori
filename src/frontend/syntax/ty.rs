#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Boolean,
    String,
    Number,
    Function {
        parameters: Vec<Ty>,
        return_type: Box<Ty>,
    },
}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_type: Box<Ty>) -> Ty {
        Ty::Function {
            parameters,
            return_type,
        }
    }

    pub fn number() -> Ty {
        Ty::Number
    }

    pub fn string() -> Ty {
        Ty::String
    }

    pub fn boolean() -> Ty {
        Ty::Boolean
    }
}
