use crate::lexer::Token;
use chumsky::prelude::*;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

pub fn create_writer(no_color: bool) -> StandardStream {
    StandardStream::stderr(if no_color {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    })
}

pub fn report_error(
    errs: Vec<Rich<'_, Token<'_>>>,
    input: &str,
    writer: &StandardStream,
    config: &term::Config,
) {
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

        term::emit(&mut writer.lock(), config, &file, &diagnostic)
            .expect("failed writing diagnostics");
    }
}
