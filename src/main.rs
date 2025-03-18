use std::borrow::Cow;

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
    completion::FilenameCompleter, highlight::Highlighter, hint::HistoryHinter,
    validate::MatchingBracketValidator,
};
use seva::{args::Args, context::Context, eval::eval, lexer::Token, parser::parser};

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct RustyLineHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Highlighter)]
    highlighter: SevaHighlighter,
    prompt: String,
}

struct SevaHighlighter;

impl Highlighter for SevaHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let tokens = Token::lexer(line).spanned();

        let highlighted_line = tokens.fold(String::new(), |acc, (lex_result, span)| {
            acc + &{
                match lex_result {
                    Err(_) => format!("{}", line[span].truecolor(237, 135, 150)),
                    Ok(token) => match token {
                        Token::Number(_) => format!("{}", line[span].truecolor(245, 169, 127)),
                        Token::Ident(_) => format!("{}", line[span].truecolor(138, 173, 244)),
                        Token::Plus
                        | Token::Minus
                        | Token::Asterisk
                        | Token::Slash
                        | Token::Percent
                        | Token::Caret
                        | Token::Exclamation => format!("{}", line[span].truecolor(125, 196, 228)),
                        Token::LParen | Token::RParen => {
                            format!("{}", line[span].truecolor(238, 212, 159))
                        }
                        _ => line[span].to_string(),
                    },
                }
            }
        });

        highlighted_line.into()
    }
    fn highlight_char(
        &self,
        _line: &str,
        _pos: usize,
        kind: rustyline::highlight::CmdKind,
    ) -> bool {
        use rustyline::highlight::CmdKind;
        kind != CmdKind::MoveCursor && kind != CmdKind::ForcedRefresh
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        format!("{}", hint.truecolor(91, 96, 120)).into()
    }
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        format!("{}", prompt.truecolor(166, 218, 149)).into()
    }
}

fn main() {
    let args = Args::parse();

    let context = Context::new(args);

    let editor_config = Config::builder()
        .auto_add_history(true)
        .completion_type(rustyline::CompletionType::List)
        .bell_style(rustyline::config::BellStyle::None)
        .build();

    let helper = RustyLineHelper {
        completer: FilenameCompleter::new(),
        validator: MatchingBracketValidator::new(),
        hinter: HistoryHinter::new(),
        highlighter: SevaHighlighter,
        prompt: "".into(),
    };

    let mut editor = Editor::with_config(editor_config).unwrap();
    editor.set_helper(Some(helper));
    editor.bind_sequence(rustyline::KeyEvent::ctrl('f'), rustyline::Cmd::CompleteHint);

    loop {
        let prompt = "> ".to_string();
        let input = editor.readline(&prompt);
        editor.helper_mut().expect("No helper").prompt = prompt;

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
            .unwrap();
    }
}
