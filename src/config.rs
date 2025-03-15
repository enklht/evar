use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
enum AngleUnit {
    Radian,
    Degree,
    Gradian,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
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
