#[derive(Debug)]
pub enum RuntimeError {
    InvalidEvaluation,
    NotFound,
    VariableAlreadyDeclared,
}
