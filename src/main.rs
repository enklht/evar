use std::io::stdin;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use logos::Logos;
use seva::{context::Context, eval::eval, lexer::Token, parser::parser};

fn main() {
    let context = Context::parse();

    loop {
        let mut input = String::new();
        if let Ok(_) = stdin().read_line(&mut input) {
            if input == "exit".to_string() {
                break;
            }

            let token_iter = Token::lexer(&input).spanned().map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(()) => (Token::Error, span.into()),
            });

            let token_stream = Stream::from_iter(token_iter).map((0..input.len()).into(), |x| x);

            match parser().parse(token_stream).into_result() {
                Ok(expr) => match eval(expr, &context) {
                    Ok(out) => println!("{}", out),
                    Err(err) => println!("{}", err),
                },
                Err(errs) => {
                    for err in errs {
                        Report::build(ReportKind::Error, ("", err.span().into_range()))
                            .with_label(
                                Label::new(("", err.span().into_range()))
                                    .with_message(err.to_string())
                                    .with_color(Color::Red),
                            )
                            .with_labels(err.contexts().map(|(label, span)| {
                                Label::new(("", span.into_range()))
                                    .with_message(format!(
                                        "while parsing this {}",
                                        label.to_string()
                                    ))
                                    .with_color(Color::Yellow)
                            }))
                            .finish()
                            .eprint(("", Source::from(&input)))
                            .unwrap();
                    }
                }
            }
        }
    }
}
