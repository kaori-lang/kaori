use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::compiler::scanner::span::Span;

#[macro_export]
macro_rules! kaori_error {
    ($span:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        KaoriError::new(
            $span,
            format!($msg $(, $arg)*),
        )
    };
}

pub struct KaoriError {
    pub span: Span,
    pub message: String,
}

impl KaoriError {
    pub fn new(span: Span, message: String) -> Self {
        Self { span, message }
    }

    pub fn report(&self, source: &str) {
        let file_id = "source";

        Report::build(ReportKind::Error, (file_id, self.span.start..self.span.end))
            .with_message(&self.message)
            .with_label(
                Label::new((file_id, self.span.start..self.span.end))
                    .with_message(&self.message)
                    .with_color(Color::Red),
            )
            .finish()
            .print((file_id, Source::from(source)))
            .unwrap();
    }
}
