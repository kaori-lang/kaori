use ariadne::{Color, Label, Report, ReportKind, Source};
use logos::Span;

#[macro_export]
macro_rules! kaori_error {
    ($span:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        KaoriError::new(Some($span), format!($msg $(, $arg)*))
    };
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        KaoriError::new(None, format!($msg $(, $arg)*))
    };
}

#[derive(Clone)]
pub struct KaoriError {
    pub span: Option<Span>,
    pub message: String,
}

impl KaoriError {
    pub fn new(span: Option<Span>, message: String) -> Self {
        Self { span, message }
    }

    pub fn report(&self, source: &str) {
        let file_id = "source";
        let span = self.span.clone().unwrap_or(Span { start: 0, end: 0 });

        let mut report = Report::build(ReportKind::Error, (file_id, span.start..span.end))
            .with_message(&self.message);

        if let Some(span) = &self.span {
            report = report.with_label(
                Label::new((file_id, span.start..span.end))
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
