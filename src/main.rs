mod args;
mod default_context;
mod error_report;
mod models;
mod parser;
mod readline;

use args::{Args, args};
use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use default_context::create_context;
use directories::ProjectDirs;
use error_report::ErrorReporter;
use models::{Stmt, Token};
use parser::parser;
use readline::SevaEditor;
use rustyline::error::ReadlineError;

fn lex_and_parse(input: &str) -> Result<Stmt, Vec<Rich<'_, Token<'_>>>> {
    let token_iter = models::token::lex(input).filter(|token| !matches!(token, (Token::Space, _)));

    let token_stream = Stream::from_iter(token_iter)
        .map((input.len()..input.len()).into(), |(token, span)| {
            (token, span.into())
        });

    parser().parse(token_stream).into_result()
}

fn main() {
    let Args {
        fix,
        debug,
        no_color,
        angle_unit,
    } = args().run();

    let mut context = create_context(&angle_unit);
    let mut editor = SevaEditor::new(no_color);
    let mut reporter = ErrorReporter::new(no_color);

    let seva_dirs =
        ProjectDirs::from("", "enklht", "seva").expect("no valid home directory path retrieved");
    let mut history_path = std::path::PathBuf::from(seva_dirs.data_local_dir());

    match std::fs::create_dir_all(&history_path) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to create data directory: {}", e),
    };

    history_path.push("history.txt");

    match editor.load_history(history_path.as_path()) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to load historoy: {}", e),
    }

    loop {
        match editor.readline() {
            Ok(input) => {
                if input == "exit" || input == "quit" {
                    break;
                };

                if input == "help" {
                    context.print_help();
                    continue;
                }

                match lex_and_parse(&input) {
                    Ok(stmt) => {
                        if debug {
                            println!("{}", stmt)
                        };
                        match stmt.eval(&mut context) {
                            Ok(out) => out.print(fix),
                            Err(err) => eprintln!("{}", err),
                        }
                    }
                    Err(errs) => reporter.report_error(errs, &input),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                println!("Readline Error: {}", e);
                break;
            }
        }
    }

    match editor.save_history(history_path.as_path()) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to save history: {}", e),
    }
}
