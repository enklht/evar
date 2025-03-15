use clap::Parser;
use dialoguer::{BasicHistory, Input, theme::ColorfulTheme};
use seva::config::Config;
use seva::parser::parse;

fn main() {
    let config = Config::parse();

    let mut history = BasicHistory::new().max_entries(100).no_duplicates(true);

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("seva")
            .history_with(&mut history)
            .interact_text()
        {
            if cmd == "exit" {
                break;
            }

            match parse(&cmd) {
                Ok(res) => println!("{res:#?}"),
                Err(e) => println!("{e}"),
            }
        }
    }
}
