use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use seva::{args::Args, create_context, lex, parser, readline::SevaEditor, report_error};

fn main() {
    let args = Args::parse();
    let debug = args.debug;

    let mut context = create_context(&args);

    let mut editor = SevaEditor::new(&args);

    let writer = StandardStream::stderr(if args.no_color {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    });
    let config = codespan_reporting::term::Config::default();

    loop {
        let input = editor.readline();

        if input == "exit" {
            break;
        };

        let token_iter = lex(&input);

        let token_stream = Stream::from_iter(token_iter)
            .map((input.len()..input.len()).into(), |(token, span)| {
                (token, span.into())
            });

        match parser().parse(token_stream).into_result() {
            Ok(stmt) => {
                if debug {
                    println!("{}", stmt)
                };
                match stmt.eval(&mut context) {
                    Ok(out) => println!("{}", out),
                    Err(err) => println!("{}", err),
                }
            }
            Err(errs) => report_error(errs, &input, &writer, &config),
        }
    }
}
