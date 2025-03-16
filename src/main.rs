use clap::Parser;
use dialoguer::{BasicHistory, Input, theme::ColorfulTheme};
use seva::{context::Context, eval::eval, parser::parse};

fn main() {
    let context = Context::parse();

    let mut history = BasicHistory::new().max_entries(100).no_duplicates(true);

    loop {
        if let Ok(input) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("seva")
            .history_with(&mut history)
            .interact_text()
        {
            if input == "exit" {
                break;
            }

            let parse_result = parse(&input);

            let Ok(expr) = parse_result else {
                println!("{}", parse_result.err().unwrap());
                continue;
            };

            let value = eval(expr, &context);
            println!("{}", value);
        }
    }
}
