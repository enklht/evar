use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum AngleUnit {
    Radian,
    Degree,
}

#[derive(Parser, Debug)]
#[command(version, about)]
/// Command line arguments for the application
pub struct Args {
    /// Number of decimal places in output
    #[arg(short, long, default_value_t = 10)]
    pub fix: u8,
    /// Radix of calculation output
    #[arg(short, long, default_value_t = 10)]
    pub base: u8,
    /// Print parsed expression for debug purpose
    #[arg(short, long)]
    pub debug: bool,
    /// Angle Unit
    #[arg(value_enum, short, long, default_value_t = AngleUnit::Radian)]
    pub angle_unit: AngleUnit,
}
