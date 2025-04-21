use bpaf::Bpaf;

#[derive(Debug, Clone)]
pub enum AngleUnit {
    Radian,
    Degrees,
}

#[derive(Bpaf, Debug)]
#[bpaf(options, version)]
/// Modern ergonomic math calculator inspired by eva
pub struct Args {
    /// Use degrees instead of radians
    #[bpaf(
        short('d'),
        long("degrees"),
        flag(AngleUnit::Degrees, AngleUnit::Radian)
    )]
    pub angle_unit: AngleUnit,

    /// Number of decimal places in output (0-63) [default: None]
    #[bpaf(short, long, guard(fix_in_range, "fix must be in range 0-63"))]
    pub fix: Option<usize>,

    /// Disable colored output
    #[bpaf(long, fallback(false))]
    pub no_color: bool,

    /// Print parsed expression for debug purpose
    #[bpaf(long)]
    pub debug: bool,
}

fn fix_in_range(fix: &Option<usize>) -> bool {
    match fix {
        None => true,
        Some(fix) => (0..64).contains(fix),
    }
}
