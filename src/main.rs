use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use seva::{ErrorReporter, args::Args, create_context, lex, parser, readline::SevaEditor};

fn main() {
    let Args {
        debug,
        no_color,
        angle_unit,
    } = Args::parse();

    let (mut fcontext, vcontext) = create_context(&angle_unit);
    let mut editor = SevaEditor::new(no_color);
    let mut reporter = ErrorReporter::new(no_color);

    loop {
        let input = editor.readline();

        if input == "exit" || input == "quit" {
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
                match stmt.eval(&mut fcontext, vcontext.clone()) {
                    Ok(out) => println!("{:?}", out),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(errs) => reporter.report_error(errs, &input),
        }
    }
}
