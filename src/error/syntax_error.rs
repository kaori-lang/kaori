use crate::lexer::token_type::TokenType;
use core::fmt;

pub struct SyntaxError {
    pub error_type: Syntax,
    pub line: u32,
}

#[derive(Debug)]
pub enum Syntax {
    InvalidToken(char),
    UnexpectedToken(TokenType, TokenType),
    UnexpectedEof,
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Syntax::UnexpectedToken(expected, found) => {
                write!(f, "expected {:?} token, but found {:?}", expected, found)
            }
            Syntax::InvalidToken(c) => {
                write!(f, "invalid token: {:?}", c)
            }
            Syntax::UnexpectedEof => {
                write!(f, "unexpected end of file")
            }
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {} at {}", self.error_type, self.line)
    }
}
