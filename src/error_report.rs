use crate::lexer::Token;
use chumsky::prelude::*;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        Config, emit,
        termcolor::{ColorChoice, StandardStream},
    },
};

pub struct ErrorReporter {
    writer: StandardStream,
    config: Config,
}

impl ErrorReporter {
    pub fn new(no_color: bool) -> Self {
        let writer = StandardStream::stderr(if no_color {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        });
        ErrorReporter {
            writer: writer,
            config: codespan_reporting::term::Config::default(),
        }
    }

    pub fn report_error(&mut self, errs: Vec<Rich<'_, Token<'_>>>, input: &str) {
        let file = SimpleFile::new("<repl>", input);

        for err in errs {
            let mut labels = vec![
                Label::primary((), err.span().into_range()).with_message(err.reason().to_string()),
            ];

            labels.extend(err.contexts().map(|(label, span)| {
                Label::secondary((), span.into_range())
                    .with_message(&format!("while parsing this {}", label))
            }));

            let diagnostic = Diagnostic::error()
                .with_message(err.reason().to_string())
                .with_labels(labels);

            emit(&mut self.writer.lock(), &self.config, &file, &diagnostic)
                .expect("failed writing diagnostics");
        }
    }
}
