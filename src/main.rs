use clap::{Parser, ValueEnum};
use dialoguer::{BasicHistory, Input, theme::ColorfulTheme};

#[derive(ValueEnum, Debug, Clone)]
enum AngleUnit {
    Radian,
    Degree,
    Gradian,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Config {
    /// Number of decimal places in output
    #[arg(short, long, default_value_t = 10)]
    fix: u8,
    /// Radix of calculation output
    #[arg(short, long, default_value_t = 10)]
    base: u8,
    /// Angle Unit
    #[arg(value_enum, short, long, default_value_t = AngleUnit::Radian)]
    angle_unit: AngleUnit,
}

fn main() {
    let config = Config::parse();

    let mut history = BasicHistory::new().max_entries(100).no_duplicates(true);

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("seva")
            .history_with(&mut history)
            .interact()
        {
            if cmd == "exit" {
                break;
            }
            println!("{cmd}");
        }
    }
}
