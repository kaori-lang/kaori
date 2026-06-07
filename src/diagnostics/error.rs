use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::{compiler::INTERNER, syntax::token::Span, util::string_interner::Symbol};

#[derive(Clone, Debug)]
pub struct Error(Box<InnerError>);

#[derive(Clone, Debug)]
pub struct InnerError {
    pub span: Span,
    pub message: String,
    pub file: Symbol,
}

impl Error {
    pub fn new(span: Span, file: Symbol, message: String) -> Self {
        Self(Box::new(InnerError {
            span,
            file,
            message,
        }))
    }

    pub fn report(&self) {
        let file = INTERNER.lock().unwrap().resolve(self.0.file).to_string();
        let source = std::fs::read_to_string(&file).unwrap_or_default();

        let span: Range<usize> = self.0.span.into();

        let report = Report::build(ReportKind::Error, (file.as_str(), span.clone()))
            .with_message(&self.0.message)
            .with_label(
                Label::new((file.as_str(), span))
                    .with_message(&self.0.message)
                    .with_color(Color::BrightRed),
            );

        report
            .finish()
            .print((file.as_str(), Source::from(&source)))
            .unwrap();
    }
}
