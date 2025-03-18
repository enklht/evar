use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::Parser as ClapParser;
use dialoguer::{BasicHistory, theme::ColorfulTheme};
use seva::{context::Context, eval::eval, parser::parse};

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

            match parse(&input) {
                Ok(out) => println!("{:?}", eval(out, &context)),
                Err(err) => println!("{}", err),
            };

            //     match parser().parse(token_stream).into_result() {
            //         Ok(expr) => match eval(expr, &context) {
            //             Ok(out) => println!("{}", out),
            //             Err(err) => println!("{}", err),
            //         },
            //         Err(errs) => {
            //             for err in errs {
            //                 Report::build(ReportKind::Error, ("", err.span().into_range()))
            //                     .with_message(err.to_string())
            //                     .with_label(
            //                         Label::new(("", err.span().into_range()))
            //                             .with_message(err.to_string())
            //                             .with_color(Color::Red),
            //                     )
            //                     .finish()
            //                     .print(("", Source::from(&input)))
            //                     .unwrap();
            //             }
            //         }
            //     }
        }
    }
}
