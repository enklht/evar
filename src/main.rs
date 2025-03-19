use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use logos::Logos;
use seva::{
    args::Args, context::Context, errors::SevaError, eval::eval, lexer::Token, parser::parser,
    readline::SevaEditor, report_error,
};

fn main() -> Result<(), SevaError> {
    let args = Args::parse();
    let debug = args.debug;

    let context = Context::new(&args);

    let mut editor = SevaEditor::new(&args);

    let writer = StandardStream::stderr(if args.no_color {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    });
    let config = codespan_reporting::term::Config::default();

    loop {
        let input = editor.readline()?;

        if input == "exit" {
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
            Err(errs) => report_error(errs, &input, &writer, &config),
        }
    }

    Ok(())
}
