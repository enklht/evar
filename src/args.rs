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
    /// Angle Unit
    #[arg(value_enum, short, long, default_value_t = AngleUnit::Radian)]
    pub angle_unit: AngleUnit,
    /// Enable colored output
    #[arg(long, default_value_t = false)]
    pub no_color: bool,
    /// Number of decimal places in output (0-63) [default: None]
    #[arg(short, long, value_parser = fix_in_range)]
    pub fix: Option<usize>,
    // /// Radix of calculation output
    // #[arg(short, long, default_value_t = 10)]
    // pub base: u8,
    /// Print parsed expression for debug purpose
    #[arg(short, long)]
    pub debug: bool,
}

fn fix_in_range(s: &str) -> Result<Option<usize>, String> {
    if s.is_empty() {
        return Ok(None);
    }
    let fix = s
        .parse::<usize>()
        .map_err(|_| format!("{} is not a usize", s))?;
    if (0..64).contains(&fix) {
        Ok(Some(fix))
    } else {
        Err(String::from("fix must be in range 0-63"))
    }
}
