use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::{compiler::INTERNER, syntax::token::Span, util::string_interner::Symbol};

#[macro_export]
macro_rules! report_error {
    ($span:expr, $path:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        Err($crate::diagnostics::error::Error::new(
            $span,
            Some($path),
            format!($msg $(, $arg)*)
        ))
    };
    ($msg:literal $(, $arg:expr)* $(,)?) => {
        Err($crate::diagnostics::error::Error::new(
            $crate::syntax::token::Span::default(),
            None,
            format!($msg $(, $arg)*)
        ))
    };
}

#[derive(Clone, Debug)]
pub struct Error(Box<InnerError>);

#[derive(Clone, Debug)]
pub struct InnerError {
    pub span: Span,
    pub message: String,
    pub path: Option<Symbol>,
}

impl Error {
    pub fn new(span: Span, path: Option<Symbol>, message: String) -> Self {
        Self(Box::new(InnerError {
            span,
            message,
            path,
        }))
    }

    pub fn report(&self) {
        let (file_name, source) = match self.0.path {
            Some(path) => {
                let file_name = INTERNER.lock().unwrap().resolve(path).to_string();
                let source = std::fs::read_to_string(&file_name).unwrap_or_default();
                (file_name, source)
            }
            None => ("unknown".to_string(), String::new()),
        };

        let span: Range<usize> = self.0.span.into();

        let report = Report::build(ReportKind::Error, (file_name.as_str(), span.clone()))
            .with_message(&self.0.message)
            .with_label(
                Label::new((file_name.as_str(), span))
                    .with_message(&self.0.message)
                    .with_color(Color::BrightRed),
            );

        report
            .finish()
            .print((file_name.as_str(), Source::from(&source)))
            .unwrap();
    }
}
