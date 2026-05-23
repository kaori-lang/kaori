use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::syntax::token::Span;

#[macro_export]
macro_rules! report_error {
    ($span:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        Error::new(Some($span), format!($msg $(, $arg)*))
    };
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        Error::new(None, format!($msg $(, $arg)*))
    };
}

#[derive(Clone, Debug)]
pub struct Error {
    pub span: Option<Span>,
    pub message: String,
}

impl Error {
    pub fn new(span: Option<Span>, message: String) -> Self {
        Self { span, message }
    }

    pub fn report(&self, source: &str) {
        let file_id = "source";
        let span: Range<usize> = self.span.unwrap_or_default().into();

        let report = Report::build(ReportKind::Error, (file_id, span.clone())).with_label(
            Label::new((file_id, span))
                .with_message(&self.message)
                .with_color(Color::Red),
        );

        report
            .finish()
            .print((file_id, Source::from(source)))
            .unwrap();
    }
}
