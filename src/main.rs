use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use colored::Colorize;
use logos::Logos;
use rustyline::{
    Completer, Config, Editor, Helper, Highlighter, Hinter, Validator,
    completion::FilenameCompleter, highlight::MatchingBracketHighlighter, hint::HistoryHinter,
    validate::MatchingBracketValidator,
};
use seva::{context::Context, eval::eval, lexer::Token, parser::parser};

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct RustyLineHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
    prompt: String,
}

fn main() {
    let context = Context::parse();

    let editor_config = Config::builder().auto_add_history(true).build();

    let helper = RustyLineHelper {
        completer: FilenameCompleter::new(),
        validator: MatchingBracketValidator::new(),
        hinter: HistoryHinter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        prompt: "".into(),
    };

    let mut editor = Editor::with_config(editor_config).unwrap();
    editor.set_helper(Some(helper));

    loop {
        let prompt = format!("{}", "> ".green());
        editor.helper_mut().expect("No helper").prompt = prompt.clone();
        let input = editor.readline(&prompt);

        match input {
            Ok(input) => {
                if input == *"exit" {
                    break;
                };

                let token_iter = Token::lexer(&input).spanned().map(|(tok, span)| match tok {
                    Ok(tok) => (tok, span.into()),
                    Err(()) => (Token::Error, span.into()),
                });

                let token_stream =
                    Stream::from_iter(token_iter).map((0..input.len()).into(), |x| x);

                match parser().parse(token_stream).into_result() {
                    Ok(expr) => match eval(expr, &context) {
                        Ok(out) => println!("{}", out),
                        Err(err) => println!("{}", err),
                    },
                    Err(errs) => {
                        report_error(errs, &input);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        };
    }
    editor.append_history("history.txt").unwrap();
}

fn report_error(errs: Vec<Rich<'_, Token<'_>>>, input: &str) {
    for err in errs {
        Report::build(ReportKind::Error, ("", err.span().into_range()))
            .with_label(
                Label::new(("", err.span().into_range()))
                    .with_message(err.to_string())
                    .with_color(Color::Red),
            )
            .with_labels(err.contexts().map(|(label, span)| {
                Label::new(("", span.into_range()))
                    .with_message(format!("while parsing this {}", label))
                    .with_color(Color::Yellow)
            }))
            .finish()
            .eprint(("", Source::from(input)))
            .unwrap();
    }
}
