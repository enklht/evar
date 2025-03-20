use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
use clap::Parser as ClapParser;
use seva::{ErrorReporter, args::Args, create_context, lex, parser, readline::SevaEditor};

fn main() {
    let args = Args::parse();
    let debug = args.debug;

    let mut context = create_context(&args);
    let mut editor = SevaEditor::new(&args);
    let mut reporter = ErrorReporter::new(args.no_color);

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
                match stmt.eval(&mut context) {
                    Ok(out) => println!("{}", out),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(errs) => reporter.report_error(errs, &input),
        }
    }
}
