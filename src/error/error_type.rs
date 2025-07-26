use core::fmt;

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError(String),
    TypeError,
    NotFound,
}

pub enum Kaori {}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::SyntaxError(message) => write!(f, "SyntaxError: {}", message),
            ErrorType::TypeError => write!(f, "TypeError"),
            ErrorType::NotFound => write!(f, "NotFound"),
        }
    }
}
