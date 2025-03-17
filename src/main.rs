use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use dialoguer::{BasicHistory, theme::ColorfulTheme};
use logos::Logos;
use seva::{context::Context, eval::eval, lexer::Token, parser::parser};

fn main() {
    let context = Context::parse();

    let mut history = BasicHistory::new().max_entries(100).no_duplicates(true);

    loop {
        if let Ok(input) = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("seva")
            .history_with(&mut history)
            .interact_text()
        {
            if input == "exit" {
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
                        println!("{}", err.to_string())
                    }
                }
            }
        }
    }
}
