#[derive(Debug)]
pub struct YFError {
    pub error_type: ErrorType,
    pub line: u32,
}

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    EndOfFile,
    TypeError,
    NotFound,
}
