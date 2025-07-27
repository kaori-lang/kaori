use crate::lexer::token::Token;

#[macro_export]
macro_rules! compiler_error {
    ($token:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        $crate::CompilerError::new(
            $token,
            format!($msg $(, $arg)*),
        )
    };
}

pub struct CompilerError {
    pub token: Token,
    pub message: String,
}

impl CompilerError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}
