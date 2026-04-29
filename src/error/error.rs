use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};

#[macro_export]
macro_rules! report_error {
    ($span:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        Error::new(Some($span), format!($msg $(, $arg)*))
    };
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        Error::new(None, format!($msg $(, $arg)*))
    };
}

#[derive(Clone)]
pub struct Error {
    pub span: Option<Range<usize>>,
    pub message: String,
}

impl Error {
    pub fn new(span: Option<Range<usize>>, message: String) -> Self {
        Self { span, message }
    }

    pub fn report(&self, source: &str) {
        let file_id = "source";
        let span = self.span.clone().unwrap_or(0..0);

        let mut report =
            Report::build(ReportKind::Error, (file_id, span.clone())).with_message(&self.message);

        if let Some(span) = &self.span {
            report = report.with_label(
                Label::new((file_id, span.clone()))
                    .with_message(&self.message)
                    .with_color(Color::Red),
            );
        }

        report
            .finish()
            .print((file_id, Source::from(source)))
            .unwrap();
    }
}
