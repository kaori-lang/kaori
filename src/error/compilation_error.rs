use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::compiler::lexer::span::Span;

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

    pub fn report(&self, source: &str) {
        let file_id = "source";

        Report::build(
            ReportKind::Error,
            (file_id, self.span.start..self.span.start + self.span.size),
        )
        .with_message(&self.message)
        .with_label(
            Label::new((file_id, self.span.start..self.span.start + self.span.size))
                .with_message(&self.message)
                .with_color(Color::Red),
        )
        .finish()
        .print((file_id, Source::from(source)))
        .unwrap();
    }
}
