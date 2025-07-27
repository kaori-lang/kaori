use crate::lexer::span::Span;

#[macro_export]
macro_rules! compiler_error {
    ($span:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        CompilerError::new(
            $span,
            format!($msg $(, $arg)*),
        )
    };
}

pub struct CompilerError {
    pub span: Span,
    pub message: String,
}

impl CompilerError {
    pub fn new(span: Span, message: String) -> Self {
        Self { span, message }
    }
}
