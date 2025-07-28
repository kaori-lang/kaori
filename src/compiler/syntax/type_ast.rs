#[derive(Debug, PartialEq, Eq)]
pub enum TypeAST {
    Boolean,
    String,
    Number,
    Function {
        parameters: Vec<TypeAST>,
        return_type: Box<TypeAST>,
    },
}
