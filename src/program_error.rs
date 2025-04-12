use crate::{interpreter::runtime_error::RuntimeError, lexer::LexerError, parser::ParserError};

#[derive(Debug)]
pub enum ProgramError {
    LexerError(LexerError),
    ParserError(ParserError),
    RuntimeError(RuntimeError),
}

impl From<LexerError> for ProgramError {
    fn from(error: LexerError) -> Self {
        ProgramError::LexerError(error)
    }
}

impl From<ParserError> for ProgramError {
    fn from(error: ParserError) -> Self {
        ProgramError::ParserError(error)
    }
}

impl From<RuntimeError> for ProgramError {
    fn from(error: RuntimeError) -> Self {
        ProgramError::RuntimeError(error)
    }
}
