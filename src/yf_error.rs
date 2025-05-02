use core::fmt;

#[derive(Debug)]
pub struct YFError {
    pub error_type: ErrorType,
    pub line: u32,
}

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    TypeError,
    NotFound,
}

impl fmt::Display for YFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let YFError { error_type, line } = self;

        let error = match error_type {
            ErrorType::SyntaxError => "SyntaxError",
            ErrorType::TypeError => "TypeError",
            ErrorType::NotFound => "NotFound",
        };

        write!(f, "> {}\n> Line: {}", error, line)
    }
}
