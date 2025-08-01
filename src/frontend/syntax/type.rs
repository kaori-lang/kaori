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
