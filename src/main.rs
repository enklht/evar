use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use logos::Logos;
use seva::{
    args::Args, context::Context, errors::SevaError, eval::eval, lexer::Token, parser::parser,
    readline::SevaEditor,
};

fn main() -> Result<(), SevaError> {
    let args = Args::parse();
    let debug = args.debug;

    let context = Context::new(&args);

    let mut editor = SevaEditor::new(&args);

    loop {
        let input = editor.readline()?;

        if input == *"exit" {
            break;
        };

        let token_iter = Token::lexer(&input).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

        let token_stream =
            Stream::from_iter(token_iter).map((input.len()..input.len()).into(), |x| x);

        match parser().parse(token_stream).into_result() {
            Ok(expr) => {
                if debug {
                    println!("{}", expr)
                };
                match eval(expr, &context) {
                    Ok(out) => println!("{}", out),
                    Err(err) => println!("{}", err),
                }
            }
            Err(errs) => report_error(errs, &input),
        }
    }

    Ok(())
}

fn report_error(errs: Vec<Rich<'_, Token<'_>>>, input: &str) {
    for err in errs {
        Report::build(ReportKind::Error, ("", err.span().into_range()))
            .with_label(
                Label::new(("", err.span().into_range()))
                    .with_message(err.reason().to_string())
                    .with_color(Color::Red),
            )
            .with_labels(err.contexts().map(|(label, span)| {
                Label::new(("", span.into_range()))
                    .with_message(format!("while parsing this {}", label))
                    .with_color(Color::Yellow)
            }))
            .finish()
            .eprint(("", Source::from(input)))
            .expect("failed to report error");
    }
}
