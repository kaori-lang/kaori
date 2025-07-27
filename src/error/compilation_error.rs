use crate::lexer::span::Span;

#[macro_export]
macro_rules! compilation_error {
    ($span:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        CompilationError::new(
            $span,
            format!($msg $(, $arg)*),
        )
    };
}

#[derive(Debug)]
pub struct CompilationError {
    pub span: Span,
    pub message: String,
}

impl CompilationError {
    pub fn new(span: Span, message: String) -> Self {
        Self { span, message }
    }
}
